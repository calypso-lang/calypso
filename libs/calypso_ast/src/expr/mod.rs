use std::fmt::{self, Display};

use calypso_base::{span::Spanned, symbol::Symbol};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    BinOp(Spanned<Box<Expr>>, Spanned<BinOpKind>, Spanned<Box<Expr>>),
    UnOp(Spanned<UnOpKind>, Spanned<Box<Expr>>),
    Primary(Spanned<Primary>),
    Block(Vec<Spanned<Expr>>),
    Let(Mutability, Spanned<Symbol>, Spanned<Box<Expr>>, Spanned<Box<Expr>>)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mutability {
    Mut,
    Immut,
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Primary {
    Number(Numeral),
    Bool(bool),
    Var(Symbol),
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
    #[must_use]
    pub fn radix(self) -> u32 {
        match self {
            Self::None | Self::Decimal => 10,
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
            Self::None => Ok(()),
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
}

impl Display for Suffix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Uint => write!(f, "u"),
            Self::Sint => write!(f, "s"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Numeral {
    Integer {
        suffix: Option<Suffix>,
        radix: Radix,
    },
    Float {
        from_integer: bool,
    },
}
