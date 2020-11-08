use super::{reporting, FileMgr};
use calypso_base::span::Span;
use reporting::diagnostic::{Diagnostic as CodespanDiag, Label};
pub use reporting::diagnostic::{LabelStyle, Severity};
use reporting::term::{self, termcolor::Ansi};

use std::fmt;
use std::io::Cursor;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Diagnostic(CodespanDiag<usize>, Arc<FileMgr>);

#[derive(Clone, Debug)]
pub struct DiagnosticBuilder {
    level: Severity,
    code: Option<String>,
    message: String,
    labels: Vec<Label<usize>>,
    notes: Vec<String>,
    files: Arc<FileMgr>,
}

impl DiagnosticBuilder {
    pub fn new(level: Severity, files: Arc<FileMgr>) -> Self {
        Self {
            level,
            code: None,
            message: String::new(),
            labels: Vec::new(),
            notes: Vec::new(),
            files,
        }
    }

    pub fn code(&mut self, code: impl Into<String>) -> &mut Self {
        self.code = Some(code.into());
        self
    }

    pub fn message(&mut self, message: impl Into<String>) -> &mut Self {
        self.message = message.into();
        self
    }

    pub fn label(
        &mut self,
        style: LabelStyle,
        message: impl Into<String>,
        span: Span,
        file_id: usize,
    ) -> &mut Self {
        self.labels
            .push(Label::new(style, file_id, span).with_message(message));
        self
    }

    pub fn note(&mut self, message: impl Into<String>) -> &mut Self {
        self.notes.push(message.into());
        self
    }

    pub fn diag(&mut self, builder: impl Fn(&mut Self) -> &mut Self) -> &mut Self {
        builder(self)
    }

    pub fn build(&mut self) -> Diagnostic {
        let mut diagnostic = CodespanDiag::new(self.level);
        if let Some(code) = self.code.clone() {
            diagnostic = diagnostic.with_code(code)
        }
        if !self.message.is_empty() {
            diagnostic = diagnostic.with_message(self.message.clone())
        }
        Diagnostic(
            diagnostic
                .with_labels(self.labels.clone())
                .with_notes(self.notes.clone()),
            Arc::clone(&self.files),
        )
    }
}

impl Diagnostic {
    pub fn reason(&self) -> &str {
        &self.0.message
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let mut stream = Ansi::new(cursor);
        let config = term::Config::default();

        term::emit(&mut stream, &config, &*self.1, &self.0).map_err(|_| fmt::Error)?;
        let cursor = stream.into_inner();
        let buffer = cursor.into_inner();
        let data = std::str::from_utf8(&buffer).map_err(|_| fmt::Error)?;
        f.write_str(data)
    }
}
