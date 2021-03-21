//! Processed versions of [`crate::lexer::TokenType`] for use with LALRPOP.

use calypso_diagnostic::prelude::*;

use crate::lexer::{Suffix, Token, TokenType};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tok<'tok> {
    /// Representable as [`crate::lexer::TokenType`]
    Unprocessed(TokenType),
    /// Signed integer literal
    Sint(i64),
    /// Unsigned integer literal
    Uint(u64),
    /// Float literal
    Float(f64),
    /// Boolean
    Bool(bool),
    /// Identifier
    Ident(&'tok str),
    /// String literal
    String(&'tok str),
}

pub fn process<'tok>(tok: Token<'tok>) -> CalResult<Tok<'tok>> {
    Ok(match tok.value_owned() {
        (TokenType::Int { suffix, radix }, string) => match suffix {
            Some(Suffix::Float) => Tok::Float(
                string
                    .replace("_", "")
                    .parse::<f64>()
                    .map_err(anyhow::Error::from)?,
            ),
            Some(Suffix::Sint) => Tok::Sint(
                i64::from_str_radix(&string.replace("_", ""), radix.radix())
                    .map_err(anyhow::Error::from)?,
            ),
            Some(Suffix::Uint) => Tok::Uint(
                u64::from_str_radix(&string.replace("_", ""), radix.radix())
                    .map_err(anyhow::Error::from)?,
            ),
            None => {
                let as_u64 = u64::from_str_radix(&string.replace("_", ""), radix.radix())
                    .map_err(anyhow::Error::from)?;
                if as_u64 <= i64::MAX as u64 {
                    Tok::Sint(as_u64 as i64)
                } else {
                    Tok::Uint(as_u64)
                }
            }
            _ => unreachable!(),
        },
        (TokenType::Float, string) => Tok::Float(
            string
                .replace("_", "")
                .parse::<f64>()
                .map_err(anyhow::Error::from)?,
        ),
        (TokenType::Ident, string) => Tok::Ident(string),
        (TokenType::String, string) => Tok::String(string),
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
