#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Types of Calypso tokens
pub enum TokenType {
    /// `<`
    Lt,
    /// `<=`
    LtEq,
    /// `!=`
    BangEq,
    /// `==`
    EqEq,
    /// `>`
    Gt,
    /// `>=`
    GtEq,
    /// `||`
    PipePipe,
    /// `&&`
    AndAnd,
    /// `!`
    Bang,
    /// `+`
    Plus,
    /// `+=`
    PlusEq,
    /// `-`
    Minus,
    /// `-=`
    MinusEq,
    /// `*`
    Star,
    /// `*=`
    StarEq,
    /// `/`
    Slash,
    /// `/=`
    SlashEq,
    /// `**`
    StarStar,
    /// `**=`
    StarStarEq,
    /// `%`
    Percent,
    /// `%=`
    PercentEq,
    /// `..`
    DotDot,
    /// `..=`
    DotDotEq,
    /// `<>`
    LtGt,
    /// `<<`
    LtLt,
    /// `<<=`
    LtLtEq,
    /// `>>`
    GtGt,
    /// `>>=`
    GtGtEq,
    /// `|`
    Pipe,
    /// `|=`
    PipeEq,
    /// `&`
    And,
    /// `&=`
    AndEq,
    /// `^`
    Caret,
    /// `^=`
    CaretEq,
    /// `~`
    Tilde,
    /// `=`
    Eq,
    /// Keywords
    Keyword(Keyword),
    /// End of file
    Eof,
    /// Whitespace, including newlines. This token just spans the whitespace
    Ws,
    /// Unexpected characters consumed by synchronization. These are invalid in regular code.
    Unexpected,
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBrace,
    /// `}`
    RBrace,
    /// `[`
    LBracket,
    /// `]`
    RBracket,
    /// `,`
    Comma,
    /// `;`
    Semi,
    /// `.`
    Dot,
    /// `_`
    Under,
    /// `:`
    Colon,
    /// `->`
    Arrow,
    /// `|>`
    PipeGt,
    /// `#`
    Hash,
    /// `#!`
    HashBang,
    /// Identifiers, excluding keywords.
    Ident,
    /// Signed integer literal
    SintLiteral(Radix),
    /// Unsigned integer literal
    UintLiteral(Radix),
    /// Float literal
    FloatLiteral,
    /// String literal
    StringLiteral,
    /// Character literal
    CharLiteral,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Keyword {
    /// `is`
    Is,
    /// `isa`
    Isa,
    /// `bool`
    BoolTy,
    /// `sint`
    SintTy,
    /// `uint`
    UintTy,
    /// `float`
    FloatTy,
    /// `string`
    StringTy,
    /// `char`
    CharTy,
    /// `tuple`
    TupleTy,
    /// `array`
    ArrayTy,
    /// `false`
    False,
    /// `true`
    True,
    /// `if`
    If,
    /// `else`
    Else,
    /// `for`
    For,
    /// `in`
    In,
    /// `loop`
    Loop,
    /// `wile`
    While,
    /// `case`
    Case,
    /// `cond`
    Cond,
    /// `ret`
    Ret,
    /// `break`
    Break,
    /// `fn`
    Fn,
    /// `extern`
    Extern,
    /// `mod`
    Mod,
    /// `use`
    Use,
    /// `import`
    Import,
    /// `pub`
    Pub,
    /// `let`
    Let,
    /// `mut`
    Mut,
    /// `undef`
    Undef,
    /// `null`
    Null,
    /// `del`
    Del,
    /// `as`
    As,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Number radixes.
pub enum Radix {
    /// No prefix
    Decimal,
    /// `0b`
    Binary,
    /// `0o`
    Octal,
    /// `0x`
    Hexadecimal,
}
