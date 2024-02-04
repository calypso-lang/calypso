use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
};

use calypso::{ctxt::GlobalCtxt, error::CalResult, symbol::Symbol};
use calypso_repl::Repl;

use crate::cli::UnprettyFormat;

pub mod ast;
pub mod toks;

pub fn unpretty(
    gcx: &GlobalCtxt,
    format: UnprettyFormat,
    path: &Path,
    repl: bool,
) -> CalResult<()> {
    let (file_name, contents) = if path == Path::new("-") {
        if repl {
            run_repl(gcx, format);
            return Ok(());
        }

        let stdin = io::stdin();
        let mut contents = String::new();
        if let Err(err) = stdin.lock().read_to_string(&mut contents) {
            gcx.emit
                .borrow_mut()
                .err
                .error(None, "while reading from stdin:", None)?
                .error(None, &format!("{err}"), None)?
                .flush()?;
            return Ok(());
        }

        ("<stdin>".to_string(), contents)
    } else {
        if !path.exists() {
            gcx.emit
                .borrow_mut()
                .err
                .error(
                    None,
                    "file does not exist",
                    Some(&format!("`{}`", path.display())),
                )?
                .flush()?;
            return Ok(());
        }

        (
            path.display().to_string(),
            match fs::read_to_string(path) {
                Ok(v) => v,
                Err(err) => {
                    gcx.emit
                        .borrow_mut()
                        .err
                        .error(
                            None,
                            "while reading file",
                            Some(&format!("`{}`:", path.display())),
                        )?
                        .error(None, &format!("{err}"), None)?
                        .flush()?;
                    return Ok(());
                }
            },
        )
    };

    match format {
        UnprettyFormat::Ast => ast::run_parser(gcx, Symbol::intern(&file_name), &contents)?,
        UnprettyFormat::TokenList => toks::run_lexer(gcx, Symbol::intern(&file_name), &contents)?,
    }

    Ok(())
}

pub fn run_repl(gcx: &GlobalCtxt, format: UnprettyFormat) {
    struct ReplCtx {
        line: usize,
    }

    let mut repl = Repl::new(
        Box::new(move |rcx: &mut ReplCtx, contents| {
            let res = match format {
                UnprettyFormat::Ast => ast::run_parser(
                    gcx,
                    Symbol::intern(&format!("<repl:{}>", rcx.line)),
                    &contents,
                ),
                UnprettyFormat::TokenList => toks::run_lexer(
                    gcx,
                    Symbol::intern(&format!("<repl:{}>", rcx.line)),
                    &contents,
                ),
            }
            .ok()
            .map(|()| String::new());
            rcx.line += 1;
            gcx.diag.borrow_mut().clear();
            gcx.arenas.ast.clear();
            res
        }),
        ReplCtx { line: 1 },
    )
    .prefix("\\".to_string());
    repl.run(
        &format!(
            "Calypso CLI v{} - unpretty: {}",
            env!("CARGO_PKG_VERSION"),
            format
        ),
        |rcx| format!("[{}]: ", rcx.line),
    )
    .expect("REPL failure");
}
