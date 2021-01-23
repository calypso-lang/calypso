use super::helpers::{is_valid_for_char_literal, is_whitespace, is_whitespace_ch};
use super::{Lexer, Token, TokenType};

use calypso_base::span::Span;
use calypso_base::streams::Stream;
use calypso_diagnostic::prelude::*;

impl<'lex> Lexer<'lex> {
    pub(super) fn handle_escape_character(&mut self) -> CalResult<bool> {
        let saved_start = self.start;
        self.current_to_start();
        if self.next_if_eq(&'\\').is_some() {
            match self.peek().map(|v| v.value_owned()) {
                Some('n') | Some('r') | Some('t') | Some('\\') | Some('0') | Some('\'')
                | Some('"') => {
                    self.next();
                }
                Some('x') => self.handle_hex_escape()?,
                Some('u') => self.handle_unicode_escape()?,
                Some(ch) => {
                    if is_whitespace_ch(ch) {
                        gen_error!(Err(self => {
                            E0008;
                            labels: [
                                LabelStyle::Primary =>
                                    (self.source_id, self.new_span());
                                    "expected an escape sequence here"
                            ]
                        }) as ())?
                    }
                    self.next();
                    gen_error!(Err(self => {
                        E0006, ch = ch;
                        labels: [
                            LabelStyle::Primary =>
                                (self.source_id, self.new_span());
                                "this escape sequence is unknown"
                        ]
                    }) as ())?
                }
                None => gen_error!(Err(self => {
                        E0007;
                        labels: [
                            LabelStyle::Primary =>
                                (self.source_id, self.new_span());
                                "expected an escape sequence here"
                        ]
                    }) as ())?,
            }
            self.set_start(saved_start);
            return Ok(true);
        }

        self.set_start(saved_start);
        // We don't care *what* sequence was found, just if there was one.
        Ok(false)
    }

