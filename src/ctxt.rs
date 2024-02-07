//! The global context of the Calypso compiler.

use std::cell::RefCell;

use crate::{
    ast::AstArenas,
    diagnostic::{sourcemgr::SourceCache, DiagReportCtxt},
    symbol::Symbol,
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
    /// Session
    pub session: RefCell<Session>,
}

#[derive(Default)]
pub struct Session {
    pub current_file: Option<Symbol>,
}

#[derive(Default)]
pub struct GlobalArenas {
    pub ast: AstArenas,
}
