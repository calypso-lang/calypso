//! Diagnostics and builders to create them.

use std::io::prelude::*;

use calypso_base::ui::termcolor::Buffer;
use calypso_error::CalResult;

use codespan_reporting::files::SimpleFiles;
use codespan_reporting::{
    diagnostic::Diagnostic as CodespanDiag,
    term::{self, Config},
};

use super::error::DiagnosticError;

pub mod builder;

pub use builder::{Builder, EnsembleBuilder};

pub use codespan_reporting::diagnostic::{LabelStyle, Severity};

/// The structure used for managing source file names, IDs, and contents.
pub type SourceMgr = SimpleFiles<String, String>;

/// A diagnostic.
pub struct Diagnostic(CodespanDiag<usize>);

impl Diagnostic {
    /// Render the diagnostic to the provided buffer. This will add a newline
    /// at the end.
    ///
    /// # Errors
    ///
    /// This function will error if rendering the diagnostic or writing to the
    /// buffer failed.
    pub fn render<'gcx>(
        &self,
        buf: &mut Buffer,
        sourcemgr: &'gcx SourceMgr,
        config: Option<&Config>,
    ) -> CalResult<()> {
        let def_config = Config::default();
        let config = config.unwrap_or(&def_config);

        term::emit(buf, &config, sourcemgr, &self.0).map_err(DiagnosticError::from)?;
        writeln!(buf)?;

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
    /// Render the ensemble diagnostic to the provided buffer. This will add a
    /// newline at the end.
    ///
    /// # Errors
    ///
    /// This function will error if rendering the diagnostic or writing to the
    /// buffer failed.
    pub fn render<'gcx>(
        &self,
        buf: &mut Buffer,
        sourcemgr: &'gcx SourceMgr,
        config: Option<&Config>,
    ) -> CalResult<()> {
        match self {
            Self::One(diag) => diag.render(buf, sourcemgr, config),
            Self::Many(diags) => diags
                .iter()
                .try_for_each(|diag| diag.render(buf, sourcemgr, config)),
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
