use std::io::prelude::*;
use std::rc::Rc;
use std::{cell::RefCell, sync::Arc};

use calypso_base::{
    session::BaseSession,
    ui::{atty::Stream, parse_color_pref},
};
use calypso_diagnostic::{prelude::*, report::GlobalReportingCtxt};
use calypso_parsing::parser::tokens::process_iter;
use calypso_parsing::{lexer::Lexer, parser::grammar::ExprParser};

use lalrpop_util::ParseError;

fn main() {
    let sess = Arc::new(BaseSession::new(
        parse_color_pref("auto", Stream::Stdout),
        parse_color_pref("auto", Stream::Stderr),
    ));
    let stdin = std::io::stdin();
    loop {
        let mut contents = String::new();
        stdin.lock().read_line(&mut contents).expect("io");

        let mut files = FileMgr::new();
        let source_id = files.add("<anon>".to_string(), contents);
        let grcx = Rc::new(RefCell::new(GlobalReportingCtxt::new(Arc::clone(&sess))));
        let lexer = Lexer::new(
            source_id,
            files.get(source_id).unwrap().source(),
            &files,
            Rc::clone(&grcx),
        );
        let iter = lexer.into_iter();
        let iter = process_iter(iter, true);
        let parser = ExprParser::new();
        let res = parser.parse(source_id, iter);
        if let Ok(parsed) = res {
            println!("{:?}", parsed);
        } else if let Err(err) = res {
            if matches!(err, ParseError::UnrecognizedEOF { .. }) {
                break;
            } else {
                println!("{:#?}", err);
            }
        }
    }
}
