use std::{marker::PhantomData, ops::Index};

#[derive(Debug)]
pub struct Arena<Id: IdLike, T> {
    inner: Vec<T>,
    _phantom: PhantomData<Id>,
}

impl<Id: IdLike, T> Arena<Id, T> {
    pub fn new() -> Self {
        Self {
            inner: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn push(&mut self, x: T) -> Id {
        self.inner.push(x);
        Id::from_raw(self.inner.len() - 1)
    }
}

impl<Id: IdLike, T> Default for Arena<Id, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id: IdLike, T> Index<Id> for Arena<Id, T> {
    type Output = T;

    fn index(&self, index: Id) -> &Self::Output {
        &self.inner[index.into_raw()]
    }
}

pub trait IdLike {
    fn from_raw(index: usize) -> Self;
    fn into_raw(self) -> usize;
}
