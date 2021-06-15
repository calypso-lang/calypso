use super::helpers::{is_valid_for_char_literal, is_whitespace, is_whitespace_ch};
use super::{Lexer, Token, TokenType};

use calypso_base::span::Span;
use calypso_base::streams::Stream;
use calypso_diagnostic::diagnostic::{EnsembleBuilder, LabelStyle};
use calypso_diagnostic::prelude::*;

// todo(@ThePuzzlemaker: parse):
//   looks like a lot of these could be converted to sync rather than panic
impl<'lex> Lexer<'lex> {
    pub(super) fn handle_escape_character(&mut self) -> CalResult<bool> {
        let saved_start = self.start;
        self.current_to_start();
        if self.next_if_eq(&'\\').is_some() {
            match self.peek().map(|v| v.value_owned()) {
                Some('n') | Some('r') | Some('t') | Some('\\') | Some('0') | Some('\'')
                | Some('"') | Some('\r') | Some('\n') => {
                    self.next();
                }
                Some('x') => self.handle_hex_escape()?,
                Some('u') => self.handle_unicode_escape()?,
                Some(ch) => {
                    if is_whitespace_ch(ch) {
                        self.gcx.grcx.write().report_syncd(
                            EnsembleBuilder::new()
                                .error(|b| {
                                    b.code("E0008").short(err!(E0008)).label(
                                        LabelStyle::Primary,
                                        None,
                                        self.file_id,
                                        self.new_span(),
                                    )
                                })
                                .build(),
                        );
                        self.next();
                    } else {
                        self.next();
                        self.gcx.grcx.write().report_syncd(
                            EnsembleBuilder::new()
                                .error(|b| {
                                    b.code("E0006").short(err!(E0006, ch = ch)).label(
                                        LabelStyle::Primary,
                                        Some("this escape sequence is invalid"),
                                        self.file_id,
                                        self.new_span(),
                                    )
                                })
                                .build(),
                        );
                    }
                }
                None => {
                    self.gcx.grcx.write().report_fatal(
                        EnsembleBuilder::new()
                            .error(|b| {
                                b.code("E0007").short(err!(E0007)).label(
                                    LabelStyle::Primary,
                                    None,
                                    self.file_id,
                                    self.new_span(),
                                )
                            })
                            .build(),
                    );
                    return Err(DiagnosticError::Diagnostic.into());
                }
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
            if sp.is_none()
                || is_whitespace(sp.unwrap())
                || sp.unwrap().value_owned() == '\''
                || sp.unwrap().value_owned() == '\"'
            {
                if i == 1 {
                    self.gcx.grcx.write().report_fatal(
                        EnsembleBuilder::new()
                            .error(|b| {
                                b.code("E0004").short(err!(E0004)).label(
                                    LabelStyle::Primary,
                                    None,
                                    self.file_id,
                                    self.new_span(),
                                )
                            })
                            .build(),
                    );
                    return Err(DiagnosticError::Diagnostic.into());
                } else if i == 2 {
                    self.gcx.grcx.write().report_fatal(
                        EnsembleBuilder::new()
                            .error(|b| {
                                b.code("E0009")
                                    .short(err!(E0009))
                                    .label(LabelStyle::Primary, None, self.file_id, self.new_span())
                                    .note(format!(
                                        "perhaps you meant to use `\\x0{}`?",
                                        self.prev().unwrap().value_owned()
                                    ))
                            })
                            .build(),
                    );
                    return Err(DiagnosticError::Diagnostic.into());
                }
            }
            let sp = *sp.unwrap();
            let ch = sp.value_owned();

            if ch.is_ascii_hexdigit() {
                self.next();
            } else {
                self.set_start(sp.span());
                self.gcx.grcx.write().report_syncd(
                    EnsembleBuilder::new()
                        .error(|b| {
                            b.code("E0005").short(err!(E0005, ch = ch)).label(
                                LabelStyle::Primary,
                                None,
                                self.file_id,
                                self.new_span(),
                            )
                        })
                        .build(),
                );
                self.next();
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    pub(super) fn handle_unicode_escape(&mut self) -> CalResult<()> {
        // Handle the `u` in `\u{1234}`
        self.next();
        self.current_to_start();
        match self.peek().copied() {
            Some(sp) if is_whitespace(&sp) => {
                self.gcx.grcx.write().report_fatal(
                    EnsembleBuilder::new()
                        .error(|b| {
                            b.code("E0012").short(err!(E0012)).label(
                                LabelStyle::Primary,
                                None,
                                self.file_id,
                                self.new_span(),
                            )
                        })
                        .build(),
                );
                return Err(DiagnosticError::Diagnostic.into());
            }
            None => {
                self.gcx.grcx.write().report_fatal(
                    EnsembleBuilder::new()
                        .error(|b| {
                            b.code("E0011").short(err!(E0011)).label(
                                LabelStyle::Primary,
                                None,
                                self.file_id,
                                self.new_span(),
                            )
                        })
                        .build(),
                );
                return Err(DiagnosticError::Diagnostic.into());
            }
            Some(sp) if sp != '{' => {
                self.next();
                self.gcx.grcx.write().report_fatal(
                    EnsembleBuilder::new()
                        .error(|b| {
                            b.code("E0010")
                                .short(err!(E0010, ch = sp.value_owned()))
                                .label(LabelStyle::Primary, None, self.file_id, self.new_span())
                        })
                        .build(),
                );
                return Err(DiagnosticError::Diagnostic.into());
            }
            Some(..) => (),
        }
        self.next();

        let count = self.handle_unicode_escape_internal()?;

        if self.is_at_end() {
            self.current_to_start();
            self.gcx.grcx.write().report_fatal(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0015").short(err!(E0015)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
            return Err(DiagnosticError::Diagnostic.into());
        }

        let sp = *self.peek().unwrap();
        if is_whitespace(&sp) {
            self.current_to_start();
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0017").short(err!(E0017)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
        } else if self.peek_eq(&'}') != Some(true) {
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0016")
                            .short(err!(E0016, ch = sp.value_owned()))
                            .label(LabelStyle::Primary, None, self.file_id, self.new_span())
                    })
                    .build(),
            );
        }

        // We need to check for this after curly bracket checks
        if count == 0 {
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0019")
                            .short(err!(E0019))
                            .label(LabelStyle::Primary, None, self.file_id, self.new_span())
                            .note("help: if you wanted a null byte, you can use `\\u{0}` or `\\0`.")
                    })
                    .build(),
            );
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
                self.gcx.grcx.write().report_fatal(
                    EnsembleBuilder::new()
                        .error(|b| {
                            b.code("E0018").short(err!(E0018)).label(
                                LabelStyle::Primary,
                                None,
                                self.file_id,
                                self.new_span(),
                            )
                        })
                        .build(),
                );
                return Err(DiagnosticError::Diagnostic.into());
            } else if !ch.is_ascii_hexdigit() {
                self.gcx.grcx.write().report_fatal(
                    EnsembleBuilder::new()
                        .error(|b| {
                            b.code("E0014").short(err!(E0014, ch = ch)).label(
                                LabelStyle::Secondary,
                                Some("help: perhaps you meant to put a `}` here?"),
                                self.file_id,
                                self.new_span(),
                            )
                        })
                        .build(),
                );
                return Err(DiagnosticError::Diagnostic.into());
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
            if chs_found == 1 {
                expected_quote_here = self.current();
            }
            if self.handle_escape_character()? {
                chs_found += 1;
            } else if is_valid_for_char_literal(self.peek().unwrap()) {
                self.next();
                chs_found += 1;
            } else {
                if chs_found >= 1 {
                    chs_found += 1;
                    break;
                }
                self.current_to_start();
                let ch = self.next().unwrap().value_owned();
                self.gcx.grcx.write().report_syncd(
                    EnsembleBuilder::new()
                        .error(|b| {
                            b.code("E0020").short(err!(E0020)).label(
                                LabelStyle::Secondary,
                                Some(&format!("help: try escaping this character: `{:?}`", ch)),
                                self.file_id,
                                self.new_span().shrink_to_lo(),
                            )
                        })
                        .build(),
                );
                chs_found += 1;
            }
        }

