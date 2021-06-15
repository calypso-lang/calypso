use std::sync::Arc;

use calypso_common::gcx::GlobalCtxt;

pub mod lexer;

// note(@ThePuzzlemaker: frame): This may be changed into a visitor API once I get an AST working.
pub struct Printer {
    file_id: usize,
    gcx: Arc<GlobalCtxt>,
}

impl Printer {
    #[must_use]
    pub fn new(file_id: usize, gcx: Arc<GlobalCtxt>) -> Self {
        Self { file_id, gcx }
    }
}
