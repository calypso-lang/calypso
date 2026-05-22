//! This lexer is inspired by [Gleam's][1] quite simple lexer
//! implementation, licensed Apache-2.0.
//!
//! [1]: https://github.com/gleam-lang/gleam/blob/0f47830d2e9fe3ec58356575658fc6f8f07dd728/compiler-core/src/parse/lexer.rs

// TODO: try to synchronize from some lexical errors? numerals would
// be easy to sync from.

use crate::{
    ctxt::GlobalCtxt,
    symbol::{Symbol, kw::Keyword},
    syntax::span::Span,
};

use super::{
    error::{LexicalError, LexicalErrorKind},
    token::{IntegerWidth, Numeral, Radix, Suffix, Token},
};

#[derive(Debug)]
pub struct Lexer<'gcx, T: Iterator<Item = (u32, char)>> {
    gcx: &'gcx GlobalCtxt,
    file: Symbol,
    chars: T,
    pending: Vec<SpanTok>,
    eof: bool,
    chr0: Option<char>,
    chr1: Option<char>,
    loc0: u32,
    loc1: u32,
}

pub type SpanTok = (Span, Token);

pub fn tokens<'gcx, 'src: 'gcx>(
    gcx: &'gcx GlobalCtxt,
    source: &'src str,
    file: Symbol,
) -> impl Iterator<Item = SpanTok> + 'gcx {
    let chars = source.char_indices().map(|(i, c)| (i as u32, c));
    Lexer::new(gcx, chars, file)
}

