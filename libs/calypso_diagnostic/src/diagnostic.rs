use std::{fmt, sync::Arc};

use crate::prelude::DiagnosticError;

use super::{reporting, FileMgr};
use calypso_base::{session::BaseSession, span::Span};
use calypso_error::CalResult;
use reporting::diagnostic::{Diagnostic as CodespanDiag, Label};
use reporting::term::{self, termcolor::Buffer};

pub use reporting::diagnostic::{LabelStyle, Severity};

pub struct Diagnostic(CodespanDiag<usize>, Buffer, Arc<BaseSession>);

#[derive(Clone)]
pub struct Builder<'a> {
    level: Severity,
    code: Option<String>,
    message: String,
    labels: Vec<Label<usize>>,
    notes: Vec<String>,
    files: &'a FileMgr,
    sess: Arc<BaseSession>,
}

impl<'a> Builder<'a> {
    #[must_use]
    pub fn new(sess: Arc<BaseSession>, level: Severity, files: &'a FileMgr) -> Self {
        Self {
            level,
            code: None,
            message: String::new(),
            labels: Vec::new(),
            notes: Vec::new(),
            files,
            sess,
        }
    }

    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn label(
        mut self,
        style: LabelStyle,
        message: impl Into<String>,
        span: Span,
        file_id: usize,
    ) -> Self {
        self.labels
            .push(Label::new(style, file_id, span).with_message(message));
        self
    }

    pub fn note(mut self, message: impl Into<String>) -> Self {
        self.notes.push(message.into());
        self
    }

    /// Build the diagnostic. This pre-renders the diagnostic.
    ///
    /// # Errors
    /// As the diagnostic is pre-rendered, it returns an error if
    /// `codespan_reporting` fails to render it.
    pub fn build(self) -> CalResult<Diagnostic> {
        let mut diagnostic = CodespanDiag::new(self.level);
        if let Some(code) = self.code.clone() {
            diagnostic = diagnostic.with_code(code)
        }
        if !self.message.is_empty() {
            diagnostic = diagnostic.with_message(self.message.clone())
        }
        let diagnostic = diagnostic.with_labels(self.labels).with_notes(self.notes);
        let mut buf = self.sess.stderr.buffer();
        let config = term::Config::default();

        term::emit(&mut buf, &config, self.files, &diagnostic).map_err(DiagnosticError::from)?;
        Ok(Diagnostic(diagnostic, buf, self.sess))
    }
}

impl Diagnostic {
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.0.message
    }

    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.0.code.as_ref().map(AsRef::as_ref)
    }

    #[must_use]
    pub fn rendered(&self) -> &Buffer {
        &self.1
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.2.stderr.print(&self.1).map_err(|_| fmt::Error)?;
        Ok(())
    }
}

impl fmt::Debug for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Diagnostic").field(&self.0).finish()
    }
}
