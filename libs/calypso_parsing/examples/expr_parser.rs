use std::cell::RefCell;
use std::rc::Rc;

use calypso_diagnostic::{prelude::*, report::GlobalReportingCtxt};
use calypso_parsing::parser::tokens::process_iter;
use calypso_parsing::{lexer::Lexer, parser::grammar::ExprParser};

fn main() {
    let mut files = FileMgr::new();
    let source_id = files.add(
        "<anon>".to_string(),
        ":foobar + :\"baz bar \\u{1f308}\\u{200d}\\u{1f3f3}\"".to_string(),
    );
    let grcx = Rc::new(RefCell::new(GlobalReportingCtxt::new()));
    let lexer = Lexer::new(
        source_id,
        files.get(source_id).unwrap().source(),
        &files,
        Rc::clone(&grcx),
    );
    let iter = lexer.into_iter();
    let iter = process_iter(iter, true);
    let parser = ExprParser::new();
    println!("{:#?}", parser.parse(source_id, iter).unwrap());
}
