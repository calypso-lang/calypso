use calypso_base::{
    span::Spanned,
    symbol::{PotentiallyInterned, Symbol},
};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnOpKind {
    Negative,
    UnaryNot,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Primary<'tok> {
    Number(&'tok str, Radix, Option<Suffix>),
    Bool(bool),
    Atom(Symbol),
    AtomStr(PotentiallyInterned<'tok>),
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
    /// No prefix or `0d`
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
            Self::Decimal => 10,
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Hexadecimal => 16,
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
}
