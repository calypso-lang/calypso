use std::cell::RefCell;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

use clap::ArgMatches;

use crate::messages::error;

use calypso_diagnostic::prelude::*;
use calypso_diagnostic::report::GlobalReportingCtxt;
use calypso_parsing::lexer::{Lexer, TokenType};
use calypso_parsing::pretty::Printer;
use calypso_repl::Repl;

#[allow(clippy::single_match)]
pub fn internal(matches: &ArgMatches) {
    match matches.subcommand() {
        ("lexer", Some(matches)) => lexer(matches),
        _ => {}
    }
}

pub fn lexer(matches: &ArgMatches) {
    let ignore_ws = matches.is_present("ignore_ws");
    let path = matches.value_of("INPUT").unwrap();

    if path == "-" {
        lexer_stdin(matches);
        return;
    }

    let path = Path::new(path);
    if !path.exists() {
        error(format!("file does not exist: `{}`", path.display()));
        return;
    }

    let contents = match fs::read_to_string(&path) {
        Ok(v) => v,
        Err(err) => {
            error(format!("while reading file `{}`:", path.display()));
            error(err);
            return;
        }
    };

    let mut files = FileMgr::new();
    let source_id = files.add(path.display().to_string(), contents);
    let grcx = Rc::new(RefCell::new(GlobalReportingCtxt::new()));
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
            error("while lexing input:");
            error(err);
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
            error("while pretty-printing tokens:");
            error(err);
        }
    }
}

pub fn lexer_stdin(matches: &ArgMatches) {
    let ignore_ws = matches.is_present("ignore_ws");
    if matches.is_present("repl") {
        lexer_stdin_repl(ignore_ws);
        return;
    }

    let stdin = io::stdin();
    let mut contents = String::new();
    if let Err(err) = stdin.lock().read_to_string(&mut contents) {
        error("while reading from stdin:");
        error(err);
        return;
    }

    let mut files = FileMgr::new();
    let source_id = files.add("<anon>".to_string(), contents);
    let grcx = Rc::new(RefCell::new(GlobalReportingCtxt::new()));
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
            error("while lexing input:");
            error(err);
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
            error("while pretty-printing tokens:");
            error(err);
        }
    }
}

pub fn lexer_stdin_repl(ignore_ws: bool) {
    struct ReplCtx {}

    let mut repl = Repl::new(
        Box::new(move |_ctx, contents| {
            let mut files = FileMgr::new();
            let source_id = files.add("<anon>".to_string(), contents);
            let grcx = Rc::new(RefCell::new(GlobalReportingCtxt::new()));
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
                    error("while lexing input:");
                    error(err);
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
                    error("while pretty-printing tokens:");
                    error(err);
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
            env!("CARGO_PKG_VERSION")
        ),
        |_| String::from(">>> "),
    )
    .expect("REPL failure");
}
