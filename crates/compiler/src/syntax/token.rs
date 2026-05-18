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
    Or,  // |
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
    Numeral(Numeral),
    // Strings
    String,
    // End-of-file
    Eof,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntegerWidth {
    I8,
    I16,
    I32,
    I64,
    Ptr,
}
