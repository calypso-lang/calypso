use calypso_base::init_trie;
use calypso_base::span::{Span, Spanned};
use calypso_base::static_list as sl;
use calypso_util::buffer::Buffer;
use radix_trie::Trie;

sl!(WHITESPACE: char = [
    '\t',       // Horizontal tab
    '\n',       // Line feed
    '\u{000B}', // Vertical tab
    '\u{000C}', // Form feed
    '\r',       // Carriage return
    ' ',        // Space
    '\u{0085}', // Next line
    '\u{200E}', // Left-to-right mark
    '\u{200F}', // Right-to-left mark
    '\u{2028}', // Line separator
    '\u{2029}', // Paragraph separator
]);

fn is_whitespace(ch: char) -> bool {
    WHITESPACE.contains(&ch)
}

/*fn is_ident_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_ident_continue(ch: char) -> bool {
    is_ident_start(ch) || ch.is_ascii_digit()
}

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

init_trie!(TOKENS_TRIE: TokenType => {

    // Operators

    // Booleans
    "<" => Less, "<=" => LessEqual,
    "==" => BoolEqual, "!=" => NotEqual,
    ">" => Greater, ">=" => GreaterEqual,
    "||" => BoolOr, "&&" => BoolAnd,
    "!" => Bang,
    // Numbers
    "+" => Plus, "+=" => PlusAssign,
    "-" => Minus, "-=" => MinusAssign,
    "*" => Star, "*=" => StarAssign,
    "/" => Slash, "/=" => SlashAssign,
    "**" => Exp, "**=" => ExpAssign,
    "%" => Rem, "%=" => RemAssign,
    // Bitwise
    ">>" => Shr, ">>=" => ShrAssign,
    "<<" => Shl, "<<=" => ShlAssign,
    "|" => Pipe, "|=" => PipeAssign,
    "&" => And, "&=" => AndAssign,
    "^" => Caret, "^=" => CaretAssign,
    "~" => Tilde,
    // Variables
    "=" => Equal,

    // Other Characters

    // Grouping and Parentheses
    "(" => LeftParen, ")" => RightParen,
    "{" => LeftBrace, "}" => RightBrace,
    "[" => LeftBracket, "]" => RightBracket,
    "," => Comma, ";" => Semi,
    "." => Dot, "_" => Under
});

impl<'lex> Lexer<'lex> {
    pub fn new(buf: Buffer<'lex>) -> Self {
        Self { buf }
    }

    pub fn scan(&mut self) -> Result<Token<'lex>, ()> {
        self.skip_whitespace();

        self.buf.set_start(self.buf.current());

        if self.buf.is_at_end() {
            return Ok(self.new_token(TokenType::Eof));
        }

        // let ch = self.buf.advance();

        Err(())
    }
}

impl<'lex> Lexer<'lex> {
    fn skip_whitespace(&mut self) {
        loop {
            let ch = self.buf.peek();
            match ch {
                // Comments not currently implemented
                Some(ch) if is_whitespace(ch) => {
                    self.buf.advance();
                }
                _ => break,
            }
        }
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

    fn identifier(&mut self) -> Result<Token<'lex>, ()> {
        let ch = self.peek();
        if is_ident_start(ch) {
            self.advance();
            if ch == '_' && !is_ident_continue(self.peek()) {
                return Ok(self.new_token(TokenType::Under));
            }
        }

        while is_ident_continue(self.peek()) {
            self.advance();
        }

        let mut token_type = TokenType::Ident;

        let token_type_trie = KEYWORD_TRIE.get(
            &self.buffer[self.start..self.current]
                .to_vec()
                .iter()
                .collect::<String>(),
        );

        if let Some(token_type_trie) = token_type_trie {
            token_type = TokenType::Keyword(*token_type_trie);
        }

        Ok(self.new_token(token_type))
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

pub mod types;
pub use types::*;

pub type Token<'lex> = Spanned<(TokenType, Lexeme<'lex>)>;
