//! The root [`Parent`] object for SaturnVM bytecode.

use super::module::{Module, ModuleBuilder, ModuleEntry};
use super::traits::Parent;

#[derive(Debug, Clone)]
pub struct Context {
    pub(crate) modules: Vec<(bool, Module)>,
}

impl Parent<Module> for Context {
    fn get(&self, id: usize) -> Option<&Module> {
        self.modules.get(id).map(|(_, m)| m)
    }

    fn is_finished(&self, id: usize) -> bool {
        *self.modules.get(id).map_or(&false, |(b, _)| b)
    }

    fn finish(&mut self, id: usize) -> &Module {
        *self.modules.get_mut(id).map(|(b, _)| b).unwrap() = true;
        self.modules.get(id).map(|(_, m)| m).unwrap()
    }

    fn create_builder(&mut self, id: usize) -> ModuleBuilder {
        ModuleBuilder::new(id)
    }
}

impl Context {
    #[must_use]
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
        }
    }

    pub fn module(&mut self, name: &str) -> ModuleEntry {
        let pos =
            self.modules.iter().enumerate().find_map(
                |(pos, (_, m))| {
                    if m.name == name {
                        Some(pos)
                    } else {
                        None
                    }
                },
            );
        pos.map_or_else(
            || {
                self.modules.push((false, Module::new(name.to_string())));
                ModuleEntry::new(self.modules.len() - 1)
            },
            ModuleEntry::new,
        )
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
