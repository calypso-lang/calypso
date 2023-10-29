use std::sync::Arc;

use crate::ctxt::GlobalCtxt;

pub mod lexer;

// note(@ThePuzzlemaker: frame): This may be changed into a visitor API once I get an AST working.
pub struct Printer {
    gcx: Arc<GlobalCtxt>,
}

impl Printer {
    #[must_use]
    pub fn new(gcx: Arc<GlobalCtxt>) -> Self {
        Self { gcx }
    }
}
