//! The global context of the Calypso compiler.

use parking_lot::{Mutex, RwLock};

use calypso_base::ui::Emitters;
use calypso_diagnostic::{diagnostic::SourceMgr, report::GlobalReportingCtxt};

/// The global context of the Calypso compiler. Usually held in a
/// [`std::sync::Arc`].
pub struct GlobalCtxt {
    /// Terminal emitters
    pub emit: Mutex<Emitters>,
    /// Global diagnostic reporting context
    pub grcx: RwLock<GlobalReportingCtxt>,
    /// Source code manager used within diagnostics
    pub sourcemgr: RwLock<SourceMgr>,
}
