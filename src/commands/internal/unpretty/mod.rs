use std::{
    fs,
    io::{self, prelude::*},
    path::Path,
    sync::Arc,
};

use calypso_common::gcx::GlobalCtxt;
use calypso_diagnostic::prelude::*;
use calypso_repl::Repl;

use crate::{buildinfo::BUILD_INFO, cli::UnprettyFormat};

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
                .error(None, &format!("{}", err), None)?
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
            match fs::read_to_string(&path) {
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
                        .error(None, &format!("{}", err), None)?
                        .flush()?;
                    return Ok(());
                }
            },
        )
    };

    match format {
        UnprettyFormat::Ast => todo!(),
        UnprettyFormat::TokenList => toks::run_lexer(gcx, file_name, contents),
    }
}

pub fn run_repl(gcx: &Arc<GlobalCtxt>, format: UnprettyFormat) {
    struct ReplCtx {
        line: usize,
    }

    let repl_gcx = Arc::clone(gcx);
    let mut repl = Repl::new(
        Box::new(move |rcx: &mut ReplCtx, contents| {
            let res = match format {
                UnprettyFormat::Ast => todo!(),
                UnprettyFormat::TokenList => {
                    toks::run_lexer(&repl_gcx, format!("<repl:{}>", rcx.line), contents)
                }
            }
            .ok()
            .map(|_| String::new());
            rcx.line += 1;
            repl_gcx.grcx.write().clear();
            res
        }),
        ReplCtx { line: 1 },
    )
    .prefix("\\".to_string());
    repl.run(
        &format!(
            "Calypso CLI v{} - unpretty: {}",
            BUILD_INFO.version,
            format.to_string()
        ),
        |rcx| format!("[{}]: ", rcx.line),
    )
    .expect("REPL failure");
}
