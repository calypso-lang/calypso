//! IR for SaturnVM. Very WIP and subject to change.

pub mod module;

use std::{
    cell::{RefCell, RefMut},
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use calypso_base::symbol::Symbol;

use module::Module;

#[derive(Debug)]
pub struct Context {
    modules: RefCell<HashMap<Symbol, Module>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            modules: RefCell::new(HashMap::new()),
        }
    }

    pub fn module<'a>(&'a self, name: Symbol) -> Cursor<'a, Module, Self> {
        let module = RefMut::map(self.modules.borrow_mut(), |m| {
            m.entry(name).or_insert_with(|| Module::new())
        });

        Cursor::new(module, self)
    }
}

#[derive(Debug)]
pub struct Cursor<'a, T, P> {
    target: RefMut<'a, T>,
    parent: &'a P,
}

impl<'a, T, P> Cursor<'a, T, P> {
    pub(crate) fn new(target: RefMut<'a, T>, parent: &'a P) -> Self {
        Self { target, parent }
    }
}

impl<'a, T, P> Deref for Cursor<'a, T, P> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.target
    }
}

impl<'a, T, P> DerefMut for Cursor<'a, T, P> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.target
    }
}
