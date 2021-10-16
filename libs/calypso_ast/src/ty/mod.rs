use calypso_base::{span::Spanned, symbol::Symbol};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ty {
    Symbol(Spanned<Symbol>),
}
