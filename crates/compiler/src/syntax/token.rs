use std::fmt;

use crate::symbol::{Symbol, kw::Keyword};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token {
    // Grouping characters
    LeftParen,   // (
    RightParen,  // )
    LeftSquare,  // [
    RightSquare, // ]
    LeftCurly,   // {
    RightCurly,  // }
    // Basic arithmetic operations
    Plus,     // +
    Minus,    // -
    Star,     // *
    Slash,    // /
    Percent,  // %
    StarStar, // **
    // Bitwise operations
    Shr, // >>
    Shl, // <<
    And, // &
    Pipe,  // |
    Xor, // ^
    // Comparison operations
    EqEq,  // ==
    NotEq, // !=
    Gt,    // >
    GtEq,  // >=
    Lt,    // <
    LtEq,  // <=
    // Boolean operations
    BoolAnd, // &&
    BoolOr,  // ||
    Not,     // !
    // General punctuation
    Eq,    // =
    Comma, // ,
    Dot,   // .
    Colon, // :
    Semi,  // ;
    Arrow, // ->
    Nl,    // Newline sequences
    // Identifiers
    Ident(Symbol),
    // Keywords
    Keyword(Keyword),
    // Numerals
    Numeral(i128, Numeral),
    // Strings
    String(Symbol),
    // End-of-file
    Eof,
    // Error
    Error,
}

impl Token {
    pub fn description(&self) -> &'static str {
        match self {
            Token::LeftParen => "`(`",
            Token::RightParen => "`)`",
            Token::LeftSquare => "`[`",
            Token::RightSquare => "`]`",
            Token::LeftCurly => "`{`",
            Token::RightCurly => "`}`",
            Token::Plus => "`+`",
            Token::Minus => "`-`",
            Token::Star => "`*`",
            Token::Slash => "`/`",
            Token::Percent => "`%`",
            Token::StarStar => "`**`",
            Token::Shr => "`>>`",
            Token::Shl => "`<<`",
            Token::And => "`&`",
            Token::Pipe => "`|`",
            Token::Xor => "`^`",
            Token::EqEq => "`==`",
            Token::NotEq => "`!=`",
            Token::Gt => "`>`",
            Token::GtEq => "`>=`",
            Token::Lt => "`<`",
            Token::LtEq => "`<=`",
            Token::BoolAnd => "`&&`",
            Token::BoolOr => "`||`",
            Token::Not => "`!`",
            Token::Eq => "`=`",
            Token::Comma => "`,`",
            Token::Dot => "`.`",
            Token::Colon => "`:`",
            Token::Semi => "`;`",
            Token::Arrow => "`->`",
            Token::Nl => "newline",
            Token::Ident(_) => "identifier",
            Token::Keyword(keyword) => keyword.description(),
            Token::Numeral(..) => "number",
            Token::String(..) => "string",
            Token::Eof => "end-of-file",
            Token::Error => "invalid token",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Numeral {
    Integer { radix: Radix, suffix: Suffix },
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Radix {
    /// No prefix (decimal by default)
    None,
    /// `0d`
    Decimal,
    /// `0b`
    Binary,
    /// `0o`
    Octal,
    /// `0x`
    Hexadecimal,
}

impl Radix {
    pub fn radix(self) -> u8 {
        match self {
            Self::None | Self::Decimal => 10,
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Hexadecimal => 16,
        }
    }
}

impl fmt::Display for Radix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Radix::None => Ok(()),
            Radix::Decimal => write!(f, "0d"),
            Radix::Binary => write!(f, "0b"),
            Radix::Octal => write!(f, "0o"),
            Radix::Hexadecimal => write!(f, "0x"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Suffix {
    /// No suffix (compiler-inferred integer)
    None,
    Signed(IntegerWidth),
    Unsigned(IntegerWidth),
}

impl fmt::Display for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suffix::None => Ok(()),
            Suffix::Signed(width) => write!(f, "s{width}"),
            Suffix::Unsigned(width) => write!(f, "u{width}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntegerWidth {
    I8,
    I16,
    I32,
    I64,
    Ptr,
}

impl fmt::Display for IntegerWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IntegerWidth::I8 => write!(f, "8"),
            IntegerWidth::I16 => write!(f, "16"),
            IntegerWidth::I32 => write!(f, "32"),
            IntegerWidth::I64 => write!(f, "64"),
            IntegerWidth::Ptr => write!(f, "ptr"),
        }
    }
}
