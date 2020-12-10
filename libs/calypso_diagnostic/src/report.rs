use crate::diagnostic::Diagnostic;

#[derive(Clone, Debug, Default)]
pub struct GlobalReportingCtxt {
    errors: Vec<SyncState>,
    nonfatals: Vec<Diagnostic>,
    panic: bool,
}

impl GlobalReportingCtxt {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn report_fatal(&mut self, value: Diagnostic) {
        self.errors.push(Err(value));
    }

    pub fn report_syncd(&mut self, value: Diagnostic) {
        self.errors.push(Ok(value));
    }

    pub fn report_non_fatal(&mut self, value: Diagnostic) {
        self.nonfatals.push(value);
    }

    pub fn nonfatals(&self) -> &[Diagnostic] {
        &self.nonfatals
    }

    pub fn errors(&self) -> &[SyncState] {
        &self.errors
    }
}

/// A synchronization state. `Ok(Diagnostic)` means
/// synchronization was required and `Err(Diagnostic)`
/// means that it was a fatal error.
type SyncState = Result<Diagnostic, Diagnostic>;
