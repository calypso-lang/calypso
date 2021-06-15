use super::helpers::{is_valid_for, is_whitespace};
use super::{Lexer, Token, TokenType};

use calypso_ast::expr::{Radix, Suffix};
use calypso_base::span::Spanned;
use calypso_base::streams::Stream;
use calypso_diagnostic::diagnostic::{EnsembleBuilder, LabelStyle};
use calypso_diagnostic::prelude::*;

impl<'lex> Lexer<'lex> {
    pub(super) fn handle_number(&mut self) -> Token<'lex> {
        if self.handle_float_part() {
            return self.new_token(TokenType::Float);
        }
        let possibly_incomplete_tok = self.handle_int(Radix::None);
        if self.handle_float_part() {
            return self.new_token(TokenType::Float);
        }
        possibly_incomplete_tok
    }

    pub(super) fn handle_int_leading_zero(&mut self) -> Token<'lex> {
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
                self.handle_float_part();
                self.new_token(TokenType::Float)
            }
            's' => {
                self.next();
                self.new_token(TokenType::Int {
                    suffix: Some(Suffix::Sint),
                    radix: Radix::None,
                })
            }
            'u' => {
                self.next();
                self.new_token(TokenType::Int {
                    suffix: Some(Suffix::Uint),
                    radix: Radix::None,
                })
            }
            'f' => {
                self.next();
                self.new_token(TokenType::Int {
                    suffix: Some(Suffix::Float),
                    radix: Radix::None,
                })
            }
            ch if ch.is_ascii_digit() => {
                self.current_to_start();
                self.gorge_digits();
                // Leading zeroes are not a problem with floats.
                if self.handle_float_part() {
                    return self.new_token(TokenType::Float);
                }
                let suffix = self.handle_suffix();
                let len = if suffix.is_some() {
                    self.next().unwrap().value_owned().len_utf8()
                } else {
                    0
                };

                self.gcx.grcx.write().report_syncd(
                    EnsembleBuilder::new()
                        .error(|b| {
                            b.code("E0025").short(err!(E0025)).label(
                                LabelStyle::Primary,
                                Some("didn't expect a number here"),
                                self.file_id,
                                self.new_span(),
                            ).note("leading zeroes are not allowed in integer literals, even ones with the `f` suffix.")
                            .note(format!("help: perhaps you meant to use an octal literal: `0o{}`?", self.slice(self.new_span())))
                        })
                        .build(),
                );
                self.new_token_with_span(
                    self.new_span().sub_hi(len),
                    TokenType::Int {
                        suffix,
                        // We assume octal, as other languages (e.g. C) use
                        // `0755` to mean `0o755`.
                        radix: Radix::Octal,
                    },
                )
            }
            _ => self.new_token(TokenType::Int {
                suffix: None,
                radix: Radix::None,
            }),
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

    pub(super) fn handle_float_part(&mut self) -> bool {
        let base_cond = |c: &Spanned<char>| {
            let c = c.value_owned();
            c == 'e' || c == 'E' || c == '.'
        };
        if self.peek_cond(base_cond) != Some(true) {
            return false;
        }
        self.handle_unexpected_underscore();

        if self.next_if(|c| c.value_owned() == '.').is_some() {
            self.inval_float_decimal();
            self.gorge_digits();
            self.handle_unexpected_underscore();
        }

        if self
            .next_if(|c| c.value_owned() == 'e' || c.value_owned() == 'E')
            .is_some()
        {
            // +/- are optional
            self.next_if(|c| c.value_owned() == '+' || c.value_owned() == '-');
            self.inval_float_exponent();
            self.gorge_digits();
            self.handle_unexpected_underscore();
        }

        true
    }

    fn inval_float_decimal(&mut self) {
        let start = self.start;
        let exp_part = |c: &Spanned<char>| {
            let c = c.value_owned();
            !c.is_ascii_digit() && c != 'e' && c != 'E'
        };
        if self.peek_cond(is_whitespace) == Some(true) {
            self.current_to_start();
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0028").short(err!(E0028)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
        } else if self.peek_cond(exp_part) == Some(true) {
            self.current_to_start();
            let ch = self.peek().unwrap().value_owned();
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0027").short(err!(E0027, ch = ch)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
        } else if self.is_at_end() {
            self.current_to_start();
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0029").short(err!(E0029)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
        }
        self.set_start(start);
    }

    fn inval_float_exponent(&mut self) {
        let start = self.start;
        if self.peek_cond(is_whitespace) == Some(true) {
            self.current_to_start();
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0031").short(err!(E0031)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
        } else if self.peek_cond(|c| !c.value_owned().is_ascii_digit()) == Some(true) {
            self.current_to_start();
            let ch = self.peek().unwrap().value_owned();
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0030").short(err!(E0030, ch = ch)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
        } else if self.is_at_end() {
            self.current_to_start();
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0032").short(err!(E0032)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
        }
        self.set_start(start);
    }

    fn handle_unexpected_underscore(&mut self) {
        if self.prev_eq(&'_') == Some(true) {
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0026").short(err!(E0026)).label(
                            LabelStyle::Secondary,
                            Some("help: remove this underscore"),
                            self.file_id,
                            self.prev().unwrap().span(),
                        )
                    })
                    .build(),
            );
        }
    }

    fn gorge_digits(&mut self) -> usize {
        self.gorge_digits_radix(Radix::Decimal)
    }

    fn gorge_digits_radix(&mut self, radix: Radix) -> usize {
        self.gorge_while(|c, n| is_valid_for(c, radix) || (n > 0 && c.value_owned() == '_'))
    }

    fn handle_int(&mut self, radix: Radix) -> Token<'lex> {
        let n_gorged = self.gorge_digits_radix(radix);
        if n_gorged == 0 && radix != Radix::None {
            self.current_to_start();
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0035").short(err!(E0035)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        ).note(format!("help: perhaps you meant to use a zero literal with this base: `{}0`?", radix))
                    })
                    .build(),
            );
        }
        self.handle_unexpected_underscore();
        let suffix = self.handle_suffix();

        if radix != Radix::None {
            if let Some(Suffix::Float) = suffix {
                self.gcx.grcx.write().report_syncd(
                    EnsembleBuilder::new()
                        .error(|b| {
                            b.code("E0033").short(err!(E0033)).label(
                                LabelStyle::Primary,
                                None,
                                self.file_id,
                                self.new_span(),
                            )
                        })
                        .build(),
                );
            }
        }

        let len = if suffix.is_some() {
            self.next().unwrap().value_owned().len_utf8()
        } else {
            0
        };
        self.new_token_with_span(
            self.new_span().sub_hi(len),
            TokenType::Int { suffix, radix },
        )
    }
}
