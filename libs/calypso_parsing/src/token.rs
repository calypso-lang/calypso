use calypso_base::init_trie;
use calypso_base::span::{Span, Spanned};
use calypso_diagnostic::{
    diagnostic::Diagnostic,
    error::{ErrorKind, Result as CalResult, ResultExt},
};
use calypso_util::buffer::Buffer;
use radix_trie::Trie;

pub mod types;
pub use types::*;

pub mod helpers;
use helpers::*;

pub type Token<'lex> = Spanned<(TokenType, Lexeme<'lex>)>;

/*



fn is_valid_digit_for_radix(ch: char, radix: Radix) -> bool {
    match radix {
        Radix::Decimal => ch.is_ascii_digit(),
        Radix::Hexadecimal => ch.is_ascii_hexdigit(),
        Radix::Octal => ch >= '0' && ch <= '7',
        Radix::Binary => ch == '0' || ch == '1',
    }
}

fn is_valid_for_any_radix(ch: char) -> bool {
    ch.is_ascii_hexdigit()
}*/

type Lexeme<'lex> = &'lex [char];

#[derive(Debug, Clone, PartialEq)]
pub struct Lexer<'lex> {
    buf: Buffer<'lex>,
    source_name: String,
}

init_trie!(KEYWORD_TRIE: Keyword => {
    "is"     => KwIs,
    "false"  => KwFalse,
    "true"   => KwTrue,
    "if"     => KwIf,
    "else"   => KwElse,
    "loop"   => KwLoop,
    "while"  => KwWhile,
    "match"  => KwMatch,
    "ret"    => KwRet,
    "break"  => KwBreak,
    "fn"     => KwFn,
    "mod"    => KwMod,
    "use"    => KwUse,
    "import" => KwImport,
    "pub"    => KwPub,
    "let"    => KwLet,
    "undef"  => KwUndef,
    "null"   => KwNull,
    "del"    => KwDel,
    "as"     => KwAs
});

impl<'lex> Lexer<'lex> {
    pub fn new(source_name: String, source: &'lex [char]) -> Self {
        let buf = Buffer::new(source);
        Self { buf, source_name }
    }

    pub fn scan(&mut self) -> CalResult<Token<'lex>> {
        self.skip_whitespace()?;
        self.buf.current_to_start();

        if self.buf.is_at_end() {
            return Ok(self.new_token(TokenType::Eof));
        }

        // We've already checked if we're at the end (which is when it gives None), so
        // unwrapping should be safe here.
        let ch = self.buf.advance().unwrap();

        // Is valid character for identifier's first character
        if is_ident_start(ch) {
            return self.handle_identifier();
        }

        // TODO: literals

        use TokenType::*;

        let token_type = match ch {
            '<' if self.buf.match_next('<') => {
                if self.buf.match_next('=') {
                    ShlAssign
                } else {
                    Shl
                }
            }
            '<' if self.buf.match_next('=') => LessEqual,
            '<' => Less,

            '>' if self.buf.match_next('>') => {
                if self.buf.match_next('=') {
                    ShrAssign
                } else {
                    Shr
                }
            }
            '>' if self.buf.match_next('=') => GreaterEqual,
            '>' => Greater,

            '=' if self.buf.match_next('=') => BoolEqual,
            '=' => Equal,

            '!' if self.buf.match_next('=') => NotEqual,
            '!' => Bang,

            '|' if self.buf.match_next('|') => BoolOr,
            '|' if self.buf.match_next('=') => PipeAssign,
            '|' => Pipe,

            '&' if self.buf.match_next('&') => BoolAnd,
            '&' if self.buf.match_next('=') => AndAssign,
            '&' => And,

            '+' if self.buf.match_next('=') => PlusAssign,
            '+' => Plus,

            '-' if self.buf.match_next('=') => MinusAssign,
            '-' => Minus,

            '*' if self.buf.match_next('*') => {
                if self.buf.match_next('=') {
                    ExpAssign
                } else {
                    Exp
                }
            }
            '*' if self.buf.match_next('=') => StarAssign,
            '*' => Star,

            '/' if self.buf.match_next('=') => SlashAssign,
            '/' => Slash,

            '%' if self.buf.match_next('=') => RemAssign,
            '%' => Rem,

            '^' if self.buf.match_next('=') => CaretAssign,
            '^' => Caret,

            '~' => Tilde,

            '(' => LeftParen,
            ')' => RightParen,

            '{' => LeftBrace,
            '}' => RightBrace,

            '[' => LeftBracket,
            ']' => RightBracket,

            ',' => Comma,
            ';' => Semi,
            '.' => Dot,
            // `'_' => Under` is already taken care of by idents
            '#' if self.buf.match_next('!') => HashBang,
            '#' => Hash,

            // Unexpected character
            ch => {
                let diagnostic = Diagnostic::new(
                    Span::new(self.buf.start(), self.buf.current() - self.buf.start()),
                    self.buf.buffer(),
                    self.source_name.clone(),
                    format!("did not expect `{}` here.", ch),
                    3, // Unexpected character.
                );
                return Err(ErrorKind::Diagnostic(diagnostic).into());
            }
        };

