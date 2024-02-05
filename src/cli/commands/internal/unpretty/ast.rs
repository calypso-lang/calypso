use std::borrow::Cow;

use ariadne::{Color, Fmt, Label, Report, ReportBuilder, ReportKind};
use calypso::{
    ctxt::GlobalCtxt,
    error::CalResult,
    parse::{
        lexer::{self, Token},
        pretty::Printer,
        Span, SpanWithFile,
    },
    symbol::Symbol,
};
use chumsky::{error::RichReason, input::Stream, prelude::Input, primitive::end, Parser};

#[allow(dead_code)]
pub fn run_parser(mut gcx: &GlobalCtxt, file_name: Symbol, contents: &str) -> CalResult<()> {
    gcx.source_cache.borrow_mut().add(file_name, contents);

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
    let (expr, parse_errs) = calypso::parse::parser::stmt()
        .then_ignore(end())
        .parse_with_state(stream, &mut gcx)
        .into_output_errors();

    for e in parse_errs {
        let mut report = Report::build(ReportKind::Error, file_name, e.span().lo() as usize);

        report = render_diagnostic(e.reason(), *e.span(), file_name, report);

        let mut source_cache = gcx.source_cache.borrow_mut();
        report.finish().eprint(&mut *source_cache).unwrap();
    }

    if let Some(expr) = expr {
        let printer = Printer::new(gcx);
        let mut w = Vec::new();
        printer.print_expr(expr).render(15, &mut w).unwrap();
        println!("{}", String::from_utf8(w).unwrap());
    }

    Ok(())
}

fn render_diagnostic(
    e: &RichReason<'_, Token>,
    span: Span,
    file: Symbol,
    mut report: ReportBuilder<'static, SpanWithFile>,
) -> ReportBuilder<'static, SpanWithFile> {
    match e {
        RichReason::Custom(msg) => report.with_message(msg).with_label(
            Label::new(SpanWithFile(file, span))
                .with_message(format!("{}", msg.fg(Color::Red)))
                .with_color(Color::Red),
        ),
        RichReason::ExpectedFound { expected, found } => report
            .with_message(format!(
                "{}, expected: {}",
                if let Some(found) = found {
                    Cow::from(format!("Unexpected token {}", found.description()))
                } else {
                    Cow::from("Unexpected end of input")
                },
                if expected.is_empty() {
                    Cow::from("end of input")
                } else {
                    Cow::from(
                        expected
                            .iter()
                            .map(|x| match x {
                                chumsky::error::RichPattern::Token(tok) => {
                                    Cow::from(tok.description())
                                }
                                chumsky::error::RichPattern::Label(l) => Cow::from(*l),
                                chumsky::error::RichPattern::EndOfInput => {
                                    Cow::from("end of input")
                                }
                            })
                            .collect::<Vec<_>>()
                            .join(", "),
                    )
                },
            ))
            .with_label(
                Label::new(SpanWithFile(file, span))
                    .with_message("didn't expect this token".fg(Color::Red))
                    .with_color(Color::Red),
            ),
        RichReason::Many(vec) => {
            for reason in vec {
                report = render_diagnostic(reason, span, file, report);
            }
            report
        }
    }
}
