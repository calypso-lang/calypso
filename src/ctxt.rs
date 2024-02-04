//! The global context of the Calypso compiler.

use std::cell::RefCell;

use crate::{
    ast::AstArenas,
    diagnostic::{sourcemgr::SourceCache, DiagReportCtxt},
    ui::Emitters,
};

/// The global context of the Calypso compiler.
pub struct GlobalCtxt {
    /// Terminal emitters
    pub emit: RefCell<Emitters>,
    /// Global diagnostic reporting context
    pub diag: RefCell<DiagReportCtxt>,
    /// Source cache
    pub source_cache: RefCell<SourceCache>,
    /// Arenas
    pub arenas: GlobalArenas,
}

#[derive(Default)]
pub struct GlobalArenas {
    pub ast: AstArenas,
}