    pub(super) fn handle_hex_escape(&mut self) -> CalResult<()> {
        // Handle the `x` in `\x41`
        self.next();
        self.current_to_start();
        for i in 1..=2 {
            let sp = self.peek();
            if sp.is_none() || is_whitespace(sp.unwrap()) {
                if i == 1 {
                    gen_error!(Err(self => {
                        E0004;
                        labels: [
                            LabelStyle::Primary =>
                                (self.source_id, self.new_span());
                                "expected two hexadecimal digits here"
                        ]
                    }) as ())?
                } else if i == 2 {
                    gen_error!(Err(self => {
                        E0009;
                        labels: [
                            LabelStyle::Primary =>
                                (self.source_id, self.new_span());
                                "found only one hexadecimal digit here"
                        ],
                        notes: [
                            format!(
                                "perhaps you meant to use `\\x0{}`?",
                                self.prev().unwrap().value_owned()
                            )
                        ]
                    }) as ())?
                } else {
                    return Ok(());
                }
            }
            let sp = *sp.unwrap();
            let ch = sp.value_owned();

            if ch.is_ascii_hexdigit() {
                self.next();
            } else {
                self.set_start(sp.span());
                gen_error!(Err(self => {
                    E0005, ch = ch;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.new_span());
                            "found an invalid digit here"
                    ]
                }) as ())?
            }
        }
        Ok(())
    }

    pub(super) fn handle_unicode_escape(&mut self) -> CalResult<()> {
        // Handle the `u` in `\u{1234}`
        self.next();
        self.current_to_start();
        match self.peek().copied() {
            Some(sp) if is_whitespace(&sp) => gen_error!(Err(self => {
                    E0012;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.new_span());
                            "this should be an opening curly bracket"
                    ]
                }) as ())?,
            None => gen_error!(Err(self => {
                E0011;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "this should be an opening curly bracket"
                ]
            }) as ())?,
            Some(sp) if sp != '{' => {
                self.next();
                gen_error!(Err(self => {
                    E0010, ch = sp.value_owned();
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.new_span());
                            "this should be an opening curly bracket"
                    ]
                }) as ())?
            }
            Some(..) => (),
        }
        self.next();

        let count = self.handle_unicode_escape_internal()?;

        if self.is_at_end() {
            self.current_to_start();
            gen_error!(Err(self => {
                E0015;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a closing curly bracket here"
                ]
            }) as ())?
        }

        let sp = *self.peek().unwrap();
        if is_whitespace(&sp) {
            self.current_to_start();
            gen_error!(Err(self => {
                E0017;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a closing curly bracket here"
                ]
            }) as ())?
        } else if self.peek_eq(&'}') != Some(true) {
            gen_error!(Err(self => {
                E0016, ch = sp.value_owned();
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a closing curly bracket here"
                ]
            }) as ())?
        }

        // We need to check for this after curly bracket checks
        if count == 0 {
            gen_error!(Err(self => {
                E0019;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected at least one hex digit here"
                ],
                notes: [
                    "if you wanted a null byte, you can use `\\u{0}` or `\\0`"
                ]
            }) as ())?
        }
        // Handle closing `}`
        self.next();
        Ok(())
    }

    fn handle_unicode_escape_internal(&mut self) -> CalResult<usize> {
        let mut count = 0;
        while self.peek_eq(&'}') != Some(true) && !self.is_at_end() {
            self.current_to_start();
            let sp = self.peek().unwrap();
            let ch = sp.value_owned();
            if count == 6 {
                break;
            } else if is_whitespace(&sp) {
                gen_error!(Err(self => {
                    E0018;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.new_span());
                            "expected a hexadecimal digit here"
                    ]
                }) as ())?
            } else if !ch.is_ascii_hexdigit() {
                gen_error!(Err(self => {
                    E0014, ch = ch;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.new_span());
                            "found an invalid digit here. perhaps you meant to put a `}` here?"
                    ]
                }) as ())?
            }
            self.next();
            count += 1;
        }
        Ok(count)
    }

    pub(super) fn handle_char_literal(&mut self) -> CalResult<Token<'lex>> {
        let saved_start = self.start;
        let mut chs_found = 0;
        let mut expected_quote_here = Span::new_dummy();
        while self.peek_eq(&'\'') != Some(true) && !self.is_at_end() {
            if self.handle_escape_character()? {
                chs_found += 1;
            } else if is_valid_for_char_literal(self.peek().unwrap()) {
                self.next();
                chs_found += 1;
            } else {
                let ch = self.next().unwrap().value_owned();
                gen_error!(sync self.grcx.borrow_mut(), self => {
                    E0020;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.new_span());
                            format!(
                                "this character ({:?}) is invalid here; it must be escaped",
                                ch
                            )
                    ]
                });
                chs_found += 1;
            }
            if chs_found == 1 {
                expected_quote_here = self.current();
            }
        }

        if chs_found > 1 {
            self.set_start(expected_quote_here);
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0021;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a `'` here"
                ]
            });
        } else if chs_found == 0 {
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0022;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected one character here"
                ]
            });
        }

        if self.is_at_end() {
            self.current_to_start();
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0023;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a `'` here"
                ]
            });
        }
        self.next();

        self.set_start(saved_start);
        Ok(self.new_token(TokenType::CharLiteral))
    }

    pub(super) fn handle_string_literal(&mut self) -> CalResult<Token<'lex>> {
        while self.peek_eq(&'"') != Some(true) && !self.is_at_end() {
            let sp = *self.peek().unwrap();
            if self.handle_escape_character()? {
                self.next();
            } else if sp == '\n' || sp == '\r' {
                gen_error!(sync self.grcx.borrow_mut(), self => {
                    E0025;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.current());
                            "newlines or carriage returns are not valid in string literals"
                    ]
                });
                self.next();
            } else {
                self.next();
            }
        }

        if self.peek_eq(&'"') != Some(true) {
            self.current_to_start();
            gen_error!(Err(self => {
                E0024;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a `\"` here"
                ]
            }) as ())?
        }

        self.next();

        Ok(self.new_token(TokenType::StringLiteral))
    }
}
