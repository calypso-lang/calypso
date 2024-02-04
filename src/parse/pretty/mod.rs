use crate::ctxt::GlobalCtxt;

pub mod ast;
pub mod lexer;

// note(@ThePuzzlemaker: frame): This may be changed into a visitor API once I get an AST working.
pub struct Printer<'gcx> {
    gcx: &'gcx GlobalCtxt,
}

impl<'gcx> Printer<'gcx> {
    #[must_use]
    pub fn new(gcx: &'gcx GlobalCtxt) -> Self {
        Self { gcx }
    }
}
