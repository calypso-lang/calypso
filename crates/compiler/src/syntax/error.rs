use std::borrow::Cow;

use ariadne::{Color, Label, Report, ReportKind};

use crate::ctxt::GlobalCtxt;

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
    UnclosedString,
}

impl LexicalError {
    fn message(self) -> &'static str {
        match self.kind {
            LexicalErrorKind::UnexpectedToken => "syntax error: unexpected token",
            LexicalErrorKind::InvalidNumeralRadixSpecifier => {
                "syntax error: invalid number base specifier"
            }
            LexicalErrorKind::InvalidNumeralRadixCharacter => {
                "syntax error: invalid digit in number"
            }
            LexicalErrorKind::InvalidNumeralWidth => "syntax error: invalid width for integer",
            LexicalErrorKind::ZeroPrefixedNumeral => {
                "syntax error: zero-prefixed numerals are not allowed"
            }
            LexicalErrorKind::UnclosedString => "syntax error: unclosed string",
        }
    }

    pub fn into_report(self, _gcx: &GlobalCtxt) -> Report<Span> {
        let mut b = Report::build(ReportKind::Error, self.location);
        b.set_message(self.message());
        b.add_label(Label::new(self.location).with_color(Color::Red));
        if matches!(self.kind, LexicalErrorKind::ZeroPrefixedNumeral) {
            b.add_note(
                "zero-prefixed numerals are disallowed to \
		 prevent confusion with octal numerals",
            );

            b.add_help(
                "if you wanted octal, use `0o...`; \
		 if you wanted decimal, use `0d...`",
            );
        }
        b.finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SyntaxError {
    pub kind: SyntaxErrorKind,
    pub location: Span,
}

impl SyntaxError {
    fn message(self) -> Cow<'static, str> {
        match self.kind {
            SyntaxErrorKind::Unexpected { expected, found } => {
                format!("syntax error: expected {expected}, found {found}.").into()
            }
            SyntaxErrorKind::ExpectedMatching {
                expected,
                to_match,
                found,
                ..
            } => {
                format!("syntax error: expected matching {expected} for {to_match}, found {found}.")
                    .into()
            }
        }
    }

    pub fn into_report(self, _gcx: &GlobalCtxt) -> Report<Span> {
        let mut b = Report::build(ReportKind::Error, self.location);
        b.set_message(self.message());
        b.add_label(Label::new(self.location).with_color(Color::Red));
        if let SyntaxErrorKind::ExpectedMatching { at, .. } = self.kind {
            b.add_label(
                Label::new(at)
                    .with_message("unclosed delimiter")
                    .with_color(Color::Blue),
            );
        }
        b.finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SyntaxErrorKind {
    Unexpected {
        expected: &'static str,
        found: &'static str,
    },
    ExpectedMatching {
        expected: &'static str,
        to_match: &'static str,
        at: Span,
        found: &'static str,
    },
}
