use ariadne::{Report, ReportKind};
use calypso::{
    ctxt::GlobalCtxt,
    error::CalResult,
    parse::{
        lexer::{self, Token},
        parser::{self, render_diagnostic},
        pretty::Printer,
        Span,
    },
    resolve,
    symbol::Symbol,
};
use chumsky::{input::Stream, prelude::Input, primitive::end, IterParser, Parser};

#[allow(dead_code)]
pub fn run_parser(mut gcx: &GlobalCtxt, file_name: Symbol, contents: &str) -> CalResult<()> {
    gcx.source_cache.borrow_mut().add(file_name, contents);
    gcx.session.borrow_mut().current_file = Some(file_name);

    let tokens = lexer::tokens(contents, file_name, gcx)
        .filter_map(|x| {
            if matches!(x.value().0, Token::Comment(_)) {
                None
            } else {
                Some((x.value_owned().0, x.span()))
            }
        })
        .peekable()
        .collect::<Vec<_>>();

    let diag_read = gcx.diag.borrow();
    if let Some(fatal) = diag_read.fatal() {
        let mut emit = gcx.emit.borrow_mut();
        let mut buf = emit.err.buffer();
        let mut cache = gcx.source_cache.borrow_mut();
        fatal.write(&mut *cache, &mut buf)?;
        emit.err.emit(&buf)?.flush()?;
    } else {
        diag_read
            .errors()
            .iter()
            .try_for_each(|e| -> CalResult<()> {
                let mut emit = gcx.emit.borrow_mut();
                let mut buf = emit.err.buffer();
                let mut cache = gcx.source_cache.borrow_mut();
                e.write(&mut *cache, &mut buf)?;
                emit.err.emit(&buf)?;
                Ok(())
            })?;
        gcx.emit.borrow_mut().err.flush()?;
    }
    drop(diag_read);

    let srclen = contents.len().try_into().unwrap();
    let stream = Stream::from_iter(tokens).boxed();
    let stream = stream.spanned(Span::new(srclen, srclen));
    let (items, parse_errs) = parser::item(file_name)
        .separated_by(parser::maybe_nls())
        .collect::<Vec<_>>()
        .then_ignore(end())
        .parse_with_state(stream, &mut gcx)
        .into_output_errors();

    for e in parse_errs {
        let mut report = Report::build(ReportKind::Error, file_name, e.span().lo() as usize);

        report = render_diagnostic(e.reason(), *e.span(), file_name, report);

        let mut source_cache = gcx.source_cache.borrow_mut();
        report.finish().eprint(&mut *source_cache).unwrap();
    }

    if let Some(items) = items {
        let printer = Printer::new(gcx);
        for item in &items {
            let mut w = Vec::new();

            printer.print_item(*item).render(30, &mut w).unwrap();
            println!("{}", String::from_utf8(w).unwrap());
        }
        resolve::resolve_code_unit(gcx, &items)?;
    }
    {
        let mut drcx = gcx.diag.borrow_mut();
        let mut source_cache = gcx.source_cache.borrow_mut();
        for err in drcx.errors() {
            err.eprint(&mut *source_cache)?;
        }

        if let Some(fatal) = drcx.fatal() {
            fatal.eprint(&mut *source_cache)?;
            drcx.clear();
        }
        drcx.clear();
    }

    Ok(())
}
