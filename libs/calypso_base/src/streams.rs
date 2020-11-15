/// A trait for streamed data.
pub trait Stream: Iterator {
    type Elem;

    /// This stream can no longer produce elements if this returns true
    fn is_at_end(&self) -> bool;

    /// Peek at the value one element ahead, without consuming it.
    fn peek(&self) -> Option<&Self::Elem>;
    /// Peek at the value two element ahead, without consuming anything.
    fn peek2(&self) -> Option<&Self::Elem>;
    /// Peek at the value three element ahead, without consuming anything.
    fn peek3(&self) -> Option<&Self::Elem>;
    /// Peek at the value 1 element behind, without moving backwards.
    fn prev(&self) -> Option<&Self::Elem>;

    /// Consume the next element if the condition is true.
    ///
    /// If `func` returns true for the element, consume and return it.
    /// Otherwise, return `None`.
    fn next_if(&mut self, func: impl FnOnce(&Self::Elem) -> bool) -> Option<Self::Elem>;

    /// Consume the next element if it is equal to `expected`.
    fn next_if_eq<R>(&mut self, expected: &R) -> Option<Self::Elem>
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>;

    /// Keep consuming elements while the condition is true
    /// and there are remaining elements.
    ///
    /// If `func` returns true for the element, consume it.
    /// Repeat until `func` returns false for an element.
    ///
    /// `func` takes in the element first, then the number of
    /// elements consumed so far.
    fn gorge_while(&mut self, func: impl FnMut(&Self::Elem, usize) -> bool);

    /// Keep consuming elements while the element is equal to `expected`
    /// and there are remaining elements.
    fn gorge_while_eq<R>(&mut self, expected: &R)
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>;

    /// Returns `true` if the value one element ahead is equal to `expected`.
    /// Otherwise, return false. In neither case should any values be consumed.
    fn peek_eq<R>(&self, expected: &R) -> bool
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>;

    /// Returns `true` if the value two elements ahead is equal to `expected`.
    /// Otherwise, return false. In neither case should values be consumed.
    fn peek2_eq<R>(&self, expected: &R) -> bool
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>;

    /// Returns `true` if the value three elements ahead is equal to `expected`.
    /// Otherwise, return false. In neither case should values be consumed.
    fn peek3_eq<R>(&self, expected: &R) -> bool
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>;

    /// Returns `true` if the value one element behind is equal to `expected`.
    /// Otherwise, return false. In neither case should the cursor move backward.
    fn prev_eq<R>(&self, expected: &R) -> bool
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>;

    /// Returns `true` if the condition is true. Otherwise, return false.
    /// In neither case should any values be consumed.
    ///
    /// `func` takes in the value one element ahead.
    fn peek_cond(&self, func: impl FnOnce(&Self::Elem) -> bool) -> bool;

    /// Returns `true` if the condition is true. Otherwise, return false.
    /// In neither case should any values be consumed.
    ///
    /// `func` takes in the value one element ahead.
    fn peek2_cond(&self, func: impl FnOnce(&Self::Elem) -> bool) -> bool;

    /// Returns `true` if the condition is true. Otherwise, return false.
    /// In neither case should any values be consumed.
    ///
    /// `func` takes in the value one element ahead.
    fn peek3_cond(&self, func: impl FnOnce(&Self::Elem) -> bool) -> bool;

    /// Returns `true` if the condition is true. Otherwise, return false.
    /// In neither case should any values be consumed.
    ///
    /// `func` takes in the value one element ahead.
    fn prev_cond(&self, func: impl FnOnce(&Self::Elem) -> bool) -> bool;
}

/// String Streams
mod string;
pub use string::*;

/// Generic Streams
mod generic;
pub use generic::*;
