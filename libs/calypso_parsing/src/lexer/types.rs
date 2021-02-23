#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Types of Calypso tokens
pub enum TokenType {
    /// `&`
    And,
    /// `&&`
    AndAnd,
    /// `&=`
    AndEq,
    /// `->`
    Arrow,
    /// `@`
    At,
    /// `@!`
    AtBang,
    /// `!`
    Bang,
    /// `!=`
    BangEq,
    /// `^`
    Caret,
    /// `^=`
    CaretEq,
    /// `:`
    Colon,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `..=`
    DotDotEq,
    /// `=`
    Eq,
    /// `==`
    EqEq,
    /// `>`
    Gt,
    /// `>=`
    GtEq,
    /// `>>`
    GtGt,
    /// `>>=`
    GtGtEq,
    /// `{`
    LBrace,
    /// `[`
    LBracket,
    /// `(`
    LParen,
    /// `<`
    Lt,
    /// `<=`
    LtEq,
    /// `<<`
    LtLt,
    /// `<<=`
    LtLtEq,
    /// `-`
    Minus,
    /// `-=`
    MinusEq,
    /// `%`
    Percent,
    /// `%=`
    PercentEq,
    /// `|`
    Pipe,
    /// `|=`
    PipeEq,
    /// `|>`
    PipeGt,
    /// `||`
    PipePipe,
    /// `+`
    Plus,
    /// `+=`
    PlusEq,
    /// `}`
    RBrace,
    /// `]`
    RBracket,
    /// `)`
    RParen,
    /// `/`
    Slash,
    /// `/=`
    SlashEq,
    /// `*`
    Star,
    /// `*=`
    StarEq,
    /// `**`
    StarStar,
    /// `**=`
    StarStarEq,
    /// `~`
    Tilde,
    /// `_`
    Under,

    /// End of file or input
    Eof,
    /// Unexpected characters, included for lexer synchronization
    Unexpected,
    /// Identifier
    Ident,
    /// Whitespace
    Ws,
    /// Comment
    Comment {
        doc: bool,
        inner: bool,
        multiline: bool,
    },
    /// Keyword
    Keyword(Keyword),
    // Literal(Literal)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Keyword {
    /// `as`
    As,
    /// `break`
    Break,
    /// `case`
    Case,
    /// `del`
    Del,
    /// `do`
    Do,
    /// `else`
    Else,
    /// `end`
    End,
    /// `extern`
    Extern,
    /// `false`
    False,
    /// `fn`
    Fn,
    /// `for`
    For,
    /// `if`
    If,
    /// `import`
    Import,
    /// `in`
    In,
    /// `is`
    Is,
    /// `isa`
    Isa,
    /// `let`
    Let,
    /// `loop`
    Loop,
    /// `mod`
    Mod,
    /// `mut`
    Mut,
    /// `null`
    Null,
    /// `panic`
    Panic,
    /// `pub`
    Pub,
    /// `ret`
    Ret,
    /// `root`
    Root,
    /// `self` (named `Zelf` because `Self` is reserved)
    Zelf,
    /// `super`
    Super,
    /// `true`
    True,
    /// `try`
    Try,
    /// `while`
    While,
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
