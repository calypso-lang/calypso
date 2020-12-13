#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
/// Types of Calypso tokens
///
/// # Operators
///
/// ## Booleans
/// - `Less`: `<`: less than
/// - `BoolEqual`: `==`: equal to
/// - `Greater`: `>`: greater than
/// - `LessEqual`: `<=`: less than or equal to
/// - `NotEqual`: `!=`: not equal to
/// - `GreaterEqual`: `>=`: greater than or equal to
/// - `BoolOr`: `||`: Boolean OR
/// - `BoolAnd`: `&&`: Boolean AND
/// - `Bang`: `!`: Boolean NOT
///
/// ## Numbers
/// - `Plus`: `+`: Addition
/// - `PlusAssign`: `+=`: Addition assignment
/// - `Minus`: `-`: Subtraction (binary) and Negation (unary)
/// - `MinusAssign`: `-=`: Subtraction assignment
/// - `Star`: `*`: Multiplication
/// - `StarAssign`: `*=`: Multiplication
/// - `Slash`: `/`: Division
/// - `SlashAssign`: `/=`: Division assignment
/// - `Exp`: `**`: Exponentiation
/// - `ExpAssign`: `**=`: Exponentiation assignment
/// - `Rem`: `%`: Remainder
/// - `RemAssign`: `%=`: Remainder assignment
/// - `Range`: `..`: Integer ranges
/// - `RangeInc`: `..=`: Inclusive integer range
///
/// ## Strings/Lists
/// - `GreaterLess`: `<>`: Concatenation
///
/// ## Bitwise
/// - `Shr`: `>>`: Shift right
/// - `ShrAssign`: `>>=`: Shift right assignment
/// - `Shl`: `<<`: Shift left
/// - `ShlAssign`: `<<=`: Shift left assignment
/// - `Pipe`: `|`: Bitwise OR
/// - `PipeAssign`: `|=`: Bitwise OR assignment
/// - `And`: `&`: Bitwise AND
/// - `AndAssign`: `&=`: Bitwise AND assignment
/// - `Caret`: `^`: Bitwise XOR
/// - `CaretAssign`: `^=`: Bitwise XOR assignment
/// - `Tilde`: `~`: Bitwise NOT
///
/// ## Variables
/// - `Equal`: `=`: Variable assignment
///
/// # Keywords
///
/// ## Values
/// - `KwIs`: `is`: value A is at the same memory location as value B (aliased)
/// - `KwIsa`: `isa`: value A is of type B
///
/// ## Types
/// - `KwBoolTy`: `bool`
/// - `KwSintTy`: `sint`
/// - `KwUintTy`: `uint`
/// - `KwFloatTy`: `float`
/// - `KwStringTy`: `string`
/// - `KwCharTy`: `char`
/// - `KwTupleTy`: `tuple`
/// - `KwArrayTy`: `array`
///
/// ## Booleans
/// - `KwFalse`: `false`, `KwTrue`: `true`: Booleans
///
/// ## Control flow
/// - `KwIf`: `if`, `KwElse`: `else`: `if`, `else if`, `else` blocks
/// - `KwFor`: `for`, `KwIn`: `in`: `for...in` loops
/// - `KwLoop`: `loop`: endless loops
/// - `KwWhile`: `while`: `while` loops
/// - `KwMatch`: `match`: pattern matching
/// - `KwRet`: `ret`: return a value or nothing to short-circuit
/// - `KwBreak`: `break`: break out of a loop
///
/// ## Functions and Modularity
/// - `KwFn`: `fn`: function definition
/// - `KwNative`: `native: native function (i.e. Rust/C ABI)
/// - `KwMod`: `mod`: module declaration
/// - `KwUse`: `use`: use an external package
/// - `KwImport`: `import`: import a path
/// - `KwPub`: `pub`: public visibility
///
/// ## Variables
/// - `KwLet`: `let`: define a variable
/// - `KwMut`: `mut`: mutable variable
/// - `KwUndef`: `undef`: variable is not defined, but is not `null` as that is explicitly set
/// - `KwNull`: `null`: explicit none-type
/// - `KwDel`: `del`: delete a value (garbage-collect its value, then set it back to `undef` or, in strict mode, make the variable totally undefined)
///
/// ## Conversions
/// - `KwAs`: `as`: convert to type, rename import
///
/// # Special Tokens
/// - `Eof`: end of file
/// - `Ws`: Whitespace, including newlines. It just spans the whitespace.
/// - `Sync`: Synchronization-consumed characters.
///
/// ## Grouping and Punctuation
/// - `LeftParen`: `(`, `RightParen`: `)`: Left and right parentheses
/// - `LeftBrace`: `{`, `RightBrace`: `}`: Left and right braces
/// - `LeftBracket`: `[`, `RightBracket`: `]`: Left and right brackets
/// - `Comma`: `,`: Comma
/// - `Semi`: `;`: Semicolon
/// - `Dot`: `.`: Methods, imports, etc...
/// - `Under`: `_`: Nothing at the moment, but it's not an identifier on its own.
///
/// ## Attributes
/// - `Hash`: `#`: Attributes
/// - `HashBang`: `#!`: Global attributes
///
/// # Literals
///
/// - `Ident`: `([A-Za-z_])([A-Za-z0-9_]*)`: Identifier
/// - `SintLiteral`: `[-]<DIGIT(s)>[s]`: Signed integer literal
/// - `UintLiteral`: `<DIGIT(s)>[u]`: Unsigned integer literal
/// - `FloatLiteral`: `<DIGIT(s)>.<DIGIT(s)>[f]`: float literal
/// - `StringLiteral`: `"[STRING]"`: string literal
/// - `CharLiteral`: `'<CHAR>'`: character literal
pub enum TokenType {
    Less,
    LessEqual,

    NotEqual,
    BoolEqual,

    Greater,
    GreaterEqual,

    BoolOr,
    BoolAnd,

    Bang,

    Plus,
    PlusAssign,

    Minus,
    MinusAssign,

    Star,
    StarAssign,

    Slash,
    SlashAssign,

    Exp,
    ExpAssign,

    Rem,
    RemAssign,

    Range,
    RangeInc,

    GreaterLess,

    Shr,
    ShrAssign,

    Shl,
    ShlAssign,

    Pipe,
    PipeAssign,

    And,
    AndAssign,

    Caret,
    CaretAssign,

    Tilde,

    Equal,

    Keyword(Keyword),

    Eof,
    Ws,
    Sync,

    LeftParen,
    RightParen,

    LeftBrace,
    RightBrace,

    LeftBracket,
    RightBracket,

    Comma,
    Semi,
    Dot,
    Under,

    Hash,
    HashBang,

    Ident,

    SintLiteral(Radix),
    UintLiteral(Radix),
    FloatLiteral,
    StringLiteral,
    CharLiteral,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Keyword {
    Is,
    Isa,

    BoolTy,
    SintTy,
    UintTy,
    FloatTy,
    StringTy,
    CharTy,
    TupleTy,
    ArrayTy,

    False,
    True,

    If,
    Else,

    For,
    In,
    Loop,
    While,
    Match,
    Ret,
    Break,

    Fn,
    Native,
    Mod,
    Use,
    Import,
    Pub,

    Let,
    Mut,
    Undef,
    Null,
    Del,

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
