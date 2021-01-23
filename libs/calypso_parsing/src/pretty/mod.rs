use calypso_diagnostic::FileMgr;

pub mod lexer;

// note(@ThePuzzlemaker: frame): This may be changed into a visitor API once I get an AST working.
pub struct Printer<'a> {
    file_id: usize,
    fmgr: &'a FileMgr,
}

impl<'a> Printer<'a> {
    #[must_use]
    pub fn new(file_id: usize, fmgr: &'a FileMgr) -> Self {
        Self { file_id, fmgr }
    }
}
