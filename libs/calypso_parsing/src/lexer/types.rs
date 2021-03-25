use calypso_ast::expr::{Radix, Suffix};
use calypso_base::symbol::Symbol;

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
    Ident(Symbol),
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
    Keyword(Symbol),
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
