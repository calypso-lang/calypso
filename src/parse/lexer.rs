use std::{convert::TryFrom, ops::Range, sync::Arc};

use ariadne::{Color, Label, ReportKind};
use itertools::Itertools;
use logos::{Lexer, Logos};

use calypso_ast::expr::{Numeral, Radix, Suffix};

use crate::{
    ctxt::GlobalCtxt,
    diagnostic::Diagnostic,
    symbol::{kw::Keyword, Symbol},
};

use super::Spanned;

pub type Lexeme<'lex> = Spanned<(Token, &'lex str)>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Logos)]
#[logos(extras = (Symbol, Arc<GlobalCtxt>))]
pub enum Token {
    #[token("<<=")]
    LtLtEq,
    #[token("<<")]
    LtLt,
    #[token("<=")]
    LtEq,
    #[token("<")]
    Lt,

    #[token(">>=")]
    GtGtEq,
    #[token(">>")]
    GtGt,
    #[token(">=")]
    GtEq,
    #[token(">")]
    Gt,

    #[token("==")]
    EqEq,
    #[token("=")]
    Eq,

    #[token("!=")]
    BangEq,
    #[token("!")]
    Bang,

    #[token("||")]
    PipePipe,
    #[token("|=")]
    PipeEq,
    #[token("|")]
    Pipe,

    #[token("&&")]
    AndAnd,
    #[token("&=")]
    AndEq,
    #[token("&")]
    And,

    #[token("+=")]
    PlusEq,
    #[token("+")]
    Plus,

    #[token("->")]
    Arrow,

    #[token("-=")]
    MinusEq,
    #[token("-")]
    Minus,

    #[token("**=")]
    StarStarEq,
    #[token("**")]
    StarStar,
    #[token("*=")]
    StarEq,
    #[token("*")]
    Star,

    #[token("/=")]
    SlashEq,
    #[token("/")]
    Slash,

    #[token("%=")]
    PercentEq,
    #[token("%")]
    Percent,

    #[token("^=")]
    CaretEq,
    #[token("^")]
    Caret,

    #[token("@!")]
    AtBang,
    #[token("@")]
    At,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    Semi,
    #[token("_")]
    Under,
    #[token(".")]
    Dot,

    #[regex("_[A-Za-z0-9_]+|[A-Za-z][A-Za-z0-9_]*", ident)]
    IdentLike(IdentLike),

    #[regex("///(.*)\n?", |_| CommentProps::doc())]
    #[regex("//!(.*)\n?", |_| CommentProps::inner_doc())]
    #[regex("//(.*)\n?",  |_| CommentProps::default())]
    Comment(CommentProps),

    // #[regex("[\n]+", |lex| lex.span().len())]
    // Nl(usize),

    // this hurts.
    // cc https://github.com/maciejhirsz/logos/issues/126
    // TODO(parse): this could probably use a raw string, and it'd be slightly less ugly
    // emphasis on slightly
    #[regex(
        "\"([^\n\r\"\\\\]|(\\\\([nrt\\\\0'\"]|\r\n|\n|x[0-9a-fA-F][0-9a-fA-F]|u\\{[0-9a-fA-F][0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?\\})))*\""
    )]
    String,

    #[regex("'([^\n\r'\\\\]|(\\\\([nrt\\\\0'\"]|x[0-9a-fA-F][0-9a-fA-F]|u\\{[0-9a-fA-F][0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?[0-9a-fA-F]?\\})))'")]
    Char,

    #[regex("0x[0-9a-fA-F][0-9a-fA-F_]*[su]?", |lex| radix_numeral(lex, Radix::Hexadecimal))]
    #[regex("0o[0-7][0-7_]*[su]?", |lex| radix_numeral(lex, Radix::Octal))]
    #[regex("0b[01][01_]*[su]?", |lex| radix_numeral(lex, Radix::Binary))]
    #[regex("0d[0-9][0-9_]*[su]?", |lex| radix_numeral(lex, Radix::Decimal))]
    #[regex("[0-9][0-9_]*\\.[0-9][0-9_]*(e[+-]?[0-9][0-9_]*)?", |_| Numeral::Float { from_integer: false })]
    #[regex("[0-9][0-9_]*e[+-]?[0-9][0-9_]*", |_| Numeral::Float { from_integer: false })]
    #[regex("[0-9][0-9_]*[suf]?", |lex| integer_numeral(lex))]
    Numeral(Numeral),

    #[regex(
        "[\t\u{000B}\u{000C}\r \u{0085}\u{200E}\u{200F}\u{2028}\u{2029}\n]+",
        logos::skip
    )]
    #[error]
    Error,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IdentLike {
    Ident(Symbol),
    Keyword(Keyword),
}

