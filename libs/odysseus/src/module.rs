use super::context::Context;

#[derive(Debug, Clone)]
pub struct Module {
    name: String,
    finished: bool,
}

impl Module {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            finished: false,
        }
    }
}

#[derive(Debug)]
pub struct Entry<'a> {
    pub(crate) ctx: &'a mut Context,
    pub(crate) name: String,
}

impl<'a> Entry<'a> {
    #[must_use]
    pub fn get(&self) -> &Module {
        let module = self.ctx.modules.get(&self.name).unwrap();
        assert!(module.finished, "cannot .get() an unfinished module");
        module
    }

    pub fn enter(&mut self, f: impl FnOnce(Builder)) -> &mut Self {
        let module = self.ctx.modules.get_mut(&self.name).unwrap();
        assert!(!module.finished, "cannot .enter() a finished module");
        f(Builder::new(module));
        self
    }

    #[must_use]
    pub fn build(&mut self) -> &Module {
        let module = self.ctx.modules.get_mut(&self.name).unwrap();
        assert!(!module.finished, "cannot .build() a finished module");
        module.finished = true;
        module
    }
}

#[derive(Debug)]
pub struct Builder<'a> {
    module: &'a mut Module,
}

impl<'a> Builder<'a> {
    pub fn new(module: &'a mut Module) -> Self {
        Self { module }
    }
}
