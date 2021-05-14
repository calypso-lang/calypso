use super::helpers::{is_valid_for, is_whitespace};
use super::{Lexer, Token, TokenType};

use calypso_ast::expr::{Radix, Suffix};
use calypso_base::span::Spanned;
use calypso_base::streams::Stream;
use calypso_diagnostic::prelude::*;

impl<'lex> Lexer<'lex> {
    pub(super) fn handle_number(&mut self) -> CalResult<Token<'lex>> {
        if self.handle_float_part()? {
            return Ok(self.new_token(TokenType::Float));
        }
        let possibly_incomplete_tok = self.handle_int(Radix::None)?;
        if self.handle_float_part()? {
            return Ok(self.new_token(TokenType::Float));
        }
        Ok(possibly_incomplete_tok)
    }

    pub(super) fn handle_int_leading_zero(&mut self) -> CalResult<Token<'lex>> {
        let ch = *self.peek().unwrap();
        match ch.value() {
            'd' => {
                self.next();
                self.handle_int(Radix::Decimal)
            }
            'x' => {
                self.next();
                self.handle_int(Radix::Hexadecimal)
            }
            'o' => {
                self.next();
                self.handle_int(Radix::Octal)
            }
            'b' => {
                self.next();
                self.handle_int(Radix::Binary)
            }
            '.' => {
                self.handle_float_part()?;
                Ok(self.new_token(TokenType::Float))
            }
            's' => {
                self.next();
                Ok(self.new_token(TokenType::Int {
                    suffix: Some(Suffix::Sint),
                    radix: Radix::None,
                }))
            }
            'u' => {
                self.next();
                Ok(self.new_token(TokenType::Int {
                    suffix: Some(Suffix::Uint),
                    radix: Radix::None,
                }))
            }
            'f' => {
                self.next();
                Ok(self.new_token(TokenType::Int {
                    suffix: Some(Suffix::Float),
                    radix: Radix::None,
                }))
            }
            ch if ch.is_ascii_digit() => {
                self.current_to_start();
                self.gorge_digits();
                // Leading zeroes are not a problem with floats.
                if self.handle_float_part()? {
                    return Ok(self.new_token(TokenType::Float));
                }
                let suffix = self.handle_suffix();
                let len = if suffix.is_some() {
                    self.next().unwrap().value_owned().len_utf8()
                } else {
                    0
                };
                gen_error!(sync self.grcx.borrow_mut(), self => {
                    E0025;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.new_span());
                            "did not expect a number here"
                    ],
                    notes: [
                        "leading zeroes are not allowed in integer literals, even ones with the `f` suffix.",
                        format!(
                            "help: perhaps you meant to use an octal literal: `0o{}`?",
                            self.slice(self.new_span()),
                        )
                    ]
                });
                Ok(self.new_token_with_span(
                    self.new_span().sub_hi(len),
                    TokenType::Int {
                        suffix,
                        // We assume octal, as other languages (e.g. C) use
                        // `0755` to mean `0o755`.
                        radix: Radix::Octal,
                    },
                ))
            }
            _ => Ok(self.new_token(TokenType::Int {
                suffix: None,
                radix: Radix::None,
            })),
        }
    }

    pub(super) fn handle_suffix(&mut self) -> Option<Suffix> {
        match self.peek().copied().map(Spanned::value_owned) {
            Some('s') => Some(Suffix::Sint),
            Some('u') => Some(Suffix::Uint),
            Some('f') => Some(Suffix::Float),
            _ => None,
        }
    }

    pub(super) fn handle_float_part(&mut self) -> CalResult<bool> {
        if self.peek_cond(|c| {
            let c = c.value_owned();
            c == 'e' || c == 'E' || c == '.'
        }) != Some(true)
        {
            return Ok(false);
        }
        self.handle_unexpected_underscore()?;

        if self.next_if(|c| c.value_owned() == '.').is_some() {
            self.inval_float_decimal()?;
            self.gorge_digits();
            self.handle_unexpected_underscore()?;
        }

        if self
            .next_if(|c| c.value_owned() == 'e' || c.value_owned() == 'E')
            .is_some()
        {
            // +/- are optional
            self.next_if(|c| c.value_owned() == '+' || c.value_owned() == '-');
            self.inval_float_exponent()?;
            self.gorge_digits();
            self.handle_unexpected_underscore()?;
        }

        Ok(true)
    }

    fn inval_float_decimal(&mut self) -> CalResult<()> {
        let start = self.start;
        if self.peek_cond(is_whitespace) == Some(true) {
            self.current_to_start();
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0028;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a decimal part of this float here"
                ]
            });
        } else if self.peek_cond(|c| {
            let c = c.value_owned();
            !c.is_ascii_digit() && c != 'e' && c != 'E'
        }) == Some(true)
        {
            self.current_to_start();
            let ch = self.peek().unwrap().value_owned();
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0027, ch = ch;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a decimal part of this float here"
                ]
            });
        } else if self.is_at_end() {
            self.current_to_start();
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0029;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a decimal part of this float here"
                ]
            });
        }
        self.set_start(start);
        Ok(())
    }

    fn inval_float_exponent(&mut self) -> CalResult<()> {
        let start = self.start;
        if self.peek_cond(is_whitespace) == Some(true) {
            self.current_to_start();
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0031;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected an exponent of this float here"
                ]
            });
        } else if self.peek_cond(|c| !c.value_owned().is_ascii_digit()) == Some(true) {
            self.current_to_start();
            let ch = self.peek().unwrap().value_owned();
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0030, ch = ch;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected an exponent of this float here"
                ]
            });
        } else if self.is_at_end() {
            self.current_to_start();
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0032;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected an exponent of this float here"
                ]
            });
        }
        self.set_start(start);
        Ok(())
    }

    fn handle_unexpected_underscore(&mut self) -> CalResult<()> {
        if self.prev_eq(&'_') == Some(true) {
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0026;
                labels: [
                    LabelStyle::Secondary =>
                        (self.source_id, self.prev().unwrap().span());
                        "help: remove this underscore"
                ]
            });
        }
        Ok(())
    }

    fn gorge_digits(&mut self) -> usize {
        self.gorge_digits_radix(Radix::Decimal)
    }

    fn gorge_digits_radix(&mut self, radix: Radix) -> usize {
        self.gorge_while(|c, n| is_valid_for(c, radix) || (n > 0 && c.value_owned() == '_'))
    }

    fn handle_int(&mut self, radix: Radix) -> CalResult<Token<'lex>> {
        let n_gorged = self.gorge_digits_radix(radix);
        if n_gorged == 0 && radix != Radix::None {
            self.current_to_start();
            gen_error!(sync self.grcx.borrow_mut(), self => {
                E0035;
                labels: [
                    LabelStyle::Primary =>
                        (self.source_id, self.new_span());
                        "expected a number here"
                ],
                notes: [
                    format!(
                        "help: perhaps you meant to use a zero literal with this base: `{}0`?",
                        radix,
                    )
                ]
            });
        }
        self.handle_unexpected_underscore()?;
        let suffix = self.handle_suffix();

        if radix != Radix::None {
            if let Some(Suffix::Float) = suffix {
                gen_error!(sync self.grcx.borrow_mut(), self => {
                    E0033;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.new_span());
                            "cannot use an explicit base for this float"
                    ]
                });
            }
        }

        let len = if suffix.is_some() {
            self.next().unwrap().value_owned().len_utf8()
        } else {
            0
        };
        Ok(self.new_token_with_span(
            self.new_span().sub_hi(len),
            TokenType::Int { suffix, radix },
        ))
    }
}
