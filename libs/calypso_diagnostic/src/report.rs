//! The global reporting context for diagnostics.

use crate::diagnostic::Diagnostic;

/// The global reporting context for diagnostics.
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
    /// Create a new `GlobalReportingCtxt`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            nonfatals: Vec::new(),
            fatal: None,
        }
    }

    /// Report an error that was synchronizable.
    pub fn report_syncd(&mut self, value: Diagnostic) {
        self.errors.push(value);
    }

    /// Report a non-fatal error.
    pub fn report_non_fatal(&mut self, value: Diagnostic) {
        self.nonfatals.push(value);
    }

    /// Report a fatal error. If there is already a fatal error reported, it
    /// will not be replaced.
    pub fn report_fatal(&mut self, value: Diagnostic) {
        if self.fatal.is_none() {
            self.fatal = Some(value);
        }
    }

    /// Get the list of nonfatal errors.
    #[must_use]
    pub fn nonfatals(&self) -> &[Diagnostic] {
        &self.nonfatals
    }

    /// Get the current fatal error, if any.
    #[must_use]
    pub fn fatal(&self) -> Option<&Diagnostic> {
        self.fatal.as_ref()
    }

    /// Get the list of synchronizable errors.
    #[must_use]
    pub fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }
}