impl<'gcx, T> Lexer<'gcx, T>
where
    T: Iterator<Item = (u32, char)>,
{
    pub fn new(gcx: &'gcx GlobalCtxt, input: T, file: Symbol) -> Self {
        let mut lex = Lexer {
            gcx,
            file,
            chars: input,
            pending: vec![],
            eof: false,
            chr0: None,
            chr1: None,
            loc0: 0,
            loc1: 0,
        };
        let _ = lex.advance();
        let _ = lex.advance();

        // Check whether the first character is a UTF-8 byte order mark, and if so, consume it.
        if lex.chr0 == Some('\u{feff}') {
            let _ = lex.advance();
        }

        lex
    }

    fn consume_normal(&mut self) {
        if let Some(c) = self.chr0 {
            let mut check_for_minus = false;
            if Self::is_identifier_start(c) {
                self.consume_identifier_or_kw();
            } else if Self::is_numeral_start(c, self.chr1) {
                check_for_minus = true;
                self.consume_numeral();
            } else {
                self.consume_simple(c);
            }
            if check_for_minus {
                // We want to lex `1-1` as `1 - 1`.
                if self.chr0 == Some('-') && Self::is_numeral_start('-', self.chr1) {
                    self.eat_single_char(Token::Minus);
                }
            }
        } else {
            // EOF.
            let tok_pos = self.pos();
            self.emit(tok_pos, tok_pos, Token::Eof);
            self.eof = true;
        }
    }

    #[allow(clippy::too_many_lines)]
    fn consume_simple(&mut self, c: char) {
        match c {
            '(' => self.eat_single_char(Token::LeftParen),
            ')' => self.eat_single_char(Token::RightParen),
            '[' => self.eat_single_char(Token::LeftSquare),
            ']' => self.eat_single_char(Token::RightSquare),
            '{' => self.eat_single_char(Token::LeftCurly),
            '}' => self.eat_single_char(Token::RightCurly),
            '+' => self.eat_single_char(Token::Plus),
            '%' => self.eat_single_char(Token::Percent),
            '^' => self.eat_single_char(Token::Xor),
            ',' => self.eat_single_char(Token::Comma),
            '.' => self.eat_single_char(Token::Dot),
            ':' => self.eat_single_char(Token::Colon),
            ';' => self.eat_single_char(Token::Semi),
            '/' => {
                let tok_start = self.pos();
                let _ = self.advance();
                if let Some('/') = self.chr0 {
                    while !matches!(self.chr0, Some('\n') | None) {
                        let _ = self.advance();
                    }
                    // Absorb newline
                    if self.chr0 == Some('\n') {
                        let _ = self.advance();
                    }
                    let tok_end = self.pos();
                    // Emit comments as newlines as they can break lines
                    self.emit(tok_start, tok_end, Token::Nl);
                } else {
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::Slash);
                }
            }
            '>' => {
                let tok_start = self.pos();
                let _ = self.advance();
                match self.chr0 {
                    Some('>') => {
                        let _ = self.advance();
                        let tok_end = self.pos();
                        self.emit(tok_start, tok_end, Token::Shr);
                    }
                    Some('=') => {
                        let _ = self.advance();
                        let tok_end = self.pos();
                        self.emit(tok_start, tok_end, Token::GtEq);
                    }
                    _ => {
                        let tok_end = self.pos();
                        self.emit(tok_start, tok_end, Token::Gt);
                    }
                }
            }
            '<' => {
                let tok_start = self.pos();
                let _ = self.advance();
                match self.chr0 {
                    Some('<') => {
                        let _ = self.advance();
                        let tok_end = self.pos();
                        self.emit(tok_start, tok_end, Token::Shl);
                    }
                    Some('=') => {
                        let _ = self.advance();
                        let tok_end = self.pos();
                        self.emit(tok_start, tok_end, Token::LtEq);
                    }
                    _ => {
                        let tok_end = self.pos();
                        self.emit(tok_start, tok_end, Token::Lt);
                    }
                }
            }
            '=' => {
                let tok_start = self.pos();
                let _ = self.advance();
                if let Some('=') = self.chr0 {
                    let _ = self.advance();
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::EqEq);
                } else {
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::Eq);
                }
            }
            '&' => {
                let tok_start = self.pos();
                let _ = self.advance();
                if let Some('&') = self.chr0 {
                    let _ = self.advance();
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::BoolAnd);
                } else {
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::And);
                }
            }
            '|' => {
                let tok_start = self.pos();
                let _ = self.advance();
                if let Some('|') = self.chr0 {
                    let _ = self.advance();
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::BoolOr);
                } else {
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::Or);
                }
            }
            '!' => {
                let tok_start = self.pos();
                let _ = self.advance();
                if let Some('=') = self.chr0 {
                    let _ = self.advance();
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::NotEq);
                } else {
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::Not);
                }
            }
            '*' => {
                let tok_start = self.pos();
                let _ = self.advance();
                if let Some('*') = self.chr0 {
                    let _ = self.advance();
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::StarStar);
                } else {
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::Star);
                }
            }
            '-' => {
                let tok_start = self.pos();
                let _ = self.advance();
                if let Some('>') = self.chr0 {
                    let _ = self.advance();
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::Arrow);
                } else {
                    let tok_end = self.pos();
                    self.emit(tok_start, tok_end, Token::Minus);
                }
            }
            '\n' | '\r' | ' ' | '\t' | '\x0C' => {
                let tok_start = self.pos();
                let _ = self.advance();
                let tok_end = self.pos();
                if c == '\n' {
                    self.emit(tok_start, tok_end, Token::Nl);
                }
            }
            _ => {
                let start_pos = self.pos();
                let _ = self.advance();
                let end_pos = self.pos();
                self.report_error(LexicalError {
                    kind: LexicalErrorKind::UnexpectedToken,
                    location: Span::new(start_pos, end_pos, self.file),
                });
                self.emit(start_pos, end_pos, Token::Error);
            }
        }
    }

    fn report_error(&self, e: LexicalError) {
        let report = e.into_report(self.gcx);
        let mut drcx = self.gcx.diag.borrow_mut();
        drcx.report_syncd(report);
    }

    fn consume_identifier_or_kw(&mut self) {
        let start_pos = self.pos();
        // TODO(@ThePuzzlemaker): consider ecow/smol_str for this?
        let mut name = String::new();
        name.push(self.advance().expect("consume_identifier_or_kw"));

        while self.chr0.is_some_and(Self::is_identifier_continue) {
            name.push(self.advance().expect("consume_identifier_or_kw continue"));
        }

        let end_pos = self.pos();

        let symbol = Symbol::intern(&name);
        if let Ok(kw) = Keyword::try_from(symbol) {
            self.emit(start_pos, end_pos, Token::Keyword(kw));
        } else {
            self.emit(start_pos, end_pos, Token::Ident(symbol));
        }
    }

    fn consume_numeral(&mut self) {
        let start_pos = self.pos();
        let mut radix = Radix::None;
        let mut suffix = Suffix::None;
        let mut had_error = false;
        'end: {
            if self.chr0 == Some('-') {
                let _ = self.advance();
            }

            debug_assert!(self.chr0.is_some());
            if self.chr0 == Some('0') {
                let narrow_pos = self.pos();
                let _ = self.advance();
                match self.chr0 {
                    Some('0'..='9' | '_') => {
                        let end_pos = self.pos();
                        self.report_error(LexicalError {
                            kind: LexicalErrorKind::ZeroPrefixedNumeral,
                            location: Span::new(start_pos, end_pos, self.file),
                        });
                        had_error = true;
                    }
                    Some('x') => {
                        radix = Radix::Hexadecimal;
                        let _ = self.advance();
                    }
                    Some('d') => {
                        radix = Radix::Decimal;
                        let _ = self.advance();
                    }
                    Some('o') => {
                        radix = Radix::Octal;
                        let _ = self.advance();
                    }
                    Some('b') => {
                        radix = Radix::Binary;
                        let _ = self.advance();
                    }
                    Some(_) => {
                        let _ = self.advance();
                        let end_pos = self.pos();
                        self.report_error(LexicalError {
                            kind: LexicalErrorKind::InvalidNumeralRadixSpecifier,
                            location: Span::new(narrow_pos, end_pos, self.file),
                        });
                        had_error = true;
                    }
                    None => break 'end,
                }
            }

            loop {
                match (radix, self.chr0) {
                    (_, Some('_'))
                    | (Radix::Decimal | Radix::None, Some('0'..='9'))
                    | (Radix::Hexadecimal, Some('0'..='9' | 'a'..='f' | 'A'..='F'))
                    | (Radix::Octal, Some('0'..='7'))
                    | (Radix::Binary, Some('0' | '1')) => {
                        let _ = self.advance();
                    }
                    (_, Some('i' | 'u')) => {
                        if let Some(s) = self.parse_suffix() {
                            suffix = s;
                        } else {
                            had_error = true;
                        }
                    }
                    (_, Some('0'..='9' | 'a'..='z' | 'A'..='Z')) => {
                        let narrow_pos = self.pos();
                        let _ = self.advance();
                        let end_pos = self.pos();
                        self.report_error(LexicalError {
                            kind: LexicalErrorKind::InvalidNumeralRadixCharacter,
                            location: Span::new(narrow_pos, end_pos, self.file),
                        });
                        had_error = true;
                    }
                    (_, Some(_) | None) => break 'end,
                }
            }
        }
        let end_pos = self.pos();

        if had_error {
            self.emit(start_pos, end_pos, Token::Error);
        } else {
            self.emit(
                start_pos,
                end_pos,
                Token::Numeral(Numeral::Integer { radix, suffix }),
            );
        }
    }

    fn parse_suffix(&mut self) -> Option<Suffix> {
        let ctor = match self.chr0 {
            Some('i') => Suffix::Signed,
            Some('u') => Suffix::Unsigned,
            _ => unreachable!(),
        };
        let _ = self.advance();
        let width = self.parse_width()?;
        Some(ctor(width))
    }

    fn parse_width(&mut self) -> Option<IntegerWidth> {
        let start_pos = self.pos();
        let mut w = String::new();
        // We also consume ASCII letters to hopefully be more consistent.
        while let Some(width_ch @ ('0'..='9' | 'a'..='z' | 'A'..='Z')) = self.chr0 {
            let _ = self.advance();
            w.push(width_ch);
        }

        Some(if w.is_empty() {
            let end_pos = self.pos();
            self.report_error(LexicalError {
                kind: LexicalErrorKind::InvalidNumeralWidth,
                location: Span::new(start_pos, end_pos, self.file),
            });
            return None;
        } else if w == "ptr" {
            IntegerWidth::Ptr
        } else {
            match w.parse::<u8>() {
                Ok(8) => IntegerWidth::I8,
                Ok(16) => IntegerWidth::I16,
                Ok(32) => IntegerWidth::I32,
                Ok(64) => IntegerWidth::I64,
                _ => {
                    let end_pos = self.pos();
                    self.report_error(LexicalError {
                        kind: LexicalErrorKind::InvalidNumeralWidth,
                        location: Span::new(start_pos, end_pos, self.file),
                    });
                    return None;
                }
            }
        })
    }

    fn is_identifier_start(c: char) -> bool {
        unicode_ident::is_xid_start(c) || c == '_'
    }

    fn is_identifier_continue(c: char) -> bool {
        unicode_ident::is_xid_continue(c)
    }

    fn is_numeral_start(c: char, p: Option<char>) -> bool {
        c.is_ascii_digit() || (c == '-' && p.is_some_and(|c| c.is_ascii_digit()))
    }

    fn eat_single_char(&mut self, tok: Token) {
        let tok_start = self.pos();
        let _ = self.advance().expect("eat_single_char");
        let tok_end = self.pos();
        self.emit(tok_start, tok_end, tok);
    }

    fn pos(&self) -> u32 {
        self.loc0
    }

    #[must_use]
    fn advance(&mut self) -> Option<char> {
        let c = self.chr0;
        let next = if let Some((loc, c)) = self.chars.next() {
            self.loc0 = self.loc1;
            self.loc1 = loc;
            Some(c)
        } else {
            // EOF needs a single advance
            self.loc0 = self.loc1;
            self.loc1 += 1;
            None
        };
        self.chr0 = self.chr1;
        self.chr1 = next;
        c
    }

    fn emit(&mut self, start: u32, end: u32, tok: Token) {
        self.pending.push((Span::new(start, end, self.file), tok));
    }
}

