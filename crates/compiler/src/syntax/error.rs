use std::borrow::Cow;

use ariadne::{Color, Config, Label, Report, ReportKind};

use crate::ctxt::GlobalCtxt;

use super::{span::Span, token::Token};

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

impl LexicalError {
    fn message(self) -> &'static str {
        match self.kind {
            LexicalErrorKind::UnexpectedToken => "unexpected token",
            LexicalErrorKind::InvalidNumeralRadixSpecifier => "invalid number base specifier",
            LexicalErrorKind::InvalidNumeralRadixCharacter => "invalid digit in number",
            LexicalErrorKind::InvalidNumeralWidth => "invalid width for integer",
            LexicalErrorKind::ZeroPrefixedNumeral => "zero-prefixed numerals are not allowed",
        }
    }

    pub fn into_report(self, gcx: &GlobalCtxt) -> Report<'static, Span> {
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
            SyntaxErrorKind::UnexpectedEof => "syntax error: unexpected end-of-file".into(),
            SyntaxErrorKind::UnexpectedToken { expected, found } => {
                format!("syntax error: expected {expected}, found {found}").into()
            }
        }
    }

    pub fn into_report(self, _gcx: &GlobalCtxt) -> Report<'static, Span> {
        let mut b = Report::build(ReportKind::Error, self.location);
        b.set_message(self.message());
        b.add_label(Label::new(self.location).with_color(Color::Red));
        b.finish()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SyntaxErrorKind {
    UnexpectedEof,
    UnexpectedToken {
        expected: &'static str,
        found: &'static str,
    },
}
