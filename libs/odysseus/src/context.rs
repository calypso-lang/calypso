use std::collections::HashMap;

use super::module::{self, Module};

#[derive(Debug, Clone)]
pub struct Context {
    pub(crate) modules: HashMap<String, Module>,
}

impl Context {
    #[must_use]
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn module<'a>(&'a mut self, name: &str) -> module::Entry<'a> {
        if !self.modules.contains_key(name) {
            self.modules
                .insert(name.to_string(), Module::new(name.to_string()));
        }
        module::Entry::<'a> {
            ctx: self,
            name: name.to_string(),
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}