        if chs_found > 1 {
            self.set_start(expected_quote_here);
            self.gcx.grcx.write().report_fatal(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0021").short(err!(E0021)).label(
                            LabelStyle::Primary,
                            Some("expected a `'` here"),
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
            return Err(DiagnosticError::Diagnostic.into());
        } else if chs_found == 0 && !self.is_at_end() {
            self.gcx.grcx.write().report_syncd(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0022")
                            .short(err!(E0022))
                            .label(LabelStyle::Primary, None, self.file_id, self.new_span())
                            .note("help: if you wanted an empty string, try `\"\"`.")
                    })
                    .build(),
            );
        }

        if self.is_at_end() {
            self.current_to_start();
            self.gcx.grcx.write().report_fatal(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0023").short(err!(E0023)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
            return Err(DiagnosticError::Diagnostic.into());
        }
        self.next();

        self.set_start(saved_start);
        Ok(self.new_token(TokenType::Char))
    }

    pub(super) fn handle_string_literal(&mut self) -> CalResult<Token<'lex>> {
        while self.peek_eq(&'"') != Some(true) && !self.is_at_end() {
            if !self.handle_escape_character()? {
                self.next();
            }
        }

        if self.peek_eq(&'"') != Some(true) {
            self.current_to_start();
            self.gcx.grcx.write().report_fatal(
                EnsembleBuilder::new()
                    .error(|b| {
                        b.code("E0024").short(err!(E0024)).label(
                            LabelStyle::Primary,
                            None,
                            self.file_id,
                            self.new_span(),
                        )
                    })
                    .build(),
            );
            return Err(DiagnosticError::Diagnostic.into());
        }

        self.next();

        Ok(self.new_token(TokenType::String))
    }
}
