use std::cell::RefCell;
use std::io::prelude::*;
use std::rc::Rc;

use calypso_diagnostic::{prelude::*, report::GlobalReportingCtxt};
use calypso_parsing::parser::tokens::process_iter;
use calypso_parsing::{lexer::Lexer, parser::grammar::ExprParser};

use lalrpop_util::ParseError;

fn main() {
    let stdin = std::io::stdin();
    loop {
        let mut contents = String::new();
        stdin.lock().read_line(&mut contents).expect("io");

        let mut files = FileMgr::new();
        let source_id = files.add("<anon>".to_string(), contents);
        let grcx = Rc::new(RefCell::new(GlobalReportingCtxt::new()));
        let lexer = Lexer::new(
            source_id,
            files.get(source_id).unwrap().source(),
            &files,
            Rc::clone(&grcx),
        );
        let iter = lexer.into_iter();
        let iter = process_iter(iter, true);
        // for i in iter {
        // println!("{:#?}", i);
        // }
        let parser = ExprParser::new();
        let res = parser.parse(source_id, iter);
        if let Ok(parsed) = res {
            // println!("{:?}", parsed);
        } else if let Err(err) = res {
            if matches!(err, ParseError::UnrecognizedEOF { .. }) {
                break;
            } else {
                println!("{:#?}", err);
            }
        }
    }
}
