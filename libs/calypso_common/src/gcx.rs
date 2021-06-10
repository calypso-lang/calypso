use parking_lot::{Mutex, RwLock};

use calypso_base::ui::Emitters;
use calypso_diagnostic::{diagnostic::SourceMgr, report::GlobalReportingCtxt};

pub struct GlobalCtxt {
    pub emit: Mutex<Emitters>,
    pub grcx: RwLock<GlobalReportingCtxt>,
    pub sourcemgr: RwLock<SourceMgr>,
}