pub fn ident(lex: &mut Lexer<Token>) -> IdentLike {
    let sym = Symbol::intern(lex.slice());

    match Keyword::try_from(sym) {
        Ok(kw) => IdentLike::Keyword(kw),
        Err(sym) => IdentLike::Ident(sym),
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommentProps {
    pub is_doc: bool,
    pub is_inner: bool,
}

impl CommentProps {
    #[must_use]
    pub fn doc() -> Self {
        Self {
            is_doc: true,
            is_inner: false,
        }
    }

    #[must_use]
    pub fn inner_doc() -> Self {
        Self {
            is_doc: true,
            is_inner: true,
        }
    }
}

fn radix_numeral(lex: &mut Lexer<Token>, radix: Radix) -> Numeral {
    match lex.slice().chars().last() {
        Some('s') => Numeral::Integer {
            suffix: Some(Suffix::Sint),
            radix,
        },
        Some('u') => Numeral::Integer {
            suffix: Some(Suffix::Uint),
            radix,
        },
        _ => Numeral::Integer {
            suffix: None,
            radix,
        },
    }
}

fn integer_numeral(lex: &mut Lexer<Token>) -> Numeral {
    match lex.slice().chars().last() {
        Some('s') => Numeral::Integer {
            suffix: Some(Suffix::Sint),
            radix: Radix::None,
        },
        Some('u') => Numeral::Integer {
            suffix: Some(Suffix::Uint),
            radix: Radix::None,
        },
        Some('f') => Numeral::Float { from_integer: true },
        _ => Numeral::Integer {
            suffix: None,
            radix: Radix::None,
        },
    }
}

#[allow(clippy::missing_panics_doc)]
pub fn tokens(
    source: &'_ str,
    name: Symbol,
    gcx: Arc<GlobalCtxt>,
) -> impl Iterator<Item = Lexeme<'_>> {
    let lex = Token::lexer_with_extras(source, (name, Arc::clone(&gcx)));
    let gcx2 = Arc::clone(&gcx);
    lex.spanned()
        .map(|(tok, span)| {
            (
                tok,
                u32::try_from(span.start).expect("span.start <= u32::MAX")
                    ..u32::try_from(span.end).expect("span.end <= u32::MAX"),
            )
        })
        .map(Spanned::from)
        .map(|x| (x, false))
        .coalesce(move |a, mut b| {
            if a.0.value() == &Token::Error {
                // Mark errors on quotes/apostrophes as fatal, as they probably
                // are. The worst case is that they aren't, and the user is
                // mildly annoyed. With the current state of logos, however,
                // there's not really a way to do this without backtracking or
                // n-token-lookahead, which I'd rather not do for this at the
                // moment.
                if source
                    .get(Range::<usize>::from(a.0.span()))
                    .map_or(false, |x: &str| x == "\'" || x == "\"")
                {
                    gcx2.diag.write().report_fatal(
                        Diagnostic::build(ReportKind::Error, name, a.0.span().lo() as usize)
                            .with_code("E0001")
                            .with_message("A syntax error was encountered.")
                            .with_label(
                                Label::new((name, a.0.span()).into()).with_color(Color::Red),
                            )
                            .finish(),
                    );
                }

                // Don't coalesce if there is a fatal error, and mark when it happened
                // (and when was after it happened, so we know where to stop)
                if gcx2.diag.read().fatal().is_some() {
                    b.1 = true;
                    return Err((a, b));
                }
                if b.0.value() == &Token::Error {
                    return Ok((Spanned::new(a.0.span().to(b.0.span()), Token::Error), false));
                }
            }

            Err((a, b))
        })
        // Stop after when there's a fatal error
        .take_while(move |(_, end)| !*end)
        .map(move |(x, end)| {
            if x.value() == &Token::Error && !end {
                // Report non-fatal syntax errors
                gcx.diag.write().report_syncd(
                    Diagnostic::build(ReportKind::Error, name, x.span().lo() as usize)
                        .with_code("E0001")
                        .with_message("A syntax error was encountered.")
                        .with_label(Label::new((name, x.span()).into()).with_color(Color::Red))
                        .finish(),
                );
            }
            x
        })
        .map(move |x| {
            let tok = x.value();
            let sp = x.span();
            // Spans provided by logos will be valid except in extraordinary
            // circumstances.
            let s = source.get(Range::<usize>::from(sp)).unwrap();
            Spanned::new(sp, (*tok, s))
        })
}
