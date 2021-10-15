use calypso_base::{span::Spanned, symbol::Symbol};

use crate::expr::Expr;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Let(Spanned<Symbol>, Spanned<Expr>),
}
