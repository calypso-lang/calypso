use std::sync::Arc;

use calypso_common::gcx::GlobalCtxt;
use calypso_diagnostic::prelude::*;
use calypso_parsing::{lexer, pretty::Printer};

pub fn run_lexer(gcx: &Arc<GlobalCtxt>, file_name: String, contents: String) -> CalResult<()> {
    let file_id = gcx.sourcemgr.write().add(file_name, contents);

    let sourcemgr = gcx.sourcemgr.read();
    let source = sourcemgr.source(file_id).unwrap();
    let mut printer = Printer::new(file_id, Arc::clone(gcx));
    let tokens = lexer::tokens(source, file_id, Arc::clone(gcx)).collect::<Vec<_>>();

    let grcx_read = gcx.grcx.read();
    if let Some(fatal) = grcx_read.fatal() {
        let mut emit = gcx.emit.write();
        let mut buf = emit.err.buffer();
        fatal.render(&mut buf, &sourcemgr, None)?;
        emit.err.emit(&buf)?.flush()?;
    } else {
        grcx_read
            .errors()
            .iter()
            .try_for_each(|e| -> CalResult<()> {
                let mut emit = gcx.emit.write();
                let mut buf = emit.err.buffer();
                e.render(&mut buf, &sourcemgr, None)?;
                emit.err.emit(&buf)?;
                Ok(())
            })?;
        gcx.emit.write().err.flush()?;
    }
    drop(grcx_read);

    let tokens = tokens
        .iter()
        .map(|v| printer.print_token(v))
        .collect::<Result<Vec<String>, _>>();
    match tokens {
        Ok(tokens) => println!("{}", tokens.join("\n")),
        Err(err) => {
            gcx.emit
                .write()
                .err
                .error(None, "while pretty-printing tokens:", None)?
                .error(None, &format!("{}", err), None)?;
        }
    }

    Ok(())
}
