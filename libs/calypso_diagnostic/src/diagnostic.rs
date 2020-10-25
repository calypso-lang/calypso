use std::fmt;

use csr::diagnostic::{Diagnostic as CSDiagnostic, Severity};
use csr::files::SimpleFiles as CSRSimpleFiles;
use csr::term::{self, termcolor::Ansi};

pub extern crate codespan_reporting as csr;

pub type SimpleFiles = CSRSimpleFiles<String, String>;

use std::io::Cursor;

#[derive(Clone, Debug)]
pub struct Diagnostic {
    diagnostic: CSDiagnostic<usize>,
    files: SimpleFiles,
}

impl Diagnostic {
    pub fn new(diagnostic: CSDiagnostic<usize>, files: SimpleFiles) -> Self {
        Self { diagnostic, files }
    }

    pub fn reason(&self) -> &str {
        &self.diagnostic.message
    }

    pub fn severity(&self) -> Severity {
        self.diagnostic.severity
    }

    pub fn diagnostic(&self) -> &CSDiagnostic<usize> {
        &self.diagnostic
    }

    pub fn files(&self) -> &SimpleFiles {
        &self.files
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let buffer = Vec::new();
        let cursor = Cursor::new(buffer);
        let mut stream = Ansi::new(cursor);
        let config = term::Config::default();

        term::emit(&mut stream, &config, &self.files, &self.diagnostic).map_err(|_| fmt::Error)?;
        let cursor = stream.into_inner();
        let buffer = cursor.into_inner();
        let data = std::str::from_utf8(&buffer).map_err(|_| fmt::Error)?;
        f.write_str(data)
    }
}
