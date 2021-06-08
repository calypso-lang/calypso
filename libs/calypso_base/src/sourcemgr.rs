use std::ops::Range;

use codespan_reporting::files::{self, Error as FilesError, Files};
use dashmap::{mapref::one::Ref, DashMap};

use crate::symbol::Symbol;

#[derive(Clone, Debug, Default)]
pub struct SourceMgr {
    map: DashMap<Symbol, (String, Vec<usize>)>,
}

/// Implementation detail. You can simply use the `AsRef<str>` implementation.
pub struct SourceRef<'a>(Ref<'a, Symbol, (String, Vec<usize>)>);

impl<'a> AsRef<str> for SourceRef<'a> {
    fn as_ref(&self) -> &str {
        self.0 .0.as_str()
    }
}

impl SourceMgr {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new file to the database. This will update the file's source if the file is already in the database.
    pub fn add(&self, name: Symbol, source: String) {
        if self.map.contains_key(&name) {
            self.update(name, source);
            return;
        }

        let starts = files::line_starts(&source).collect();
        self.map.insert(name, (source, starts));
    }

    fn update(&self, name: Symbol, source: String) {
        let file = self.map.get(&name).unwrap();
        let old_source = file.0.as_str();
        if old_source == source {
            return;
        }

        let starts = files::line_starts(&source).collect();
        self.map.insert(name, (source, starts));
    }

    #[must_use]
    pub fn intern_add(&self, name: &str, source: String) -> Symbol {
        let sym = Symbol::intern(name);
        self.add(sym, source);
        sym
    }
}

impl<'a> Files<'a> for SourceMgr {
    type FileId = Symbol;
    type Name = &'static str;
    type Source = SourceRef<'a>;

    fn name(&self, id: Self::FileId) -> Result<Self::Name, FilesError> {
        Ok(id.as_str())
    }

    fn source(&'a self, id: Self::FileId) -> Result<Self::Source, FilesError> {
        self.map
            .get(&id)
            .map(SourceRef)
            .ok_or(FilesError::FileMissing)
    }

    fn line_index(&'a self, id: Self::FileId, byte_index: usize) -> Result<usize, FilesError> {
        let file = self.map.get(&id).ok_or(FilesError::FileMissing)?;
        let starts = file.1.as_slice();

        match starts.binary_search(&byte_index) {
            Ok(line) => Ok(line),
            Err(next_line) => Ok(next_line - 1),
        }
    }

    fn line_range(
        &'a self,
        id: Self::FileId,
        line_index: usize,
    ) -> Result<Range<usize>, FilesError> {
        let file = self.map.get(&id).ok_or(FilesError::FileMissing)?;
        let source = file.0.as_str();
        let starts = file.1.as_slice();

        let len = starts.len();
        if line_index >= len {
            return Err(FilesError::LineTooLarge {
                given: line_index,
                max: len - 1,
            });
        }

        let line_start = starts[line_index];
        let line_end = if line_index + 1 >= len {
            source.len() - 1
        } else {
            starts[line_index + 1] - 1
        };

        Ok(line_start..line_end)
    }
}
