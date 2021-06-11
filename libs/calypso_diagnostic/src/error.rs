//! Extensions to [`calypso_error`] used for diagnostics.

use crate::reporting::files::Error as DiagRenderError;
use thiserror::Error;

use calypso_error::CalError;

/// An extension of [`CalError`] used for diagnostics.
#[derive(Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum DiagnosticError {
    /// Error while rendering a diagnostic
    #[error("failed to render diagnostic")]
    Rendering(#[from] DiagRenderError),
    /// A fatal error was reported within the global diagnostic reporting
    /// context.
    #[error(
        "internal diagnostic representation was printed incorrectly, please file a bug report"
    )]
    Diagnostic,
}

impl From<DiagnosticError> for CalError {
    fn from(err: DiagnosticError) -> Self {
        CalError::Other(err.into())
    }
}
