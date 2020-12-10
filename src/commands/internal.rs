// use calypso_parsing::raw::{pest, CalypsoParser, Rule};

use clap::ArgMatches;

// use pest::Parser;

use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::messages::{error, error_chained};

use calypso_diagnostic::prelude::*;
use calypso_parsing::lexer::{Lexer, TokenType};
use calypso_repl::Repl;

#[allow(clippy::single_match)]
pub fn internal(matches: &ArgMatches) {
    match matches.subcommand() {
        ("lexer", Some(matches)) => lexer(matches),
        _ => {}
    }
}

pub fn lexer(matches: &ArgMatches) {
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
            error_chained(err);
            return;
        }
    };

    let mut files = FileMgr::new();
    let source_id = files.add(path.display().to_string(), contents.clone());
    let mut lexer = Lexer::new(source_id, &contents, &files);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.scan();
        if let Err(err) = token {
            error("while lexing input:");
            error_chained(err);
            break;
        } else if let Ok(token) = token {
            if token.value().0 == TokenType::Eof {
                break;
            }
            let value = *token.value();
            tokens.push((value.0, value.1, token.span()));
        }
    }
    println!(
        "{}",
        tokens
            .iter()
            .map(|tok| {
                format!(
                    "{:?} @ {}..{}: `{:?}`",
                    tok.0,
                    tok.2.lo(),
                    tok.2.hi(),
                    tok.1,
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn lexer_stdin(matches: &ArgMatches) {
    if matches.is_present("repl") {
        lexer_stdin_repl();
        return;
    }

    let stdin = io::stdin();
    let mut contents = String::new();
    if let Err(err) = stdin.lock().read_to_string(&mut contents) {
        error("while reading from stdin:");
        error_chained(err);
        return;
    }

    let mut files = FileMgr::new();
    let source_id = files.add("<anon>".to_string(), contents.clone());
    let mut lexer = Lexer::new(source_id, &contents, &files);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.scan();
        if let Err(err) = token {
            error("while lexing input:");
            error_chained(err);
            break;
        } else if let Ok(token) = token {
            if token.value().0 == TokenType::Eof {
                break;
            }
            let value = *token.value();
            tokens.push((value.0, value.1, token.span()));
        }
    }
    println!(
        "{}",
        tokens
            .iter()
            .map(|tok| { format!("{:?} @ {}..{}: {:?}", tok.0, tok.2.lo(), tok.2.hi(), tok.1,) })
            .collect::<Vec<String>>()
            .join("\n")
    );
}

pub fn lexer_stdin_repl() {
    struct ReplCtx {};

    let mut repl = Repl::new(
        Box::new(|_ctx, contents| {
            let mut files = FileMgr::new();
            let source_id = files.add("<anon>".to_string(), contents.clone());
            let mut lexer = Lexer::new(source_id, &contents, &files);
            let mut tokens = Vec::new();
            loop {
                let token = lexer.scan();
                if let Err(err) = token {
                    error("while lexing input:");
                    error_chained(err);
                    break;
                } else if let Ok(token) = token {
                    if token.value().0 == TokenType::Eof {
                        break;
                    }
                    let value = *token.value();
                    tokens.push((value.0, value.1, token.span()));
                }
            }
            Some(
                tokens
                    .iter()
                    .map(
                        |tok| format!("{:?} @ {}..{}: {:?}", tok.0, tok.2.lo(), tok.2.hi(), tok.1,),
                    )
                    .collect::<Vec<String>>()
                    .join("\n"),
            )
        }),
        ReplCtx {},
    );
    repl.run(
        format!(
            "Calypso CLI v{} - internal debugging command: lexer",
            env!("CARGO_PKG_VERSION")
        ),
        |_| String::from(">>> "),
    )
    .expect("REPL failure");
}

// pub fn dump(matches: &ArgMatches) {
//     let path = Path::new(matches.value_of("INPUT").unwrap());
//     if !path.exists() {
//         error(format!("file does not exist: `{}`", path.display()));
//         return;
//     }

//     /*let contents = match fs::read_to_string(&path) {
//             Ok(v) => v,
//             Err(err) => {
//                 error(format!(
//                     "while reading file `{}`: `{}`",
//                     path.display(),
//                     err
//                 ));
//                 return;
//             }
//         };

//         let raw = matches.is_present("raw");
//         let pretty = matches.is_present("pretty");
//     */
//     /*if raw {
//         /*let parsed = match CalypsoParser::parse(Rule::file, &contents) {
//             Ok(v) => v,
//             Err(err) => {
//                 error(format!(
//                     "syntax error while parsing file `{}`:\n{}",
//                     path.display(),
//                     err
//                 ));
//                 return;
//             }
//         };
//         if pretty {
//             println!("{:#?}", parsed);
//         } else {
//             println!("{:?}", parsed);
//         }*/
//     } else {
//         unimplemented!();
//     }*/
//     unimplemented!();
// }