        Ok(self.new_token(token_type))
    }
}

impl<'lex> Lexer<'lex> {
    fn skip_whitespace(&mut self) -> CalResult<()> {
        loop {
            self.handle_dangling_comment_ends()?;
            if (!self.handle_comment()
                && !self.handle_multiline_comment()?
                && !self.buf.match_next_if(is_whitespace))
                || self.buf.is_at_end()
            {
                break;
            }
        }
        Ok(())
    }

    fn handle_comment(&mut self) -> bool {
        // xx -> true true -> true
        // x/ -> false true -> true
        // /x -> true false -> true
        // // -> false false -> false
        if self.buf.peek() != Some('/') || self.buf.peek_next() != Some('/') {
            return false;
        }
        // A comment goes until the end of the line,
        // so gorge all the characters until we get to the newline
        // (or the end, when it automatically stops gorging).
        self.buf.gorge_while(|c| c != '\n');
        true
    }

    fn handle_multiline_comment(&mut self) -> CalResult<bool> {
        // xx -> true true -> true
        // x* -> true false -> true
        // /x -> false true -> true
        // /* -> false false -> false
        if self.buf.peek() != Some('/') || self.buf.peek_next() != Some('*') {
            return Ok(false);
        }
        self.buf.current_to_start();
        self.buf.advance();
        self.buf.advance();
        let mut nest = vec![Span::new(
            self.buf.start(),
            self.buf.current() - self.buf.start(),
        )];

        loop {
            let ch = self.buf.peek();
            if ch.is_none() {
                return Ok(false);
            }

            if ch == Some('/') && self.buf.peek_next() == Some('*') {
                // For error handling
                self.buf.current_to_start();
                self.buf.advance();
                self.buf.advance();
                nest.push(Span::new(
                    self.buf.start(),
                    self.buf.current() - self.buf.start(),
                ));
            } else if ch == Some('*') && self.buf.peek_next() == Some('/') {
                // For error handling
                self.buf.current_to_start();
                self.buf.advance();
                self.buf.advance();
                if nest.is_empty() {
                    let diagnostic = Diagnostic::new(
                        Span::new(self.buf.start(), self.buf.current() - self.buf.start()),
                        self.buf.buffer(),
                        self.source_name.clone(),
                        "this multi-line comment's end has no corresponding beginning".to_string(),
                        1, // No corresponding /* for */
                    );
                    return Err(ErrorKind::Diagnostic(diagnostic).into());
                }
                nest.pop();
            } else {
                self.buf.advance();
            }

            if nest.is_empty() && !self.buf.is_at_end() {
                break;
            }

            if self.buf.is_at_end() && !nest.is_empty() {
                let diagnostic = Diagnostic::new(
                    nest.pop().unwrap(),
                    self.buf.buffer(),
                    self.source_name.clone(),
                    "this multi-line comment's beginning has no corresponding end".to_string(),
                    2, // No corresponding */ for /*
                );
                return Err(ErrorKind::Diagnostic(diagnostic).into());
            }
        }

        Ok(true)
    }

