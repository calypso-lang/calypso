use calypso_base::{span::Spanned, symbol::Symbol};

use crate::expr::Numeral;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
    Symbol(Spanned<Symbol>),
    IndefArray(Spanned<Box<Ty>>),
    DefArray(Spanned<Box<Ty>>, Spanned<Numeral>),
    Tuple(Vec<Spanned<Ty>>),
}
