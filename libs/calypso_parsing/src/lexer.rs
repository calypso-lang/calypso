use calypso_base::init_trie;
use calypso_base::{
    span::{Span, Spanned},
    streams::{Stream, StringStream},
};
use calypso_diagnostic::{
    diagnostic::{DiagnosticBuilder, LabelStyle, Severity},
    error::Result as CalResult,
    gen_error, FileMgr,
};
use radix_trie::Trie;

use std::sync::Arc;

pub mod types;
pub use types::*;

mod helpers;
use helpers::*;

use std::ops::Deref;
use std::ops::DerefMut;

pub type Token<'lex> = Spanned<(TokenType, Lexeme<'lex>)>;
pub type Lexeme<'lex> = &'lex str;

#[derive(Debug, Clone)]
pub struct Lexer<'lex> {
    stream: StringStream<'lex>,
    source_id: usize,
    files: Arc<FileMgr>,
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

init_trie!(pub KEYWORD_TRIE: Keyword => {
    "is"     => KwIs,
    "false"  => KwFalse,
    "true"   => KwTrue,
    "if"     => KwIf,
    "else"   => KwElse,
    "for"    => KwFor,
    "in"     => KwIn,
    "loop"   => KwLoop,
    "while"  => KwWhile,
    "match"  => KwMatch,
    "ret"    => KwRet,
    "break"  => KwBreak,
    "fn"     => KwFn,
    "native" => KwNative,
    "mod"    => KwMod,
    "use"    => KwUse,
    "import" => KwImport,
    "pub"    => KwPub,
    "let"    => KwLet,
    "mut"    => KwMut,
    "undef"  => KwUndef,
    "null"   => KwNull,
    "del"    => KwDel,
    "as"     => KwAs
});

impl<'lex> Lexer<'lex> {
    pub fn new(source_id: usize, source: &'lex str, files: Arc<FileMgr>) -> Self {
        Self {
            stream: StringStream::new(source),
            source_id,
            files,
        }
    }

    pub fn scan(&mut self) -> CalResult<Token<'lex>> {
        self.skip_whitespace()?;
        gen_error!(self => {
            E0000;
            labels: [
                LabelStyle::Primary => (self.source_id, Span::new(0,0)); "not yet implemented",
            ],
        })
    }

    /*
    pub fn scan(&mut self) -> CalResult<Token<'lex>> {
        self.skip_whitespace()?;
        self.current_to_start();

        if self.is_at_end() {
            return Ok(self.new_token(TokenType::Eof));
        }

        // We've already checked if we're at the end (which is when it gives None), so
        // unwrapping should be safe here.
        let ch = self.advance().unwrap();

        // Is valid character for identifier's first character
        if is_ident_start(ch) {
            return self.handle_identifier();
        } else if ch == '\'' {
            return self.handle_char_literal();
        }

        // TODO: literals
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

        use TokenType::*;

        let token_type = match ch {
            '<' if self.match_next('<') => {
                if self.match_next('=') {
                    ShlAssign
                } else {
                    Shl
                }
            }
            '<' if self.match_next('=') => LessEqual,
            '<' => Less,

            '>' if self.match_next('>') => {
                if self.match_next('=') {
                    ShrAssign
                } else {
                    Shr
                }
            }
            '>' if self.match_next('=') => GreaterEqual,
            '>' => Greater,

            '=' if self.match_next('=') => BoolEqual,
            '=' => Equal,

            '!' if self.match_next('=') => NotEqual,
            '!' => Bang,

            '|' if self.match_next('|') => BoolOr,
            '|' if self.match_next('=') => PipeAssign,
            '|' => Pipe,

            '&' if self.match_next('&') => BoolAnd,
            '&' if self.match_next('=') => AndAssign,
            '&' => And,

            '+' if self.match_next('=') => PlusAssign,
            '+' => Plus,

            '-' if self.match_next('=') => MinusAssign,
            '-' => Minus,

            '*' if self.match_next('*') => {
                if self.match_next('=') {
                    ExpAssign
                } else {
                    Exp
                }
            }
            '*' if self.match_next('=') => StarAssign,
            '*' => Star,

            '/' if self.match_next('=') => SlashAssign,
            '/' => Slash,

            '%' if self.match_next('=') => RemAssign,
            '%' => Rem,

            '^' if self.match_next('=') => CaretAssign,
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

            '.' if self.match_next('.') => {
                if self.match_next('=') {
                    RangeClosed
                } else {
                    Range
                }
            }
            '.' => Dot,

            // `'_' => Under` is already taken care of by idents
            '#' if self.match_next('!') => HashBang,
            '#' => Hash,

            // Unexpected character
            ch => {
                let diagnostic = DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                    .diag(code!(E0003))
                    .label(
                        LabelStyle::Primary,
                        format!("did not expect `{}` here", ch),
                        self.new_span(),
                        self.source_id,
                    )
                    .build();
                return Err(diagnostic.into());
            }
        };

        Ok(self.new_token(token_type))
    }
    */
}

