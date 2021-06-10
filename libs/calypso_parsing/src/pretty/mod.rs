use std::sync::Arc;

use calypso_base::symbol::Symbol;

use crate::session::ParseSess;

pub mod lexer;

// note(@ThePuzzlemaker: frame): This may be changed into a visitor API once I get an AST working.
pub struct Printer {
    file_id: Symbol,
    sess: Arc<ParseSess>,
}

impl Printer {
    #[must_use]
    pub fn new(file_id: Symbol, sess: Arc<ParseSess>) -> Self {
        Self { file_id, sess }
    }
}
