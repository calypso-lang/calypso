#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// Types of Calypso tokens
pub enum TokenType {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Star,
    /// `**`
    StarStar,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `|`
    Pipe,
    /// `||`
    PipePipe,
    /// `&`
    And,
    /// `&&`
    AndAnd,
    /// `!`
    Bang,
    /// `^`
    Caret,
    /// `>>`
    GtGt,
    /// `<<`
    LtLt,
    /// `==`
    EqEq,
    /// `>=`
    GtEq,
    /// `<=`
    LtEq,
    /// `!=`
    BangEq,
    /// `<`
    Lt,
    /// `Gt`
    Gt,
    /// `:`
    Colon,

    /// `_`
    Under,
    /// `(`
    LParen,
    /// `)`
    RParen,

    /// End-of-file / end-of-input
    Eof,
    /// Unexpected characters, included for lexer synchronization
    Unexpected,
    /// Identifier
    Ident,
    /// Whitespace
    Ws,
    /// Line comments
    LineComment {
        /// Is this a documentation comment?
        doc: bool,
        /// Is this an inner doc comment?
        inner: bool,
    },
    /// Comment
    BlockComment {
        /// Is this a documentation comment?
        doc: bool,
        /// Is this an inner doc comment?
        inner: bool,
        /// How deeply nested this comment is
        nest_level: usize,
    },

    /// Keyword
    Keyword(Keyword),
    /// String literal
    String,
    /// Character literal
    Char,
    /// Integer literal (not split into sint/uint cause of constraints)
    Int {
        /// The integer suffix, if present
        suffix: Option<Suffix>,
        /// The integer radix, or [`Radix::Decimal`] if not present
        radix: Radix,
    },
    /// Float literal
    Float,
    // NOTE: PRODUCTIVITY MARKER: This is purposefully incomplete. I'll come back to it later.
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Keyword {
    /// `false`
    False,
    /// `null`
    Null,
    /// `true`
    True,
    // NOTE: PRODUCTIVITY MARKER: This is purposefully incomplete. I'll come back to it later.
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
