use crate::reporting::files::Error as DiagRenderError;
use thiserror::Error;

use super::diagnostic::Diagnostic;
use calypso_error::CalError;

/// An extension of [`CalError`] used for diagnostics.
#[derive(Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum DiagnosticError {
    #[error("failed to render diagnostic")]
    Rendering(#[from] DiagRenderError),
    #[error("{0}")]
    Diagnostic(Diagnostic),
}

impl From<DiagnosticError> for CalError {
    fn from(err: DiagnosticError) -> Self {
        CalError::Other(err.into())
    }
}

impl From<Diagnostic> for DiagnosticError {
    fn from(diagnostic: Diagnostic) -> Self {
        DiagnosticError::Diagnostic(diagnostic)
    }
}
