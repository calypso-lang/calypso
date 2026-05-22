use std::cell::RefCell;

use crate::{
    diag::{DiagReportCtxt, SourceCache},
    syntax::ast::AstArenas,
};

#[derive(Default, Debug)]
pub struct GlobalCtxt {
    pub arenas: Arenas,
    pub source_cache: RefCell<SourceCache>,
    pub diag: RefCell<DiagReportCtxt>,
}

impl GlobalCtxt {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&self) {
        self.arenas.clear();
        self.diag.borrow_mut().clear();
    }
}

#[derive(Default, Debug)]
pub struct Arenas {
    pub ast: AstArenas,
}

impl Arenas {
    pub fn clear(&self) {
        self.ast.clear();
    }
}
