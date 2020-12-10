use crate::diagnostic::Diagnostic;

#[derive(Clone, Debug, Default)]
pub struct GlobalReportingCtxt {
    errors: Vec<Diagnostic>,
    nonfatals: Vec<Diagnostic>,
}

impl GlobalReportingCtxt {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn report_syncd(&mut self, value: Diagnostic) {
        self.errors.push(value);
    }

    pub fn report_non_fatal(&mut self, value: Diagnostic) {
        self.nonfatals.push(value);
    }

    pub fn nonfatals(&self) -> &[Diagnostic] {
        &self.nonfatals
    }

    pub fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }
}
