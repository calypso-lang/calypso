use std::sync::Arc;

use calypso_ast::{pretty::PrettyPrinter, traverse::Visitor};
use calypso_common::gcx::GlobalCtxt;
use calypso_diagnostic::prelude::*;
use calypso_parsing::{
    lexer::{self, Token},
    parser::grammar::ExprsParser,
};

pub fn run_parser(gcx: &Arc<GlobalCtxt>, file_name: String, contents: String) -> CalResult<()> {
    let file_id = gcx.sourcemgr.write().add(file_name, contents);

    let sourcemgr = gcx.sourcemgr.read();
    let source = sourcemgr.source(file_id).unwrap();
    let mut tokens = lexer::tokens(source, file_id, Arc::clone(gcx))
        .filter_map(|x| {
            if matches!(x.value().0, Token::Comment(_)) {
                None
            } else {
                Some((x.span().lo(), x.value_owned().0, x.span().hi()))
            }
        })
        .peekable();

    // let grcx_read = gcx.grcx.read();
    // if let Some(fatal) = grcx_read.fatal() {
    //     let mut emit = gcx.emit.write();
    //     let mut buf = emit.err.buffer();
    //     fatal.render(&mut buf, &sourcemgr, None)?;
    //     emit.err.emit(&buf)?.flush()?;
    // } else {
    //     grcx_read
    //         .errors()
    //         .iter()
    //         .try_for_each(|e| -> CalResult<()> {
    //             let mut emit = gcx.emit.write();
    //             let mut buf = emit.err.buffer();
    //             e.render(&mut buf, &sourcemgr, None)?;
    //             emit.err.emit(&buf)?;
    //             Ok(())
    //         })?;
    //     gcx.emit.write().err.flush()?;
    // }
    // drop(grcx_read);

    let parser = ExprsParser::new();

    loop {
        if tokens.peek().is_none() {
            break;
        }
        match parser.parse(file_id, &mut tokens) {
            Ok(exprs) => {
                for expr in exprs {
                    let mut printer = PrettyPrinter::default();
                    printer.visit_expr(source, expr.as_ref())?;
                    println!("{}", printer);
                }
            }
            Err(err) => {
                let mut emit = gcx.emit.write();
                emit.err
                    .error(None, "Parse error", Some(&format!("{:#?}", err)))?
                    .flush()?;
                break;
            }
        };
    }

    Ok(())
}
