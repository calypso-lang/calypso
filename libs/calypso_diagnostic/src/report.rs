use crate::diagnostic::Diagnostic;

#[derive(Clone, Debug, Default)]
pub struct GlobalReportingCtxt {
    errors: Vec<Diagnostic>,
    nonfatals: Vec<Diagnostic>,
}

impl GlobalReportingCtxt {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
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
}
