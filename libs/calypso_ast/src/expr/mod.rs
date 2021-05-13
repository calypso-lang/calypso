use std::fmt::{self, Display};

use calypso_base::span::Spanned;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'tok> {
    BinOp(Box<Expr<'tok>>, BinOpKind, Box<Expr<'tok>>),
    UnOp(Spanned<UnOpKind>, Box<Expr<'tok>>),
    Primary(Primary<'tok>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BinOpKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,
    LogicalOr,
    LogicalAnd,
    BitOr,
    BitXor,
    BitAnd,
    BitShiftLeft,
    BitShiftRight,
    Equal,
    NotEqual,
    Lt,
    Gt,
    LtEq,
    GtEq,
}

impl Display for BinOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::Multiply => write!(f, "*"),
            Self::Divide => write!(f, "/"),
            Self::Modulo => write!(f, "%"),
            Self::Exponent => write!(f, "**"),
            Self::LogicalOr => write!(f, "||"),
            Self::LogicalAnd => write!(f, "&&"),
            Self::BitOr => write!(f, "|"),
            Self::BitXor => write!(f, "^"),
            Self::BitAnd => write!(f, "&"),
            Self::BitShiftLeft => write!(f, "<<"),
            Self::BitShiftRight => write!(f, ">>"),
            Self::Equal => write!(f, "=="),
            Self::NotEqual => write!(f, "!="),
            Self::Lt => write!(f, "<"),
            Self::Gt => write!(f, ">"),
            Self::LtEq => write!(f, "<="),
            Self::GtEq => write!(f, ">="),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnOpKind {
    Negative,
    UnaryNot,
}

impl Display for UnOpKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Negative => write!(f, "-"),
            Self::UnaryNot => write!(f, "!"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Primary<'tok> {
    Number(&'tok str, Radix, Option<Suffix>),
    Bool(bool),
}

impl<'tok> Primary<'tok> {
    /// Implementation detail.
    pub fn detuple_number((s, base, suffix): (&'tok str, Radix, Option<Suffix>)) -> Self {
        Self::Number(s, base, suffix)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Number radixes.
pub enum Radix {
    /// No prefix (`0d` by default)
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
    pub fn radix(self) -> u32 {
        match self {
            Self::None => 10,
            Self::Decimal => 10,
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Hexadecimal => 16,
        }
    }
}

impl Display for Radix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decimal => write!(f, "0d"),
            Self::Binary => write!(f, "0b"),
            Self::Octal => write!(f, "0o"),
            Self::Hexadecimal => write!(f, "0x"),
            _ => Ok(()),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Number suffixes.
pub enum Suffix {
    /// `u`
    Uint,
    /// `s`
    Sint,
    /// `f`
    Float,
    /// Invalid suffix
    Invalid,
    /// Actually a float literal, not an integer literal converted to a float
    TrueFloat,
}

impl Display for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uint => write!(f, "u"),
            Self::Sint => write!(f, "s"),
            Self::Float => write!(f, "f"),
            _ => Ok(()),
        }
    }
}