impl<'lex> Lexer<'lex> {
    fn skip_whitespace(&mut self) -> CalResult<()> {
        while !self.is_at_end()
            && (self.handle_comment()
                || self.handle_multiline_comment()?
                || self.next_if(is_whitespace).is_some())
        {
            //self.handle_dangling_comment_ends()?; TODO
        }
        Ok(())
    }

    fn handle_comment(&mut self) -> bool {
        // xx -> true true -> true
        // x/ -> false true -> true
        // /x -> true false -> true
        // // -> false false -> false
        if self.peek_cond(char_ne('/')) || self.peek2_cond(char_ne('/')) {
            return false;
        }
        // A comment goes until the end of the line,
        // so gorge all the characters until we get to the newline
        // (or the end, when it automatically stops gorging).
        self.gorge_while(|spanned, _| spanned.value_owned() != '\n');
        true
    }

    fn handle_multiline_comment(&mut self) -> CalResult<bool> {
        // xx -> 11 -> 1
        // x* -> 10 -> 1
        // /x -> 01 -> 1
        // /* -> 00 -> 0
        if self.peek_cond(char_ne('/')) || self.peek2_cond(char_ne('*')) {
            return Ok(false);
        }

        Ok(false)
    }
}

/*
impl<'lex> Lexer<'lex> {

    fn handle_multiline_comment(&mut self) -> CalResult<bool> {
        // xx -> true true -> true
        // x* -> true false -> true
        // /x -> false true -> true
        // /* -> false false -> false      */
        if self.peek() != Some('/') || self.peek_next() != Some('*') {
            return Ok(false);
        }
        self.current_to_start();
        self.advance();
        self.advance();
        let mut nest = vec![Span::new(self.start(), self.current() - self.start())];

        loop {
            let ch = self.peek();
            if ch.is_none() {
                return Ok(false);
            }

            if ch == Some('/') && self.peek_next() == Some('*') {
                // For error handling
                self.current_to_start();
                self.advance();
                self.advance();
                nest.push(Span::new(self.start(), self.current() - self.start()));
            } else if ch == Some('*') && self.peek_next() == Some('/') {
                // For error handling
                self.current_to_start();
                self.advance();
                self.advance();
                if nest.is_empty() {
                    let diagnostic =
                        DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                            .diag(code!(E0001))
                            .label(
                                LabelStyle::Primary,
                                "this multi-line comment's end has no corresponding beginning",
                                self.new_span(),
                                self.source_id,
                            )
                            .build();
                    return Err(diagnostic.into());
                }
                nest.pop();
            } else {
                self.advance();
            }

            if nest.is_empty() && !self.is_at_end() {
                break;
            }

            if self.is_at_end() && !nest.is_empty() {
                let diagnostic = DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                    .diag(code!(E0002))
                    .label(
                        LabelStyle::Primary,
                        "this multi-line comment's beginning has no corresponding end",
                        nest.pop().unwrap(),
                        self.source_id,
                    )
                    .build();
                return Err(diagnostic.into());
            }
        }

        Ok(true)
    }

    fn handle_dangling_comment_ends(&mut self) -> CalResult<()> {
        if self.peek() == Some('*') && self.peek_next() == Some('/') {
            // For error handling
            self.current_to_start();
            self.advance();
            self.advance();
            let diagnostic = DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                .diag(code!(E0001))
                .label(
                    LabelStyle::Primary,
                    "this multi-line comment's end has no corresponding beginning",
                    self.new_span(),
                    self.source_id,
                )
                .build();
            return Err(diagnostic.into());
        }
        Ok(())
    }

    fn new_token(&self, token_type: TokenType) -> Token<'lex> {
        let start = self.start();
        let current = self.current();
        Token::new(self.new_span(), (token_type, self.slice(start, current)))
    }

    fn new_span(&self) -> Span {
        let start = self.start();
        let current = self.current();
        Span::new(start, current - start)
    }
}

impl<'lex> Lexer<'lex> {
    pub fn handle_identifier(&mut self) -> CalResult<Token<'lex>> {
        let mut token_type = TokenType::Ident;

