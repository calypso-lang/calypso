use std::io::prelude::*;
use std::rc::Rc;
use std::{cell::RefCell, sync::Arc};

use calypso_ast::pretty::PrettyPrinter;
use calypso_ast::traverse::Visitor;
use calypso_base::sourcemgr::SourceMgr;
use calypso_base::{
    session::BaseSession,
    ui::{atty::Stream, parse_color_pref},
};
use calypso_diagnostic::{prelude::*, report::GlobalReportingCtxt};
use calypso_parsing::parser::tokens::process_iter;
use calypso_parsing::session::ParseSess;
use calypso_parsing::{lexer::Lexer, parser::grammar::ExprParser};

use lalrpop_util::ParseError;

fn main() {
    // let stdin = std::io::stdin();
    // loop {
    let mut contents = String::from("awegawe");
    // stdin.lock().read_line(&mut contents).expect("io");

    let mut smgr = SourceMgr::new();
    let source_id = smgr.intern_add("<anon>", contents);
    smgr.finalize();
    let sess = Arc::new(BaseSession::new(
        parse_color_pref("auto", Stream::Stdout),
        parse_color_pref("auto", Stream::Stderr),
        smgr,
    ));

    let grcx = Arc::new(GlobalReportingCtxt::new());
    let mut lexer =
        Lexer::new(source_id, Arc::new(ParseSess::new(sess, Arc::clone(&grcx)))).unwrap();
    println!("{:?}", lexer.scan().unwrap());
    drop(lexer);
    // let iter = lexer.into_iter();
    // let iter = process_iter(iter, true);
    // let parser = ExprParser::new();
    // let res = parser.parse(source_id, iter);
    // if let Ok(parsed) = res {
    //     let mut printer = PrettyPrinter::default();
    //     printer.visit_expr(&parsed).unwrap();
    //     println!("{}", printer);
    // } else if let Err(err) = res {
    //     if matches!(err, ParseError::UnrecognizedEOF { .. }) {
    //         break;
    //     } else {
    //         println!("{:#?}", err);
    //     }
    // }
    // grcx.borrow()
    //     .errors()
    //     .iter()
    //     .take(1)
    //     .for_each(|e| println!("{}", e));
    // }
}
