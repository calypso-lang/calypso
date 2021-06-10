use crate::diagnostic::Diagnostic;

// TODO(@ThePuzzlemaker: frame|diag):
//   rewrite nonfatals as a better "lint" system
pub struct GlobalReportingCtxt {
    errors: Vec<Diagnostic>,
    nonfatals: Vec<Diagnostic>,
    fatal: Option<Diagnostic>,
}

impl Default for GlobalReportingCtxt {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalReportingCtxt {
    #[must_use]
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            nonfatals: Vec::new(),
            fatal: None,
        }
    }

    pub fn report_syncd(&mut self, value: Diagnostic) {
        self.errors.push(value);
    }

    pub fn report_non_fatal(&mut self, value: Diagnostic) {
        self.nonfatals.push(value);
    }

    pub fn report_fatal(&mut self, value: Diagnostic) {
        if self.fatal.is_none() {
            self.fatal = Some(value);
        }
    }

    #[must_use]
    pub fn nonfatals(&self) -> &[Diagnostic] {
        &self.nonfatals
    }

    #[must_use]
    pub fn fatal(&self) -> Option<&Diagnostic> {
        self.fatal.as_ref()
    }

    #[must_use]
    pub fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }
}