impl<T> Iterator for Lexer<'_, T>
where
    T: Iterator<Item = (u32, char)>,
{
    type Item = SpanTok;

    fn next(&mut self) -> Option<Self::Item> {
        // Keep processing until there's a pending char.
        while !self.eof && self.pending.is_empty() {
            self.consume_normal();
        }

        if self.pending.is_empty() {
            None
        } else {
            Some(self.pending.remove(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::symbol::kw;
    use crate::syntax::token::Token;
    use pretty_assertions::assert_eq;

    fn file() -> Symbol {
        Symbol::intern_static("<test>")
    }

    fn span(lo: u32, hi: u32) -> Span {
        Span::new(lo, hi, file())
    }

    fn assert_source(test_source: &[(&str, Token)]) {
        let mut sep = "";
        let mut source = String::new();
        let mut expected = vec![];
        for &(s, tok) in test_source {
            source.push_str(sep);
            let ix_start = source.len();
            source.push_str(s);
            let ix_end = source.len();
            expected.push((span(ix_start as u32, ix_end as u32), tok));
            sep = " ";
        }

        let gcx = GlobalCtxt::default();
        let file = file();
        gcx.source_cache.borrow_mut().add(file, source.clone());

        assert_eq!(
            super::tokens(&gcx, &source, file).collect::<Vec<_>>(),
            expected
        );
    }

    // TODO: fix these tests
    // fn assert_err(source: &str, error: LexicalErrorKind) {
    //     assert_eq!(tokens(&source).next().unwrap().unwrap_err().kind, error);
    // }

    fn intnum(radix: Radix, suffix: Suffix) -> Token {
        Token::Numeral(Numeral::Integer { radix, suffix })
    }

    fn suffix(signed: bool, width: IntegerWidth) -> Suffix {
        if signed {
            Suffix::Signed(width)
        } else {
            Suffix::Unsigned(width)
        }
    }

    // #[test]
    // fn radix_spec_error() {
    //     assert_err("0r55", LexicalErrorKind::InvalidNumeralRadixSpecifier);
    // }

    // #[test]
    // fn radix_char_error() {
    //     let err = LexicalErrorKind::InvalidNumeralRadixCharacter;
    //     assert_err("0xG", err);
    //     assert_err("0dA", err);
    //     assert_err("0b2", err);
    //     assert_err("0o8", err);
    // }

    // #[test]
    // fn numeral_width_error() {
    //     let err = LexicalErrorKind::InvalidNumeralWidth;
    //     assert_err("1ughhh", err);
    //     assert_err("1u55", err);
    //     assert_err("1i55", err);
    //     assert_err("1iguess", err);
    // }

    // #[test]
    // fn zero_prefixed_numeral_error() {
    //     assert_err("0755", LexicalErrorKind::ZeroPrefixedNumeral);
    // }

    #[test]
    fn numerals() {
        let test_source = [
            ("-1", intnum(Radix::None, Suffix::None)),
            ("1", intnum(Radix::None, Suffix::None)),
            ("1u8", intnum(Radix::None, suffix(false, IntegerWidth::I8))),
            (
                "1u16",
                intnum(Radix::None, suffix(false, IntegerWidth::I16)),
            ),
            (
                "1u32",
                intnum(Radix::None, suffix(false, IntegerWidth::I32)),
            ),
            (
                "1u64",
                intnum(Radix::None, suffix(false, IntegerWidth::I64)),
            ),
            (
                "1uptr",
                intnum(Radix::None, suffix(false, IntegerWidth::Ptr)),
            ),
            ("-1i8", intnum(Radix::None, suffix(true, IntegerWidth::I8))),
            (
                "-1i16",
                intnum(Radix::None, suffix(true, IntegerWidth::I16)),
            ),
            (
                "-1i32",
                intnum(Radix::None, suffix(true, IntegerWidth::I32)),
            ),
            (
                "-1i64",
                intnum(Radix::None, suffix(true, IntegerWidth::I64)),
            ),
            (
                "-1iptr",
                intnum(Radix::None, suffix(true, IntegerWidth::Ptr)),
            ),
            ("1234567890", intnum(Radix::None, Suffix::None)),
            (
                "0x0123456789ABCDEFabcdef",
                intnum(Radix::Hexadecimal, Suffix::None),
            ),
            ("0b01", intnum(Radix::Binary, Suffix::None)),
            ("0d0123456789", intnum(Radix::Decimal, Suffix::None)),
            ("0o01234567", intnum(Radix::Octal, Suffix::None)),
            (
                "0d15i32",
                intnum(Radix::Decimal, Suffix::Signed(IntegerWidth::I32)),
            ),
            (
                "-0d1i32",
                intnum(Radix::Decimal, Suffix::Signed(IntegerWidth::I32)),
            ),
        ];

        assert_source(&test_source);
    }

    #[test]
    fn idents() {
        let test_source = {
            #[allow(clippy::enum_glob_use)]
            use Token::*;
            [
                ("hello", Ident(Symbol::intern_static("hello"))),
                ("hello_World", Ident(Symbol::intern_static("hello_World"))),
                ("hello0world1", Ident(Symbol::intern_static("hello0world1"))),
                ("_helloWorld", Ident(Symbol::intern_static("_helloWorld"))),
                ("_", Ident(Symbol::intern_static("_"))),
            ]
        };

        assert_source(&test_source);
    }

    #[test]
    fn keywords() {
        let test_source = {
            #[allow(clippy::enum_glob_use)]
            use Token::*;
            [
                ("true", Keyword(kw::Keyword::True)),
                ("false", Keyword(kw::Keyword::False)),
                ("let", Keyword(kw::Keyword::Let)),
                ("do", Keyword(kw::Keyword::Do)),
                ("end", Keyword(kw::Keyword::End)),
                ("in", Keyword(kw::Keyword::In)),
                ("fn", Keyword(kw::Keyword::Fn)),
                ("if", Keyword(kw::Keyword::If)),
                ("then", Keyword(kw::Keyword::Then)),
                ("else", Keyword(kw::Keyword::Else)),
                ("pub", Keyword(kw::Keyword::Pub)),
            ]
        };

        assert_source(&test_source);
    }

    #[test]
    fn basic_tokens() {
        let test_source = {
            #[allow(clippy::enum_glob_use)]
            use Token::*;
            [
                ("(", LeftParen),
                (")", RightParen),
                ("[", LeftSquare),
                ("]", RightSquare),
                ("{", LeftCurly),
                ("}", RightCurly),
                ("+", Plus),
                ("-", Minus),
                ("*", Star),
                ("%", Percent),
                ("**", StarStar),
                (">>", Shr),
                ("<<", Shl),
                ("&", And),
                ("|", Or),
                ("^", Xor),
                ("==", EqEq),
                ("!=", NotEq),
                (">", Gt),
                (">=", GtEq),
                ("<", Lt),
                ("<=", LtEq),
                ("&&", BoolAnd),
                ("||", BoolOr),
                ("!", Not),
                ("=", Eq),
                (",", Comma),
                (".", Dot),
                (":", Colon),
                (";", Semi),
                ("->", Arrow),
                ("\n", Nl),
                ("// Hello, world!\n", Nl),
                ("// Hello, world!", Nl),
            ]
        };

        assert_source(&test_source);
    }
}
