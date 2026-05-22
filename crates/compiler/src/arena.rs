use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Index, IndexMut},
};

#[derive(Clone, Debug, PartialEq)]
pub struct Arena<Id: IdLike + Hash + Copy + Eq + PartialOrd, T> {
    inner: HashMap<Id, T>,
    next_id: Id,
}

impl<Id: IdLike + Hash + Copy + Eq + PartialOrd, T> Arena<Id, T> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            next_id: Id::from_raw(0),
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn push(&mut self, x: T) -> Id {
        let id = self.next_id;
        self.next_id = Id::from_raw(self.next_id.into_raw() + 1);
        self.inner.insert(id, x);
        id
    }

    pub fn insert(&mut self, id: Id, x: T) {
        self.inner.insert(id, x);
        if id >= self.next_id {
            self.next_id = Id::from_raw(self.next_id.into_raw() + 1);
        }
    }

    pub fn get(&self, id: Id) -> Option<&T> {
        self.inner.get(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Id, &T)> {
        self.inner.iter().map(|(i, v)| (*i, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Id, &mut T)> {
        self.inner.iter_mut().map(|(i, v)| (*i, v))
    }

    pub fn retain(&mut self, mut f: impl FnMut(Id, &T) -> bool) {
        self.inner.retain(|k, v| f(*k, v));
    }

    pub fn clear(&mut self) {
        self.inner.clear();
        self.next_id = Id::from_raw(0);
    }
}

impl<Id: IdLike + Hash + Copy + Eq + PartialOrd, T> Default for Arena<Id, T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Id: IdLike + Hash + Copy + Eq + PartialOrd, T> Index<Id> for Arena<Id, T> {
    type Output = T;

    fn index(&self, index: Id) -> &Self::Output {
        &self.inner[&index]
    }
}

impl<Id: IdLike + Hash + Copy + Eq + PartialOrd, T> IndexMut<Id> for Arena<Id, T> {
    fn index_mut(&mut self, index: Id) -> &mut Self::Output {
        self.inner.get_mut(&index).unwrap()
    }
}

pub trait IdLike {
    fn from_raw(index: usize) -> Self;
    fn into_raw(self) -> usize;
}

#[macro_export]
macro_rules! new_ast_ty {
    ($ty:ident, $data:ident, $arena:ident, $subarena:ident, $($param:ident: $paramty:ty),+) => {
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $ty(u32);

	#[derive(Clone, Debug, PartialEq, Eq)]
	pub struct $data {
	    $(pub $param: $paramty),+
	}

	impl $ty {
	    pub fn new(gcx: &GlobalCtxt, $($param: $paramty),+) -> Self {
		gcx.arenas.$arena.$subarena.borrow_mut().push($data { $($param),+ })
	    }

	    pub fn get(self, gcx: &GlobalCtxt) -> $data {
		gcx.arenas.$arena.$subarena.borrow()[self].clone()
	    }
	}

        impl IdLike for $ty {
            fn from_raw(index: usize) -> Self {
                Self(index as u32)
            }
            fn into_raw(self) -> usize {
                self.0 as usize
            }
        }
    };
}
