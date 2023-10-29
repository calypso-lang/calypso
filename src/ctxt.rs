//! The global context of the Calypso compiler.

use parking_lot::RwLock;

use crate::{
    diagnostic::{sourcemgr::SourceCache, DiagReportCtxt},
    ui::Emitters,
};

/// The global context of the Calypso compiler. Usually held in a
/// [`std::sync::Arc`].
pub struct GlobalCtxt {
    /// Terminal emitters
    pub emit: RwLock<Emitters>,
    /// Global diagnostic reporting context
    pub diag: RwLock<DiagReportCtxt>,
    /// Source cache
    pub source_cache: RwLock<SourceCache>,
}
