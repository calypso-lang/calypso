use std::{collections::HashMap, fmt};

use ariadne::{Cache, Source};

use crate::symbol::Symbol;

#[derive(Default)]
pub struct SourceCache {
    inner: HashMap<Symbol, Source>,
}

impl fmt::Debug for SourceCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: make this better
        f.debug_struct("SourceMgr").finish_non_exhaustive()
    }
}

impl SourceCache {
    pub fn add(&mut self, name: Symbol, contents: &str) {
        self.inner.insert(name, Source::from(contents));
    }

    pub fn get(&self, id: Symbol) -> Option<&Source> {
        self.inner.get(&id)
    }
}

impl Cache<Symbol> for SourceCache {
    fn fetch(&mut self, id: &Symbol) -> Result<&Source, Box<dyn fmt::Debug>> {
        self.inner.get(id).ok_or_else(|| Box::new(*id) as _)
    }

    fn display(&self, id: &Symbol) -> Option<Box<dyn fmt::Display>> {
        Some(Box::new(*id) as _)
    }
}
