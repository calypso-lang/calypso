use codespan_reporting::{
    diagnostic::{Diagnostic as CodespanDiag, Label, LabelStyle, Severity},
    term::{self, Config},
};

use calypso_base::{span::Span, ui::Emitter};
use calypso_error::CalResult;

use super::{Diagnostic, EnsembleDiagnostic, SourceMgr};
use crate::error::DiagnosticError;

#[derive(Debug, Default)]
#[allow(clippy::module_name_repetitions)]
pub struct EnsembleBuilder {
    diags: Vec<Builder>,
}

impl EnsembleBuilder {
    /// Create a new ensemble diagonstic builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add an diagnostic to the ensemble, using the builder provided.
    pub fn add(&mut self, severity: Severity, f: impl FnOnce(Builder) -> Builder) -> &mut Self {
        self.diags.push(f(Builder::new(severity)));
        self
    }

    /// Build the diagnostic. This pre-renders the diagnostic.
    ///
    /// # Errors
    ///
    /// As the diagnostic is pre-rendered, it returns an error if
    /// `codespan_reporting` fails to render it.
    ///
    /// # Panics
    ///
    /// This function should not panic.
    pub fn build(
        self,
        stderr: &Emitter,
        sourcemgr: &SourceMgr,
        config: Option<&Config>,
    ) -> CalResult<EnsembleDiagnostic> {
        let diags = self
            .diags
            .into_iter()
            .map(|b| b.build(stderr, sourcemgr, config))
            .collect::<CalResult<Vec<_>>>()?;

        if diags.len() == 1 {
            Ok(diags.into_iter().next().unwrap().into())
        } else {
            Ok(diags.into())
        }
    }
}

#[derive(Debug)]
pub struct Builder {
    diag: CodespanDiag<usize>,
    labels: Vec<Label<usize>>,
    notes: Vec<String>,
}

impl Builder {
    /// Create a new diagnostic builder.
    #[must_use]
    pub fn new(severity: Severity) -> Self {
        Self {
            diag: CodespanDiag::new(severity),
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Set the code of the diagnostic. This will overwrite whatever code was
    /// already present, if any.
    #[must_use]
    pub fn code(mut self, code: &'static str) -> Self {
        self.diag = self.diag.with_code(code);
        self
    }

    /// Set the short message of the diagnostic. This will overwrite whatever
    /// short message was already present, if any.
    #[must_use]
    pub fn short(mut self, short: impl Into<String>) -> Self {
        self.diag = self.diag.with_message(short.into());
        self
    }

    /// Add a note to the diagnostic.
    #[must_use]
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }

    /// Add a label to the diagnostic.
    #[must_use]
    pub fn label(
        mut self,
        style: LabelStyle,
        message: Option<impl Into<String>>,
        file_id: usize,
        span: Span,
    ) -> Self {
        let mut label = Label::new(style, file_id, span);
        if let Some(message) = message {
            label = label.with_message(message);
        }
        self.labels.push(label);
        self
    }

    /// Build the diagnostic. This pre-renders the diagnostic.
    ///
    /// # Errors
    ///
    /// As the diagnostic is pre-rendered, it returns an error if
    /// `codespan_reporting` fails to render it.
    pub fn build(
        mut self,
        stderr: &Emitter,
        sourcemgr: &SourceMgr,
        config: Option<&Config>,
    ) -> CalResult<Diagnostic> {
        self.diag = self.diag.with_labels(self.labels).with_notes(self.notes);

        let mut buf = stderr.buffer();
        let def_config = Config::default();
        let config = config.unwrap_or(&def_config);

        term::emit(&mut buf, &config, sourcemgr, &self.diag).map_err(DiagnosticError::from)?;

        Ok(Diagnostic(buf))
    }
}