        let ch = self.peek();
        // `_` is not an ident on its own, but all other [A-Za-z]{1} idents are.
        if self.last().unwrap() == '_' && (ch.is_none() || !is_ident_continue(ch.unwrap_or('\0'))) {
            return Ok(self.new_token(TokenType::Under));
        }

        // Gorge while the character is a valid identifier character.
        self.gorge_while(|ch, _| is_ident_continue(ch));

        let keyword = KEYWORD_TRIE.get(
            &self
                .buf
                .slice(self.start(), self.current())
                .iter()
                .collect::<String>(),
        );

        if let Some(&keyword) = keyword {
            token_type = TokenType::Keyword(keyword);
        }

        Ok(self.new_token(token_type))
    }

    fn handle_escape_character(&mut self) -> CalResult<bool> {
        let start = self.current();
        if self.peek() == Some('\\') {
            self.current_to_start();
            self.advance();
            match self.peek() {
                Some('x') => {
                    self.advance();
                    self.current_to_start();
                    for i in 1..=2 {
                        let mut ch = self.peek();
                        if is_whitespace(ch.unwrap_or('\0')) {
                            ch = None;
                        }
                        if ch.is_none() {
                            let diagnostic = if i == 1 {
                                DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                    .diag(code!(E0004))
                                    .label(
                                        LabelStyle::Primary,
                                        "expected two hexadecimal digits here",
                                        self.new_span(),
                                        self.source_id,
                                    )
                                    .build()
                            } else if i == 2 {
                                DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                    .diag(code!(E0009))
                                    .label(
                                        LabelStyle::Primary,
                                        "found only one digit here",
                                        self.new_span(),
                                        self.source_id,
                                    )
                                    .note(format!(
                                        "perhaps you meant to use `\\x0{}`?",
                                        self.last().unwrap()
                                    ))
                                    .build()
                            } else {
                                return Ok(true);
                            };
                            return Err(diagnostic.into());
                        }
                        let ch = ch.unwrap();

                        if ch.is_ascii_hexdigit() {
                            self.advance();
                        } else {
                            self.set_start(start + 1 + i);
                            let diagnostic =
                                DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                    .diag(code!(E0005, ch = ch))
                                    .label(
                                        LabelStyle::Primary,
                                        "found an invalid digit here",
                                        self.new_span(),
                                        self.source_id,
                                    )
                                    .build();
                            return Err(diagnostic.into());
                        }
                    }
                }
                Some('n') | Some('r') | Some('t') | Some('\\') | Some('0') | Some('\'')
                | Some('"') => {
                    self.advance();
                }
                Some('u') => {
                    self.advance();
                    self.current_to_start();
                    match self.peek() {
                        Some(ch) if is_whitespace(ch) => {
                            let diagnostic =
                                DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                    .diag(code!(E0012))
                                    .label(
                                        LabelStyle::Primary,
                                        "this should be an opening curly bracket",
                                        self.new_span(),
                                        self.source_id,
                                    )
                                    .build();
                            return Err(diagnostic.into());
                        }
                        None => {
                            let diagnostic =
                                DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                    .diag(code!(E0011))
                                    .label(
                                        LabelStyle::Primary,
                                        "this should be an opening curly bracket",
                                        self.new_span(),
                                        self.source_id,
                                    )
                                    .build();
                            return Err(diagnostic.into());
                        }
                        _ => (),
                    }
                    if !self.match_next('{') {
                        self.advance();
                        let diagnostic =
                            DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                .diag(code!(E0010, ch = self.last().unwrap()))
                                .label(
                                    LabelStyle::Primary,
                                    "this should be an opening curly bracket",
                                    self.new_span(),
                                    self.source_id,
                                )
                                .build();
                        return Err(diagnostic.into());
                    }

                    let mut count = 0;
                    while self.peek() != Some('}') && !self.is_at_end() {
                        self.current_to_start();
                        let ch = self.peek().unwrap();
                        if count == 6 {
                            break;
                        } else if ch.is_whitespace() {
                            let diagnostic =
                                DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                    .diag(code!(E0018))
                                    .label(
                                        LabelStyle::Primary,
                                        "expected a hexadecimal digit here",
                                        self.new_span(),
                                        self.source_id,
                                    )
                                    .build();
                            return Err(diagnostic.into());
                        } else if !ch.is_ascii_hexdigit() {
                            let diagnostic =
                                DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                    .diag(code!(E0014, ch = ch))
                                    .label(
                                        LabelStyle::Primary,
                                        "found an invalid digit here. perhaps you meant to have a `}` here?",
                                        self.new_span(),
                                        self.source_id,
                                    )
                                    .build();
                            return Err(diagnostic.into());
                        }
                        self.advance();
                        count += 1;
                    }

                    if count == 0 {
                        let diagnostic =
                            DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                .diag(code!(E0019))
                                .label(
                                    LabelStyle::Primary,
                                    "expected at least one hex digit here",
                                    self.new_span(),
                                    self.source_id,
                                )
                                .note("if you wanted a null byte, you can use `\\u{0}` or `\\0`")
                                .build();
                        return Err(diagnostic.into());
                    }
                    self.current_to_start();

                    if self.is_at_end() {
                        let diagnostic =
                            DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                .diag(code!(E0015))
                                .label(
                                    LabelStyle::Primary,
                                    "expected a closing curly bracket here",
                                    self.new_span(),
                                    self.source_id,
                                )
                                .build();
                        return Err(diagnostic.into());
                    }

                    let ch = self.peek().unwrap();
                    if is_whitespace(ch) {
                        self.current_to_start();
                        let diagnostic =
                            DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                .diag(code!(E0017))
                                .label(
                                    LabelStyle::Primary,
                                    "expected a closing curly bracket here",
                                    self.new_span(),
                                    self.source_id,
                                )
                                .build();
                        return Err(diagnostic.into());
                    } else if !self.match_next('}') {
                        let diagnostic =
                            DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                .diag(code!(E0016, ch = ch))
                                .label(
                                    LabelStyle::Primary,
                                    "expected a closing curly bracket here",
                                    self.new_span(),
                                    self.source_id,
                                )
                                .build();
                        return Err(diagnostic.into());
                    }
                }
                Some(ch) => {
                    if is_whitespace(ch) {
                        let diagnostic =
                            DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                                .diag(code!(E0008))
                                .label(
                                    LabelStyle::Primary,
                                    "expected an escape sequence here",
                                    self.new_span(),
                                    self.source_id,
                                )
                                .build();
                        return Err(diagnostic.into());
                    }
                    self.advance();
                    let diagnostic =
                        DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                            .diag(code!(E0006, ch = ch))
                            .label(
                                LabelStyle::Primary,
                                "this escape sequence is unknown",
                                self.new_span(),
                                self.source_id,
                            )
                            .build();
                    return Err(diagnostic.into());
                }
                None => {
                    let diagnostic =
                        DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                            .diag(code!(E0007))
                            .label(
                                LabelStyle::Primary,
                                "expected an escape sequence here",
                                self.new_span(),
                                self.source_id,
                            )
                            .build();
                    return Err(diagnostic.into());
                }
            };
            self.set_start(start);
            return Ok(true);
        }

        // We don't care *what* sequence was found, just if there was one.
        Ok(false)
    }

    fn handle_char_literal(&mut self) -> CalResult<Token<'lex>> {
        let mut chs_found = 0;
        while self.peek() != Some('\'') && !self.is_at_end() {
            if self.handle_escape_character()? {
                chs_found += 1;
            } else if is_valid_for_char_literal(self.peek().unwrap()) {
                self.advance();
                chs_found += 1;
            } else {
                let diagnostic = DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                    .diag(code!(E0020))
                    .label(
                        LabelStyle::Primary,
                        "the character after this one is invalid here; it must be escaped",
                        self.new_span(),
                        self.source_id,
                    )
                    .build();
                return Err(diagnostic.into());
            }
        }

        if chs_found > 1 {
            let start = self.start();
            self.set_start(start + 2);
            self.advance();
            let diagnostic = DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                .diag(code!(E0021))
                .label(
                    LabelStyle::Primary,
                    "expected just a `'` here",
                    self.new_span(),
                    self.source_id,
                )
                .build();
            return Err(diagnostic.into());
        } else if chs_found == 0 {
            let diagnostic = DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                .diag(code!(E0022))
                .label(
                    LabelStyle::Primary,
                    "expected at least one character here",
                    self.new_span(),
                    self.source_id,
                )
                .build();
            return Err(diagnostic.into());
        }

        if !self.match_next('\'') {
            self.current_to_start();
            self.advance();
            let diagnostic = DiagnosticBuilder::new(Severity::Error, Arc::clone(&self.files))
                .diag(code!(E0023))
                .label(
                    LabelStyle::Primary,
                    "expected a single quote here",
                    self.new_span(),
                    self.source_id,
                )
                .build();
            return Err(diagnostic.into());
        }

        Ok(self.new_token(TokenType::CharLiteral))
    }
}
*/

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
