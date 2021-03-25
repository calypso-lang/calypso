//! Processed versions of [`crate::lexer::TokenType`] for use with LALRPOP.

use calypso_ast::expr::{Radix, Suffix};
use calypso_base::symbol::{kw::Keyword, PotentiallyInterned, Symbol};
use calypso_diagnostic::prelude::*;

use crate::lexer::{Token, TokenType};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tok<'tok> {
    /// Representable as [`crate::lexer::TokenType`]
    Unprocessed(TokenType),
    /// Number
    Number(&'tok str, Radix, Option<Suffix>),
    /// Identifier
    Ident(Symbol),
    /// Keyword
    Keyword(Keyword),
    /// String literal
    String(PotentiallyInterned<'tok>),
}

pub fn process<'tok>(tok: Token<'tok>) -> CalResult<Tok<'tok>> {
    Ok(match tok.value_owned() {
        (TokenType::Int { suffix, radix }, string) => Tok::Number(string, radix, suffix),
        (TokenType::Float, string) => Tok::Number(string, Radix::Decimal, None),
        (TokenType::Ident(symbol), _) => Tok::Ident(symbol),
        (TokenType::Keyword(symbol), _) => Tok::Keyword(Keyword::from(symbol)),
        (TokenType::String, string) => Tok::String(PotentiallyInterned::potentially_intern(string)),
        (tok, _) => Tok::Unprocessed(tok),
    })
}

pub fn process_iter<'tok>(
    iter: impl Iterator<Item = CalResult<Token<'tok>>> + 'tok,
    filter_ws: bool,
) -> impl Iterator<Item = CalResult<(usize, Tok<'tok>, usize)>> + 'tok {
    iter.map(|tok| {
        tok.map(|tok| process(tok).map(|t| (tok.span().lo(), t, tok.span().hi())))
            .and_then(std::convert::identity)
    })
    .filter(move |tok| {
        if filter_ws {
            !matches!(
                tok,
                Ok((_, Tok::Unprocessed(TokenType::Ws), _))
                    | Ok((_, Tok::Unprocessed(TokenType::BlockComment { .. }), _))
            )
        } else {
            true
        }
    })
}
