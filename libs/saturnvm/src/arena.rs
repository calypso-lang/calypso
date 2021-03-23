//! A simple arena for use within SaturnVM, built with a `Vec`, using `usize`s
//! for IDs.

/// A simple arena for use within SaturnVM, built with a `Vec`, using `usize`s
/// for IDs.
pub struct Arena<T> {
    storage: Vec<T>,
}

impl<T> Default for Arena<T> {
    fn default() -> Self {
        Self {
            storage: Vec::default(),
        }
    }
}

impl<T> Arena<T> {
    /// Create a new arena with no allocated storage.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Populate the arena with the list of items. The list returned contains
    /// the IDs of the respective indexes of elements.
    pub fn populate(&mut self, items: Vec<T>) -> Vec<usize> {
        items.into_iter().map(|item| self.alloc(item)).collect()
    }

    /// Allocate an item and return its ID.
    #[must_use]
    pub fn alloc(&mut self, item: T) -> usize {
        self.storage.push(item);
        self.storage.len() - 1
    }

    /// Get a reference to an item in the arena by its ID.
    #[must_use]
    pub fn get(&self, aref: usize) -> Option<&T> {
        self.storage.get(aref)
    }

    /// Get a mutable reference to an item in the arena by its ID.
    #[must_use]
    pub fn get_mut(&mut self, aref: usize) -> Option<&mut T> {
        self.storage.get_mut(aref)
    }
}
