//! Builders for diagnostics and ensemble diagnostics.

use codespan_reporting::diagnostic::{Diagnostic as CodespanDiag, Label, LabelStyle, Severity};

use calypso_base::span::Span;

use super::{Diagnostic, EnsembleDiagnostic};

/// A builder for an ensemble diagnostic.
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

    /// Add a diagnostic to the ensemble, using the builder provided.
    pub fn add(mut self, severity: Severity, f: impl FnOnce(Builder) -> Builder) -> Self {
        self.diags.push(f(Builder::new(severity)));
        self
    }

    /// Add a diagnostic to the ensemble, using the builder provided and the
    /// severity [`Severity::Bug`].
    pub fn bug(self, f: impl FnOnce(Builder) -> Builder) -> Self {
        self.add(Severity::Bug, f)
    }

    /// Add a diagnostic to the ensemble, using the builder provided and the
    /// severity [`Severity::Error`].
    pub fn error(self, f: impl FnOnce(Builder) -> Builder) -> Self {
        self.add(Severity::Error, f)
    }

    /// Add a diagnostic to the ensemble, using the builder provided and the
    /// severity [`Severity::Warning`].
    pub fn warning(self, f: impl FnOnce(Builder) -> Builder) -> Self {
        self.add(Severity::Warning, f)
    }

    /// Add a diagnostic to the ensemble, using the builder provided and the
    /// severity [`Severity::Note`].
    pub fn note(self, f: impl FnOnce(Builder) -> Builder) -> Self {
        self.add(Severity::Note, f)
    }

    /// Add a diagnostic to the ensemble, using the builder provided and the
    /// severity [`Severity::Help`].
    pub fn help(self, f: impl FnOnce(Builder) -> Builder) -> Self {
        self.add(Severity::Help, f)
    }

    /// Build the diagnostic.
    // This function won't panic.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn build(self) -> EnsembleDiagnostic {
        let diags = self
            .diags
            .into_iter()
            .map(Builder::build)
            .collect::<Vec<_>>();

        if diags.len() == 1 {
            diags.into_iter().next().unwrap().into()
        } else {
            diags.into()
        }
    }
}

/// A builder for a single diagnostic.
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
        message: Option<&str>,
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

    /// Build the diagnostic.
    #[must_use]
    pub fn build(self) -> Diagnostic {
        Diagnostic(self.diag.with_labels(self.labels).with_notes(self.notes))
    }
}
