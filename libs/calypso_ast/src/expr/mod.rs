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

impl BinOpKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
            Self::Modulo => "%",
            Self::Exponent => "**",
            Self::LogicalOr => "||",
            Self::LogicalAnd => "&&",
            Self::BitOr => "|",
            Self::BitXor => "^",
            Self::BitAnd => "&",
            Self::BitShiftLeft => "<<",
            Self::BitShiftRight => ">>",
            Self::Equal => "==",
            Self::NotEqual => "!=",
            Self::Lt => "<",
            Self::Gt => ">",
            Self::LtEq => "<=",
            Self::GtEq => ">=",
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum UnOpKind {
    Negative,
    UnaryNot,
}

impl UnOpKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Negative => "-",
            Self::UnaryNot => "!",
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

    pub fn name(self) -> &'static str {
        match self {
            Self::None => "",
            Self::Decimal => "0d",
            Self::Binary => "0b",
            Self::Octal => "0o",
            Self::Hexadecimal => "0x",
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

impl Suffix {
    pub fn name(self) -> &'static str {
        match self {
            Self::Invalid | Self::TrueFloat => "",
            Self::Uint => "u",
            Self::Sint => "s",
            Self::Float => "f",
        }
    }
}
