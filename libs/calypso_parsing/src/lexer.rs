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

pub use ident_kw::KEYWORD_TRIE;
pub use types::*;

pub mod types;

mod helpers;
mod ident_kw;
mod lit;
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
        let span = self.new_span();
        Token::new(span, (r#type, self.slice(span)))
    }
}

/*
    fn number(&mut self) -> Result<Token<'lex>, ()> {
        let radix = if self.last() == '0' {
            if self.peek().is_ascii_digit() {
                self.advance();
                Radix::Decimal
            } else if self.peek() == '\0' {
                Radix::Decimal
            } else {
                let ch = self.peek();
                self.advance();
                match ch {
                    'b' => Radix::Binary,
                    'x' => Radix::Hexadecimal,
                    'o' => Radix::Octal,
                    'e' | '.' => {
                        self.backup();
                        Radix::Decimal
                    }
                    _ => {
                        println!("Invalid number base.");
                        return Err(());
                    }
                }
            }
        } else {
            Radix::Decimal
        };

        while !self.is_at_end() {
            let ch = self.peek();
            if ch == '\n' || ch == '.' || ch == 'e' || ch == 'E' {
                break;
            }
            if is_valid_digit_for_radix(ch, radix) && is_valid_for_any_radix(ch) {
                self.advance();
            } else if !is_valid_for_any_radix(ch) {
                break;
            } else {
                println!("Invalid digit for number.");
                return Err(());
            }
        }

        Ok(
            // Is a float literal
            if self.peek() == '.' {
                if radix != Radix::Decimal {
                    println!("Cannot have a float with a non-10 base.");
                    return Err(());
                }
                // Consume the `.`.
                self.advance();

                if !self.peek().is_ascii_digit() {
                    println!("Expected decimal component of float");
                    return Err(());
                }

                while !self.is_at_end() {
                    let ch = self.peek();
                    if ch == '\n' || ch == 'E' || ch == 'e' {
                        break;
                    }
                    if ch.is_ascii_digit() {
                        self.advance();
                    } else {
                        println!("Invalid digit for number.");
                        return Err(());
                    }
                }

                // Has exponent
                if self.peek() == 'E' || self.peek() == 'e' {
                    // Consume the `E` or `e`.
                    self.advance();

                    if !self.peek().is_ascii_digit() {
                        println!("Expected exponent");
                        return Err(());
                    }

                    while !self.is_at_end() {
                        let ch = self.peek();
                        if ch == '\n' {
                            break;
                        }
                        if ch.is_ascii_digit() {
                            self.advance();
                        } else {
                            println!("Invalid digit for number.");
                            return Err(());
                        }
                    }
                }

                self.new_token(TokenType::FloatLiteral)
            } else if self.peek() == 'e' || self.peek() == 'E' {
                // Has exponent
                // Consume the `E` or `e`.
                self.advance();

                if !self.peek().is_ascii_digit() {
                    println!("Expected exponent");
                    return Err(());
                }

                while !self.is_at_end() {
                    let ch = self.peek();
                    if ch == '\n' {
                        break;
                    }
                    if ch.is_ascii_digit() {
                        self.advance();
                    } else {
                        println!("Invalid digit for number.");
                        return Err(());
                    }
                }

                self.new_token(TokenType::FloatLiteral)
            } else {
                self.new_token(TokenType::IntLiteral(radix))
            },
        )
    }

    fn string(&mut self) -> Result<Token<'lex>, ()> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                println!("Found a newline inside a string.");
                return Err(());
            }
            if !self.escape_character()? {
                self.advance();
            };
        }

        if self.is_at_end() {
            println!("Unterminated string.");
            return Err(());
        }

        // Closing quote
        self.advance();
        Ok(self.new_token(TokenType::StringLiteral))
    }
*/
