use super::context::Context;
use super::traits::{Container, Entry};

#[derive(Debug, Clone)]
pub struct Module {
    pub(crate) name: String,
}

impl Module {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

impl Container for Module {
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
    type Container = Module;

    fn id(&self) -> usize {
        self.id
    }
}

impl ModuleEntry {
    pub fn new(id: usize) -> Self {
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
