//! The global reporting context for diagnostics.
use std::fmt;

use ariadne::{Cache, Source};
use rustc_hash::FxHashMap;

use crate::{ctxt::GlobalCtxt, symbol::Symbol, syntax::span::Span};

pub type Diagnostic = ariadne::Report<Span>;

/// The global reporting context for diagnostics.
#[derive(Default)]
pub struct DiagReportCtxt {
    errors: Vec<Diagnostic>,
    nonfatals: Vec<Diagnostic>,
    fatal: Option<Diagnostic>,
}

impl fmt::Debug for DiagReportCtxt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DiagReportCtxt").finish_non_exhaustive()
    }
}

impl GlobalCtxt {
    /// Print and clear the list of errors, returning false if there
    /// was a fatal error.
    pub fn flush_diag(&self) -> eyre::Result<bool> {
        let diag_read = self.diag.borrow();
        if let Some(fatal) = diag_read.fatal() {
            let mut cache = self.source_cache.borrow_mut();
            fatal.eprint(&mut *cache)?;
            Ok(false)
        } else {
            diag_read
                .errors()
                .iter()
                .try_for_each(|e| -> eyre::Result<()> {
                    let mut cache = self.source_cache.borrow_mut();
                    e.eprint(&mut *cache)?;
                    Ok(())
                })?;
            Ok(true)
        }
    }
}

impl DiagReportCtxt {
    /// Clear the list of synchronized errors.
    pub fn clear_syncd(&mut self) {
        self.errors.clear();
    }

    /// Clear the list of nonfatals.
    pub fn clear_nonfatals(&mut self) {
        self.nonfatals.clear();
    }

    /// Clear the current fatal error.
    pub fn clear_fatal(&mut self) {
        self.fatal = None;
    }

    /// Clear the entire reporting context
    pub fn clear(&mut self) {
        self.clear_fatal();
        self.clear_nonfatals();
        self.clear_syncd();
    }

    /// Report an error that was synchronizable.
    pub fn report_syncd(&mut self, value: Diagnostic) {
        self.errors.push(value);
    }

    /// Report a non-fatal error.
    pub fn report_non_fatal(&mut self, value: Diagnostic) {
        self.nonfatals.push(value);
    }

    /// Report a fatal error. If there is already a fatal error reported, it
    /// will not be replaced.
    pub fn report_fatal(&mut self, value: Diagnostic) {
        if self.fatal.is_none() {
            self.fatal = Some(value);
        }
    }

    /// Get the list of nonfatal errors.
    pub fn nonfatals(&self) -> &[Diagnostic] {
        &self.nonfatals
    }

    /// Get the current fatal error, if any.
    pub fn fatal(&self) -> Option<&Diagnostic> {
        self.fatal.as_ref()
    }

    /// Get the list of synchronizable errors.
    pub fn errors(&self) -> &[Diagnostic] {
        &self.errors
    }
}

pub struct SourceCache {
    inner: FxHashMap<Symbol, Source<String>>,
    empty_source: Source<String>,
}

impl Default for SourceCache {
    fn default() -> Self {
        Self {
            inner: FxHashMap::default(),
            empty_source: Source::from(String::new()),
        }
    }
}

impl fmt::Debug for SourceCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SourceCache").finish_non_exhaustive()
    }
}

impl SourceCache {
    pub fn add(&mut self, name: Symbol, contents: String) {
        self.inner.insert(name, Source::from(contents));
    }

    pub fn get(&self, id: Symbol) -> Option<&Source> {
        if id.as_str().is_empty() {
            return Some(&self.empty_source);
        }
        self.inner.get(&id)
    }
}

impl Cache<Symbol> for SourceCache {
    type Storage = String;

    fn fetch(&mut self, id: &Symbol) -> Result<&Source, impl fmt::Debug> {
        self.inner.get(id).ok_or(*id)
    }

    fn display<'a>(&self, id: &'a Symbol) -> Option<impl fmt::Display + 'a> {
        Some(*id)
    }
}
