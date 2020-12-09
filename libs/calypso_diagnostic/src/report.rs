use super::diagnostic::Diagnostic;

use std::ops::{Deref, DerefMut};

#[derive(Clone, Debug, Default)]
/// An ordered group of diagnostics. This is basically
/// currently just a wrapper over `Vec<Diagnostic>`.
pub struct Report {
    diagnostics: Vec<Diagnostic>,
}

impl Report {
    pub fn new() -> Self {
        Self { ..Self::default() }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) -> &mut Self {
        self.diagnostics.push(diagnostic);
        self
    }
}

impl AsRef<[Diagnostic]> for Report {
    fn as_ref(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}

impl Deref for Report {
    type Target = Vec<Diagnostic>;
    fn deref(&self) -> &Self::Target {
        &self.diagnostics
    }
}

impl DerefMut for Report {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.diagnostics
    }
}

impl From<Report> for Vec<Diagnostic> {
    fn from(report: Report) -> Self {
        report.diagnostics
    }
}

impl From<Vec<Diagnostic>> for Report {
    fn from(diagnostics: Vec<Diagnostic>) -> Self {
        Self { diagnostics }
    }
}
