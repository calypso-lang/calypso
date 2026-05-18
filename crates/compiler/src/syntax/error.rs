use super::span::Span;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LexicalError {
    pub kind: LexicalErrorKind,
    pub location: Span,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LexicalErrorKind {
    UnexpectedToken,
    InvalidNumeralRadixSpecifier,
    InvalidNumeralRadixCharacter,
    InvalidNumeralWidth,
    ZeroPrefixedNumeral,
}
