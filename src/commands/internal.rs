use std::cell::RefCell;
use std::fs;
use std::io::{self, prelude::*};
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;

use clap::ArgMatches;

use calypso_base::session::BaseSession;
use calypso_base::ui::{
    self,
    termcolor::{Color, ColorSpec, WriteColor},
};
use calypso_diagnostic::prelude::*;
use calypso_diagnostic::report::GlobalReportingCtxt;
use calypso_parsing::lexer::{Lexer, TokenType};
use calypso_parsing::pretty::Printer;
use calypso_repl::Repl;

use crate::buildinfo::BUILD_INFO;

#[allow(clippy::single_match)]
pub fn internal(sess: Arc<BaseSession>, matches: &ArgMatches) {
    match matches.subcommand() {
        ("lexer", Some(matches)) => lexer(sess, matches),
        ("buildinfo", _) => buildinfo(sess),
        ("panic", _) => panic!("Intentional panic to test ICE handling, please ignore."),
        _ => {}
    }
}

pub fn lexer(sess: Arc<BaseSession>, matches: &ArgMatches) {
    let ignore_ws = matches.is_present("ignore_ws");
    let path = matches.value_of("INPUT").unwrap();

    if path == "-" {
        lexer_stdin(sess, matches);
        return;
    }

    let path = Path::new(path);
    if !path.exists() {
        ui::error_to(
            &sess.stderr,
            None,
            "file does not exist",
            Some(&format!("`{}`", path.display())),
        )
        .unwrap();
        return;
    }

    let contents = match fs::read_to_string(&path) {
        Ok(v) => v,
        Err(err) => {
            ui::error_to(
                &sess.stderr,
                None,
                "while reading file",
                Some(&format!("`{}`:", path.display())),
            )
            .unwrap();
            ui::error_to(&sess.stderr, None, &format!("{}", err), None).unwrap();
            return;
        }
    };

    let mut files = FileMgr::new();
    let source_id = files.add(path.display().to_string(), contents);
    let grcx = Rc::new(RefCell::new(GlobalReportingCtxt::new(Arc::clone(&sess))));
    let mut lexer = Lexer::new(
        source_id,
        files.get(source_id).unwrap().source(),
        &files,
        Rc::clone(&grcx),
    );
    let mut tokens = Vec::new();
    let mut printer = Printer::new(source_id, &files);
    loop {
        let token = lexer.scan();
        if let Err(err) = token {
            ui::error_to(&sess.stderr, None, "while lexing input:", None).unwrap();
            if let Some(DiagnosticError::Diagnostic) = err.try_downcast_ref::<DiagnosticError>() {
                sess.stderr
                    .print(&grcx.borrow().fatal().unwrap().rendered())
                    .unwrap();
            } else {
                ui::error_to(&sess.stderr, None, &format!("{}", err), None).unwrap();
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
    grcx.borrow()
        .errors()
        .iter()
        .for_each(|e| println!("{}", e));
    let tokens = tokens
        .iter()
        .map(|v| printer.print_token(v))
        .collect::<Result<Vec<String>, _>>();
    match tokens {
        Ok(tokens) => println!("{}", tokens.join("\n")),
        Err(err) => {
            ui::error_to(&sess.stderr, None, "while pretty-printing tokens:", None).unwrap();
            ui::error_to(&sess.stderr, None, &format!("{}", err), None).unwrap();
        }
    }
}

pub fn lexer_stdin(sess: Arc<BaseSession>, matches: &ArgMatches) {
    let ignore_ws = matches.is_present("ignore_ws");
    if matches.is_present("repl") {
        lexer_stdin_repl(sess, ignore_ws);
        return;
    }

    let stdin = io::stdin();
    let mut contents = String::new();
    if let Err(err) = stdin.lock().read_to_string(&mut contents) {
        ui::error_to(&sess.stderr, None, "while reading from stdin:", None).unwrap();
        ui::error_to(&sess.stderr, None, &format!("{}", err), None).unwrap();
        return;
    }

    let mut files = FileMgr::new();
    let source_id = files.add("<anon>".to_string(), contents);
    let grcx = Rc::new(RefCell::new(GlobalReportingCtxt::new(Arc::clone(&sess))));
    let mut lexer = Lexer::new(
        source_id,
        files.get(source_id).unwrap().source(),
        &files,
        Rc::clone(&grcx),
    );
    let mut tokens = Vec::new();
    let mut printer = Printer::new(source_id, &files);
    loop {
        let token = lexer.scan();
        if let Err(err) = token {
            ui::error_to(&sess.stderr, None, "while lexing input:", None).unwrap();
            if let Some(DiagnosticError::Diagnostic) = err.try_downcast_ref::<DiagnosticError>() {
                sess.stderr
                    .print(&grcx.borrow().fatal().unwrap().rendered())
                    .unwrap();
            } else {
                ui::error_to(&sess.stderr, None, &format!("{}", err), None).unwrap();
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
    grcx.borrow()
        .errors()
        .iter()
        .for_each(|e| println!("{}", e));
    let tokens = tokens
        .iter()
        .map(|v| printer.print_token(v))
        .collect::<Result<Vec<String>, _>>();
    match tokens {
        Ok(tokens) => println!("{}", tokens.join("\n")),
        Err(err) => {
            ui::error_to(&sess.stderr, None, "while pretty-printing tokens:", None).unwrap();
            ui::error_to(&sess.stderr, None, &format!("{}", err), None).unwrap();
        }
    }
}

pub fn lexer_stdin_repl(sess: Arc<BaseSession>, ignore_ws: bool) {
    struct ReplCtx {}

    let mut repl = Repl::new(
        Box::new(move |_ctx, contents| {
            let mut files = FileMgr::new();
            let source_id = files.add("<anon>".to_string(), contents);
            let grcx = Rc::new(RefCell::new(GlobalReportingCtxt::new(Arc::clone(&sess))));
            let mut lexer = Lexer::new(
                source_id,
                files.get(source_id).unwrap().source(),
                &files,
                Rc::clone(&grcx),
            );
            let mut tokens = Vec::new();
            let mut printer = Printer::new(source_id, &files);
            loop {
                let token = lexer.scan();
                if let Err(err) = token {
                    ui::error_to(&sess.stderr, None, "while lexing input:", None).unwrap();
                    if let Some(DiagnosticError::Diagnostic) =
                        err.try_downcast_ref::<DiagnosticError>()
                    {
                        sess.stderr
                            .print(&grcx.borrow().fatal().unwrap().rendered())
                            .unwrap();
                    } else {
                        ui::error_to(&sess.stderr, None, &format!("{}", err), None).unwrap();
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
            grcx.borrow()
                .errors()
                .iter()
                .for_each(|e| println!("{}", e));
            let tokens = tokens
                .iter()
                .map(|v| printer.print_token(v))
                .collect::<Result<Vec<String>, _>>();
            match tokens {
                Ok(tokens) => Some(tokens.join("\n")),
                Err(err) => {
                    ui::error_to(&sess.stderr, None, "while pretty-printing tokens:", None)
                        .unwrap();
                    ui::error_to(&sess.stderr, None, &format!("{}", err), None).unwrap();
                    None
                }
            }
        }),
        ReplCtx {},
    )
    .prefix("$".to_string());
    repl.run(
        &format!(
            "Calypso CLI v{} - internal debugging command: lexer",
            BUILD_INFO.version
        ),
        |_| String::from(">>> "),
    )
    .expect("REPL failure");
}

pub fn buildinfo(sess: Arc<BaseSession>) {
    let bi = BUILD_INFO;

    ui::info_to(&sess.stdout, "=:= Version =:=", None).unwrap();
    println!();
    ui::info_to(&sess.stdout, "version", Some(bi.version)).unwrap();
    ui::info_to(&sess.stdout, "git branch", Some(bi.git_branch)).unwrap();
    ui::info_to(&sess.stdout, "git commit", Some(bi.git_commit)).unwrap();
    println!();
    ui::info_to(&sess.stdout, "=:= Build Env =:=", None).unwrap();
    println!();
    ui::info_to(&sess.stdout, "features:", None).unwrap();
    for feature in bi.cargo_features.split(",") {
        let mut buf = sess.stdout.buffer();
        buf.set_color(
            ColorSpec::new()
                .set_fg(Some(Color::Green))
                .set_bold(true)
                .set_intense(true),
        )
        .unwrap();
        write!(buf, "  =>").unwrap();
        buf.reset().unwrap();
        write!(buf, " {}", feature).unwrap();
        writeln!(buf).unwrap();
        sess.stdout.print(&buf).unwrap();
    }
    ui::info_to(&sess.stdout, "profile", Some(bi.cargo_profile)).unwrap();
    ui::info_to(&sess.stdout, "target triple", Some(bi.cargo_target_triple)).unwrap();
    println!();
    ui::info_to(&sess.stdout, "=:= Rust =:=", None).unwrap();
    println!();
    ui::info_to(&sess.stdout, "channel", Some(bi.rustc_channel)).unwrap();
    ui::info_to(&sess.stdout, "commit date", Some(bi.rustc_commit_date)).unwrap();
    ui::info_to(&sess.stdout, "commit hash", Some(bi.rustc_commit_hash)).unwrap();
    ui::info_to(&sess.stdout, "host triple", Some(bi.rustc_host_triple)).unwrap();
    ui::info_to(&sess.stdout, "llvm version", Some(bi.rustc_llvm_version)).unwrap();
    ui::info_to(&sess.stdout, "version", Some(bi.rustc_version)).unwrap();
}
