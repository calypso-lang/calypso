use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    sync::Arc,
};

use calypso::{ctxt::GlobalCtxt, error::CalResult, symbol::Symbol};
use calypso_repl::Repl;

use crate::cli::UnprettyFormat;

pub mod ast;
pub mod toks;

pub fn unpretty(
    gcx: &Arc<GlobalCtxt>,
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
                .write()
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
                .write()
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
                        .write()
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
        UnprettyFormat::Ast => println!("todo: ast::run_parser"), // ast::run_parser(gcx, file_name, contents),
        UnprettyFormat::TokenList => toks::run_lexer(gcx, Symbol::intern(&file_name), &contents)?,
    }

    Ok(())
}

pub fn run_repl(gcx: &Arc<GlobalCtxt>, format: UnprettyFormat) {
    struct ReplCtx {
        line: usize,
    }

    let repl_gcx = Arc::clone(gcx);
    let mut repl = Repl::new(
        Box::new(move |rcx: &mut ReplCtx, contents| {
            let res = match format {
                UnprettyFormat::Ast => {
                    println!("todo: ast::run_parser");

                    //ast::run_parser(&repl_gcx, format!("<repl:{}>", rcx.line), contents)
                    Ok(())
                }
                UnprettyFormat::TokenList => toks::run_lexer(
                    &repl_gcx,
                    Symbol::intern(&format!("<repl:{}>", rcx.line)),
                    &contents,
                ),
            }
            .ok()
            .map(|_| String::new());
            rcx.line += 1;
            repl_gcx.diag.write().clear();
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
