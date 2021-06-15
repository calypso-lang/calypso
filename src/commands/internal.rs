use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;
use std::sync::Arc;

use calypso_repl::Repl;
use clap::ArgMatches;

use calypso_base::ui::termcolor::{Color, ColorSpec, WriteColor};
use calypso_common::gcx::GlobalCtxt;
use calypso_diagnostic::prelude::*;
use calypso_diagnostic::reporting::files::Files;
use calypso_parsing::lexer::{Lexer, TokenType};
use calypso_parsing::pretty::Printer;
// use calypso_repl::Repl;

use crate::buildinfo::BUILD_INFO;

#[allow(clippy::single_match)]
pub fn internal(gcx: &Arc<GlobalCtxt>, matches: &ArgMatches) -> CalResult<()> {
    match matches.subcommand() {
        ("lexer", Some(matches)) => lexer(gcx, matches),
        ("buildinfo", _) => buildinfo(gcx),
        ("panic", _) => panic!("Intentional panic to test ICE handling, please ignore."),
        _ => Ok(()),
    }
}

pub fn lexer(gcx: &Arc<GlobalCtxt>, matches: &ArgMatches) -> CalResult<()> {
    let ignore_ws = matches.is_present("ignore_ws");
    let path = matches.value_of("INPUT").unwrap();

    let (file_name, contents) = if path == "-" {
        if matches.is_present("repl") {
            return lexer_repl(gcx, ignore_ws);
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
        let path = Path::new(path);
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

    run_lexer(gcx, ignore_ws, file_name, contents)
}

pub fn run_lexer(
    gcx: &Arc<GlobalCtxt>,
    ignore_ws: bool,
    file_name: String,
    contents: String,
) -> CalResult<()> {
    let file_id = gcx.sourcemgr.write().add(file_name, contents);

    let sourcemgr = gcx.sourcemgr.read();
    let source = sourcemgr.source(file_id).unwrap();

    let mut lexer = Lexer::new(file_id, source, Arc::clone(gcx));
    let mut tokens = Vec::new();
    let mut printer = Printer::new(file_id, Arc::clone(gcx));
    loop {
        let token = lexer.scan();
        if let Err(err) = token {
            let mut emit = gcx.emit.write();

            emit.err.error(None, "while lexing input:", None)?;
            if let Some(DiagnosticError::Diagnostic) = err.try_downcast_ref::<DiagnosticError>() {
                let mut buf = emit.err.buffer();
                gcx.grcx
                    .read()
                    .fatal()
                    .unwrap()
                    .render(&mut buf, &sourcemgr, None)?;
                emit.err.emit(&buf)?;
            } else {
                emit.err.error(None, &format!("{}", err), None)?;
            }
            break;
        } else if let Ok(token) = token {
            let token_ty = token.value().0;
            if !ignore_ws || token_ty != TokenType::Ws {
                tokens.push(token);
            }
            if token_ty == TokenType::Eof {
                break;
            }
        }
    }

    gcx.grcx
        .read()
        .errors()
        .iter()
        .try_for_each(|e| -> CalResult<()> {
            let mut emit = gcx.emit.write();
            let mut buf = emit.err.buffer();
            e.render(&mut buf, &sourcemgr, None)?;
            emit.err.emit(&buf)?;
            Ok(())
        })?;

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

pub fn lexer_repl(gcx: &Arc<GlobalCtxt>, ignore_ws: bool) -> CalResult<()> {
    struct ReplCtx {}

    let repl_gcx = Arc::clone(gcx);
    let mut repl = Repl::new(
        Box::new(move |_ctx, contents| {
            run_lexer(&repl_gcx, ignore_ws, "<repl>".to_string(), contents)
                .ok()
                .map(|_| String::new())
        }),
        ReplCtx {},
    )
    .prefix("\\".to_string());
    repl.run(
        &format!(
            "Calypso CLI v{} - internal debugging command: lexer",
            BUILD_INFO.version
        ),
        |_| String::from(">>> "),
    )
    .expect("REPL failure");
    Ok(())
}

pub fn buildinfo(gcx: &Arc<GlobalCtxt>) -> CalResult<()> {
    let mut bi = BUILD_INFO;

    let mut emit = gcx.emit.write();
    let out = &mut emit.out;

    out.info("=:= Version =:=", None)?
        .newline()?
        .info("version", Some(bi.version))?
        .info("git branch", Some(bi.git_branch))?
        .info("git commit", Some(bi.git_commit))?
        .newline()?
        .info("=:= Build Env =:=", None)?
        .newline()?
        .info("features:", None)?;

    if bi.cargo_features.is_empty() {
        bi.cargo_features = "no cargo features enabled";
    }

    for feature in bi.cargo_features.split(',') {
        out.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Green))
                .set_bold(true)
                .set_intense(true),
        )?;
        out.print("  =>")?;
        out.reset()?;
        out.print(&format!(" {}", feature))?.newline()?;
    }

    out.info("profile", Some(bi.cargo_profile))?
        .info("target triple", Some(bi.cargo_target_triple))?
        .newline()?
        .info("=:= Rust =:=", None)?
        .newline()?
        .info("channel", Some(bi.rustc_channel))?
        .info("commit date", Some(bi.rustc_commit_date))?
        .info("commit hash", Some(bi.rustc_commit_hash))?
        .info("host triple", Some(bi.rustc_host_triple))?
        .info("llvm version", Some(bi.rustc_llvm_version))?
        .info("version", Some(bi.rustc_version))?
        .flush()?;

    Ok(())
}
