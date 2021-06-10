use std::mem::MaybeUninit;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr;
use std::sync::Arc;

use calypso_base::symbol::Symbol;
use calypso_base::{
    span::{Span, Spanned},
    streams::{Stream, StringStream},
};
use calypso_diagnostic::prelude::*;
use calypso_diagnostic::report::GlobalReportingCtxt;
use calypso_diagnostic::reporting::files::Files;

pub use types::*;

use crate::session::ParseSess;

pub mod types;

mod helpers;
mod ident_kw;
mod lit;
mod num;
mod scan;
mod ws;

pub type Token<'lex> = Spanned<(TokenType, Lexeme<'lex>)>;
pub type Lexeme<'lex> = &'lex str;

pub struct Lexer<'lex> {
    stream: StringStream<'lex>,
    source_id: Symbol,
    // source: &'lex str,
    start: Span,
    sess: Arc<ParseSess>,
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
    // no.
    // pub fn new(source_id: Symbol, sess: Arc<ParseSess>) -> CalResult<Self> {
    //     let mut zelf = MaybeUninit::<Self>::uninit();
    //     let ptr = zelf.as_mut_ptr();

    //     // SAFETY: It is safe to initialize these fields as their pointers are
    //     // valid and the values are valid.
    //     unsafe {
    //         ptr::addr_of_mut!((*ptr).source_id).write(source_id);
    //         ptr::addr_of_mut!((*ptr).sess).write(sess);
    //         ptr::addr_of_mut!((*ptr).start).write(Span::default());
    //     }

    //     // SAFETY: We have already initialized the `sess` field.
    //     let source = unsafe { &(*ptr).sess }
    //         .bsess
    //         .sourcemgr
    //         .source(source_id)
    //         .map_err(|e| CalError::Other(e.into()));

    //     if let Err(err) = source {
    //         // We must drop the fields in order to not potentially leak them.
    //         // SAFETY: We have already initialized these fields, and it is
    //         // valid to drop them (they have not already been dropped and they
    //         // will not be dropped again)
    //         unsafe {
    //             ptr::drop_in_place(ptr::addr_of_mut!((*ptr).source_id));
    //             ptr::drop_in_place(ptr::addr_of_mut!((*ptr).sess));
    //             ptr::drop_in_place(ptr::addr_of_mut!((*ptr).start));
    //         }
    //         return Err(err);
    //     }

    //     // Will not panic (since we've already checked the value), so we won't
    //     // leak anything.
    //     let source = source.unwrap();

    //     let stream = StringStream::new(source);

    //     // SAFETY: It is safe to initialize these fields as their pointers are
    //     // valid and the values are valid.
    //     unsafe {
    //         // ptr::addr_of_mut!((*ptr).source).write(source);
    //         ptr::addr_of_mut!((*ptr).stream).write(stream);
    //     }

    //     // SAFETY: We have initialized all the fields.
    //     let zelf = unsafe { zelf.assume_init() };

    //     Ok(zelf)
    // }
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
