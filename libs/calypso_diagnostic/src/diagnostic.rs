//! Pre-rendered diagnostics and builders to create them.

use calypso_base::ui::{termcolor::Buffer, Emitter};
use calypso_error::CalResult;

use super::reporting;
use reporting::files::SimpleFiles;

pub mod builder;

pub use builder::{Builder, EnsembleBuilder};
pub use reporting::diagnostic::{LabelStyle, Severity};

/// The structure used for managing source file names, IDs, and contents.
pub type SourceMgr = SimpleFiles<String, String>;

/// A pre-rendered diagnostic.
pub struct Diagnostic(Buffer);

impl Diagnostic {
    /// Emit this diagnostic now.
    ///
    /// # Errors
    ///
    /// This function will error if the diagnostic could not be emitted.
    pub fn emit(&self, stderr: &mut Emitter) -> CalResult<()> {
        stderr.emit(&self.0)?;
        Ok(())
    }
}

/// One or more diagnostics in a specific order, in order to form an "ensemble
/// diagnostic" which is emitted all at once.
#[allow(clippy::module_name_repetitions)]
pub enum EnsembleDiagnostic {
    /// One diagnostic
    One(Diagnostic),
    /// Many diagnostics
    Many(Vec<Diagnostic>),
}

impl EnsembleDiagnostic {
    /// Emit this ensemble diagnostic now.
    ///
    /// # Errors
    ///
    /// This function will error if the diagnostic could not be emitted.
    pub fn emit(&self, stderr: &mut Emitter) -> CalResult<()> {
        match self {
            Self::One(diag) => diag.emit(stderr),
            Self::Many(diags) => diags.iter().try_for_each(|diag| diag.emit(stderr)),
        }
    }
}

impl From<Diagnostic> for EnsembleDiagnostic {
    fn from(diag: Diagnostic) -> Self {
        Self::One(diag)
    }
}

impl From<Vec<Diagnostic>> for EnsembleDiagnostic {
    fn from(diags: Vec<Diagnostic>) -> Self {
        Self::Many(diags)
    }
}
