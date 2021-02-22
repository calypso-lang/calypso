pub struct Arena<T> {
    storage: Vec<T>,
}

impl<T> Arena<T> {
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
        }
    }

    pub fn populate(&mut self, items: Vec<T>) -> Vec<usize> {
        items.into_iter().map(|item| self.alloc(item)).collect()
    }

    pub fn alloc(&mut self, item: T) -> usize {
        self.storage.push(item);
        self.storage.len() - 1
    }

    pub fn get<'a>(&'a self, aref: usize) -> Option<&'a T> {
        self.storage.get(aref)
    }

    pub fn get_mut<'a>(&'a mut self, aref: usize) -> Option<&'a mut T> {
        self.storage.get_mut(aref)
    }
}
