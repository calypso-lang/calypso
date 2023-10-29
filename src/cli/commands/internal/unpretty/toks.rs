use std::sync::Arc;

use calypso::{
    ctxt::GlobalCtxt,
    error::CalResult,
    parse::{lexer, pretty::Printer},
    symbol::Symbol,
};

pub fn run_lexer(gcx: &Arc<GlobalCtxt>, name: Symbol, contents: &str) -> CalResult<()> {
    gcx.source_cache.write().add(name, contents);

    let mut printer = Printer::new(Arc::clone(gcx));
    let tokens = lexer::tokens(contents, name, Arc::clone(gcx)).collect::<Vec<_>>();

    let diag_read = gcx.diag.read();
    if let Some(fatal) = diag_read.fatal() {
        let mut emit = gcx.emit.write();
        let mut buf = emit.err.buffer();
        let mut cache = gcx.source_cache.write();
        fatal.write(&mut *cache, &mut buf)?;
        emit.err.emit(&buf)?.flush()?;
    } else {
        diag_read
            .errors()
            .iter()
            .try_for_each(|e| -> CalResult<()> {
                let mut emit = gcx.emit.write();
                let mut buf = emit.err.buffer();
                let mut cache = gcx.source_cache.write();
                e.write(&mut *cache, &mut buf)?;
                emit.err.emit(&buf)?;
                Ok(())
            })?;
        gcx.emit.write().err.flush()?;
    }
    drop(diag_read);

    let tokens = tokens
        .iter()
        .map(|v| printer.print_token(name, v))
        .collect::<Result<Vec<String>, _>>();
    match tokens {
        Ok(tokens) => println!("{}", tokens.join("\n")),
        Err(err) => {
            gcx.emit
                .write()
                .err
                .error(None, "while pretty-printing tokens:", None)?
                .error(None, &format!("{err}"), None)?;
        }
    }

    Ok(())
}
