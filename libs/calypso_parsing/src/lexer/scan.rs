use super::helpers::is_ident_start;
use super::{Lexer, Token, TokenType};

use calypso_base::streams::Stream;
use calypso_diagnostic::prelude::*;

impl<'lex> Lexer<'lex> {
    /// Scan a single token.
    ///
    /// # Errors
    /// The errors returned by this function are of type [`CalError`].
    /// When the error is of type [`CalErrorKind::Diagnostic`], it's
    /// an error that was impossible to recover from.
    #[allow(clippy::too_many_lines)]
    pub fn scan(&mut self) -> CalResult<Token<'lex>> {
        if let Some(wstok) = self.handle_whitespace()? {
            return Ok(wstok);
        }
        self.current_to_start();

        if self.is_at_end() {
            return Ok(self.new_token(TokenType::Eof));
        }

        // We've already checked if we're at the end (which is when it gives none), so
        // unwrapping should be safe here.
        let span = self.next().unwrap();
        let ch = span.value_owned();

        // Is valid character for identifier's first character
        if is_ident_start(&span) {
            return self.handle_identifier();
        } else if ch == '\'' {
            return self.handle_char_literal();
        } else if ch == '"' {
            return self.handle_string_literal();
        }

        let token_type = match ch {
            '<' if self.next_if_eq(&'>').is_some() => TokenType::LtGt,
            '<' if self.next_if_eq(&'<').is_some() => {
                if self.next_if_eq(&'=').is_some() {
                    TokenType::LtLtEq
                } else {
                    TokenType::Lt
                }
            }
            '<' if self.next_if_eq(&'=').is_some() => TokenType::LtEq,
            '<' => TokenType::Lt,

            '>' if self.next_if_eq(&'>').is_some() => {
                if self.next_if_eq(&'=').is_some() {
                    TokenType::GtGtEq
                } else {
                    TokenType::GtGt
                }
            }
            '>' if self.next_if_eq(&'=').is_some() => TokenType::GtEq,
            '>' => TokenType::Gt,

            '=' if self.next_if_eq(&'=').is_some() => TokenType::EqEq,
            '=' => TokenType::Eq,

            '!' if self.next_if_eq(&'=').is_some() => TokenType::BangEq,
            '!' => TokenType::Bang,

            '|' if self.next_if_eq(&'>').is_some() => TokenType::PipeGt,
            '|' if self.next_if_eq(&'|').is_some() => TokenType::PipePipe,
            '|' if self.next_if_eq(&'=').is_some() => TokenType::PipeEq,
            '|' => TokenType::Pipe,

            '&' if self.next_if_eq(&'&').is_some() => TokenType::AndAnd,
            '&' if self.next_if_eq(&'=').is_some() => TokenType::AndEq,
            '&' => TokenType::And,

            '+' if self.next_if_eq(&'=').is_some() => TokenType::PlusEq,
            '+' => TokenType::Plus,

            '-' if self.next_if_eq(&'=').is_some() => TokenType::MinusEq,
            '-' if self.next_if_eq(&'>').is_some() => TokenType::Arrow,
            '-' => TokenType::Minus,

            '*' if self.next_if_eq(&'*').is_some() => {
                if self.next_if_eq(&'=').is_some() {
                    TokenType::StarStarEq
                } else {
                    TokenType::StarStar
                }
            }
            '*' if self.next_if_eq(&'=').is_some() => TokenType::StarEq,
            '*' => TokenType::Star,

            '/' if self.next_if_eq(&'=').is_some() => TokenType::SlashEq,
            '/' => TokenType::Slash,

            '%' if self.next_if_eq(&'=').is_some() => TokenType::PercentEq,
            '%' => TokenType::Percent,

            '^' if self.next_if_eq(&'=').is_some() => TokenType::CaretEq,
            '^' => TokenType::Caret,

            '~' => TokenType::Tilde,

            '(' => TokenType::LParen,
            ')' => TokenType::RParen,

            '{' => TokenType::LBrace,
            '}' => TokenType::RBrace,

            '[' => TokenType::LBracket,
            ']' => TokenType::RBracket,

            ',' => TokenType::Comma,
            ';' => TokenType::Semi,

            '.' if self.next_if_eq(&'.').is_some() => {
                if self.next_if_eq(&'=').is_some() {
                    TokenType::DotDotEq
                } else {
                    TokenType::DotDot
                }
            }
            '.' => TokenType::Dot,

            // `'_' => Under` is already taken care of by idents
            '#' if self.next_if_eq(&'!').is_some() => TokenType::HashBang,
            '#' => TokenType::Hash,

            // Unexpected character
            ch => {
                gen_error!(sync self.grcx.borrow_mut(), self => {
                    E0003;
                    labels: [
                        LabelStyle::Primary =>
                            (self.source_id, self.new_span());
                            format!("did not expect `{}` here", ch)
                    ]
                });
                TokenType::Unexpected
            }
        };

        Ok(self.new_token(token_type))
    }

    /*
    pub fn scan(&mut self) -> CalResult<Token<'lex>> {
        // _T_O_D_O_: literals
        /*if ch == '0' {
            let peek = self.peek();
            if peek.is_some() {
                self.advance();
            }
            let radix = match peek {
                Some('x') => Radix::Hexadecimal,
                Some('o') => Radix::Octal,
                Some('b') => Radix::Binary,
                Some('E') | Some('e') => Radix::Decimal,
                None => Radix::Decimal,
                _ => {
                    let diagnostic = Diagnostic::new(
                        Span::new(self.start(), self.current() - self.start()),
                        self.buffer(),
                        self.source_name.clone(),
                        format!("invalid string base `{}`", peek.unwrap()),
                        4, // Invalid string base.
                    );
                    return Err(diagnostic.into());
                }
            };
            ch = self.advance();
        }*/
    }
    */
}