    fn handle_dangling_comment_ends(&mut self) -> CalResult<()> {
        if self.buf.peek() == Some('*') && self.buf.peek_next() == Some('/') {
            // For error handling
            self.buf.current_to_start();
            self.buf.advance();
            self.buf.advance();
            let diagnostic = Diagnostic::new(
                Span::new(self.buf.start(), self.buf.current() - self.buf.start()),
                self.buf.buffer(),
                self.source_name.clone(),
                "this multi-line comment's end has no corresponding beginning".to_string(),
                1, // No corresponding /* for */
            );
            return Err(ErrorKind::Diagnostic(diagnostic).into());
        }
        Ok(())
    }

    fn new_token(&self, token_type: TokenType) -> Token<'lex> {
        let start = self.buf.start();
        let current = self.buf.current();
        Token::new(
            Span::new(start, current - start),
            (token_type, &self.buf.slice(start, current)),
        )
    }
}

impl<'lex> Lexer<'lex> {
    pub fn handle_identifier(&mut self) -> CalResult<Token<'lex>> {
        let mut token_type = TokenType::Ident;

        let ch = self.buf.peek();
        // `_` is not an ident on its own, but all other [A-Za-z]{1} idents are.
        if self.buf.last().unwrap() == '_'
            && (ch.is_none() || !is_ident_continue(ch.unwrap_or('\0')))
        {
            return Ok(self.new_token(TokenType::Under));
        }

        // Gorge while the character is a valid identifier character.
        self.buf.gorge_while(is_ident_continue);

        let keyword = KEYWORD_TRIE.get(
            &self
                .buf
                .slice(self.buf.start(), self.buf.current())
                .iter()
                .collect::<String>(),
        );

        if let Some(&keyword) = keyword {
            token_type = TokenType::Keyword(keyword);
        }

        Ok(self.new_token(token_type))
    }
}

/*
impl<'lex> Lexer<'lex> {
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



    fn escape_character(&mut self) -> Result<bool, ()> {
        if self.peek() == '\\' {
            self.advance();
            match self.peek() {
                'x' => {
                    for _ in 0..2 {
                        if is_valid_digit_for_radix(self.peek(), Radix::Hexadecimal)
                            && !self.is_at_end()
                        {
                            self.advance();
                        } else {
                            println!("Expected valid digit for hex escape sequence.");
                            return Err(());
                        }
                    }
                }
                'n' | 'r' | 't' | '\\' | '0' | '\'' | '"' => {
                    self.advance();
                }
                'u' => {
                    let mut digit_count = 0;
                    if !self.match_ch('{') {
                        println!("Expected an open brace, followed by a Unicode code point.");
                        return Err(());
                    }
                    while self.peek() != '}' && !self.is_at_end() {
                        if digit_count > 6
                            || !is_valid_digit_for_radix(self.peek(), Radix::Hexadecimal)
                        {
                            println!(
                                "Expected up to 6 hexadecimal digits for a Unicode code point."
                            );
                            return Err(());
                        }
                        self.advance();
                        digit_count += 1;
                    }

                    if self.is_at_end() {
                        println!("Unterminated Unicode escape sequence.");
                        return Err(());
                    }

                    // Closing bracket
                    self.advance();
                }

                _ => {
                    println!("Expected valid escape sequence.");
                    return Err(());
                }
            }
            return Ok(true);
        }

        // We don't care what sequence was found, just if there was one.
        Ok(false)
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

    fn char_literal(&mut self) -> Result<Token<'lex>, ()> {
        let mut chs_found = 0;
        while self.peek() != '\'' && !self.is_at_end() {
            if self.escape_character()? {
                chs_found += 1;
            } else {
                self.advance();
                chs_found += 1;
            }
        }

        if chs_found > 1 {
            println!("Expected a single character, found more.");
            return Err(());
        } else if chs_found == 0 {
            // Make this branch different as it has a different error
            let _ = 0;
            println!("Expected a single character, found none.");
            return Err(());
        }

        // Closing `'`
        self.advance();
        Ok(self.new_token(TokenType::CharLiteral))
    }
}
*/
