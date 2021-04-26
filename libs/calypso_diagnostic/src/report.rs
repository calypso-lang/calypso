use std::sync::Arc;

use calypso_base::session::BaseSession;

use crate::diagnostic::Diagnostic;

pub struct GlobalReportingCtxt {
    errors: Vec<Diagnostic>,
    nonfatals: Vec<Diagnostic>,
    sess: Arc<BaseSession>,
}

impl GlobalReportingCtxt {
    #[must_use]
    pub fn new(sess: Arc<BaseSession>) -> Self {
        Self {
            errors: Vec::new(),
            nonfatals: Vec::new(),
            sess,
        }
    }

    pub fn report_syncd(&mut self, value: Diagnostic) {
        self.errors.push(value);
    }

    pub fn report_non_fatal(&mut self, value: Diagnostic) {
        self.nonfatals.push(value);
    }

    #[must_use]
    pub fn nonfatals(&self) -> &[Diagnostic] {
        &self.nonfatals
    }

    #[must_use]
    pub fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }

    #[must_use]
    pub fn sess(&self) -> &Arc<BaseSession> {
        &self.sess
    }
}
