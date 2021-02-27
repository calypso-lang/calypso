//! A bytecode [`Element`] corresponding to a single module, which in the case
//! of bytecode is a single compilation unit. Submodules are considered
//! separate compilation units.

#![allow(clippy::clippy::module_name_repetitions)]

use super::context::Context;
use super::traits::{Element, Entry};

#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) name: String,
}

impl Module {
    #[must_use]
    pub(crate) fn new(name: String) -> Self {
        Self { name }
    }
}

impl Element for Module {
    type Entry = ModuleEntry;
    type Builder = ModuleBuilder;
    type Id = usize;
}

#[derive(Debug)]
pub struct ModuleEntry {
    pub(crate) id: usize,
}

impl Entry for ModuleEntry {
    type Parent = Context;
    type Element = Module;

    fn id(&self) -> usize {
        self.id
    }

    fn internal_build(&mut self, _ctx: &mut Context, _b: ModuleBuilder) {}
}

impl ModuleEntry {
    #[must_use]
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }
}

#[derive(Debug)]
pub struct ModuleBuilder {
    pub(crate) id: usize,
}

impl ModuleBuilder {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }
}
