use calypso::{
    ctxt::GlobalCtxt,
    error::CalResult,
    parse::{lexer, pretty::Printer},
    symbol::Symbol,
};

pub fn run_lexer(gcx: &GlobalCtxt, name: Symbol, contents: &str) -> CalResult<()> {
    gcx.source_cache.borrow_mut().add(name, contents);

    let mut printer = Printer::new(gcx);
    let tokens = lexer::tokens(contents, name, gcx).collect::<Vec<_>>();

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

    let tokens = tokens
        .iter()
        .map(|v| printer.print_token(name, v))
        .collect::<Result<Vec<String>, _>>();
    match tokens {
        Ok(tokens) => println!("{}", tokens.join("\n")),
        Err(err) => {
            gcx.emit
                .borrow_mut()
                .err
                .error(None, "while pretty-printing tokens:", None)?
                .error(None, &format!("{err}"), None)?;
        }
    }

    Ok(())
}
