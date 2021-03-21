#[derive(Clone, Debug, PartialEq)]
pub enum Expr<'tok> {
    BinOp(Box<Expr<'tok>>, BinOpKind, Box<Expr<'tok>>),
    UnOp(UnOpKind, Box<Expr<'tok>>),
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
    Sint(i64),
    Uint(u64),
    Float(f64),
    Bool(bool),
    Atom(&'tok str),
    AtomStr(&'tok str),
}
