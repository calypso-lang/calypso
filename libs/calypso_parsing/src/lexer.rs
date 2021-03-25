use std::cell::RefCell;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

use calypso_base::{
    span::{Span, Spanned},
    streams::{Stream, StringStream},
};
use calypso_diagnostic::prelude::*;
use calypso_diagnostic::report::GlobalReportingCtxt;

pub use types::*;

pub mod types;

mod helpers;
mod ident_kw;
mod lit;
mod num;
mod scan;
mod ws;

pub type Token<'lex> = Spanned<(TokenType, Lexeme<'lex>)>;
pub type Lexeme<'lex> = &'lex str;

#[derive(Debug)]
pub struct Lexer<'lex> {
    stream: StringStream<'lex>,
    source_id: usize,
    files: &'lex FileMgr,
    start: Span,
    grcx: Rc<RefCell<GlobalReportingCtxt>>,
}

impl<'lex> Deref for Lexer<'lex> {
    type Target = StringStream<'lex>;

    fn deref(&self) -> &Self::Target {
        &self.stream
    }
}

impl<'lex> DerefMut for Lexer<'lex> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stream
    }
}

impl<'lex> Lexer<'lex> {
    pub fn new(
        source_id: usize,
        source: &'lex str,
        files: &'lex FileMgr,
        grcx: Rc<RefCell<GlobalReportingCtxt>>,
    ) -> Self {
        Self {
            stream: StringStream::new(source),
            source_id,
            files,
            start: Span::default(),
            grcx,
        }
    }
}

impl<'lex> Lexer<'lex> {
    /// Set the `start` span to the span of the next character or the empty span of the EOF.
    fn current_to_start(&mut self) {
        self.start = self.current();
    }

    fn set_start(&mut self, start: Span) {
        self.start = start;
    }

    /// Get the span of the next character or the empty span of the EOF.
    fn current(&self) -> Span {
        self.peek()
            .map_or_else(|| Span::new_shrunk(self.stream[..].len()), Spanned::span)
    }

    fn new_span(&self) -> Span {
        self.start.until(self.current())
    }

    fn new_token(&self, r#type: TokenType) -> Token<'lex> {
        self.new_token_with_span(self.new_span(), r#type)
    }

    fn new_token_with_span(&self, span: Span, r#type: TokenType) -> Token<'lex> {
        Token::new(span, (r#type, self.slice(span)))
    }
}

impl<'lex> IntoIterator for Lexer<'lex> {
    type IntoIter = Iter<'lex>;
    type Item = CalResult<Token<'lex>>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            lexer: self,
            encountered_eof: false,
            encountered_error: false,
        }
    }
}

#[derive(Debug)]
pub struct Iter<'lex> {
    lexer: Lexer<'lex>,
    encountered_error: bool,
    encountered_eof: bool,
}

impl<'lex> Iterator for Iter<'lex> {
    type Item = CalResult<Token<'lex>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.encountered_error || self.encountered_eof {
            None
        } else {
            let res = self.lexer.scan();
            if res.is_err() {
                self.encountered_error = true;
            }
            if let Ok(res) = res {
                if res.value().0 == TokenType::Eof {
                    self.encountered_eof = true;
                    return None;
                }
            }
            Some(res)
        }
    }
}
