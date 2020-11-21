/// A trait for streamed data. For a stream, `EOF` means either
/// the end of file / input if this stream is backed by a file or string,
/// or the end of the stream.
pub trait Stream: Iterator {
    /// This stream can no longer produce elements if this returns true
    fn is_at_end(&self) -> bool;

    /// Peek at the value one element ahead, without consuming it.
    fn peek(&self) -> Option<&<Self as Iterator>::Item>;
    /// Peek at the value two element ahead, without consuming anything.
    fn peek2(&self) -> Option<&<Self as Iterator>::Item>;
    /// Peek at the value three element ahead, without consuming anything.
    fn peek3(&self) -> Option<&<Self as Iterator>::Item>;
    /// Peek at the value one element behind, without moving backwards.
    fn prev(&self) -> Option<&<Self as Iterator>::Item>;

    /// Consume the next element if the condition is true.
    ///
    /// If `func` returns true for the element, consume and return it.
    /// Otherwise, return `None`.    
    fn next_if(
        &mut self,
        func: impl FnOnce(&<Self as Iterator>::Item) -> bool,
    ) -> Option<<Self as Iterator>::Item> {
        if func(self.peek()?) {
            self.next()
        } else {
            None
        }
    }

    /// Consume the next element if it is equal to `expected`.
    fn next_if_eq<R>(&mut self, expected: &R) -> Option<<Self as Iterator>::Item>
    where
        R: ?Sized,
        <Self as Iterator>::Item: PartialEq<R>,
    {
        if self.peek()? == expected {
            self.next()
        } else {
            None
        }
    }

    /// Keep consuming elements while the condition is true
    /// and there are remaining elements.
    ///
    /// If `func` returns true for the element, consume it.
    /// Repeat until `func` returns false for an element.
    ///
    /// `func` takes in the element first, then the number of
    /// elements consumed so far.
    fn gorge_while(&mut self, mut func: impl FnMut(&<Self as Iterator>::Item, usize) -> bool) {
        let mut count = 0;
        while self.peek().is_some() && func(self.peek().unwrap(), count) {
            self.next();
            count += 1;
        }
    }

    /// Keep consuming elements while the element is equal to `expected`
    /// and there are remaining elements.
    fn gorge_while_eq<R>(&mut self, expected: &R)
    where
        R: ?Sized,
        <Self as Iterator>::Item: PartialEq<R>,
    {
        while self.next_if_eq(expected).is_some() {}
    }

    /// Returns `Some(true)` if the value one element ahead is equal to `expected`.
    /// Otherwise, return `Some(false)`. In neither case should any values be consumed.
    /// On EOF, return `None`.
    fn peek_eq<R>(&self, expected: &R) -> Option<bool>
    where
        R: ?Sized,
        <Self as Iterator>::Item: PartialEq<R>,
    {
        self.peek().map(|v| v == expected)
    }

    /// Returns `Some(true)` if the value two elements ahead is equal to `expected`.
    /// Otherwise, return `Some(false)`. In neither case should any values be consumed.
    /// On EOF, return `None`.
    fn peek2_eq<R>(&self, expected: &R) -> Option<bool>
    where
        R: ?Sized,
        <Self as Iterator>::Item: PartialEq<R>,
    {
        self.peek2().map(|v| v == expected)
    }

    /// Returns `Some(true)` if the value three elements ahead is equal to `expected`.
    /// Otherwise, return `Some(false)`. In neither case should any values be consumed.
    /// On EOF, return `None`.
    fn peek3_eq<R>(&self, expected: &R) -> Option<bool>
    where
        R: ?Sized,
        <Self as Iterator>::Item: PartialEq<R>,
    {
        self.peek3().map(|v| v == expected)
    }

    /// Returns `Some(true)` if the value one element behind is equal to `expected`.
    /// Otherwise, return `Some(false)`. In neither case should any values be consumed.
    /// On EOF, return `None`.
    fn prev_eq<R>(&self, expected: &R) -> Option<bool>
    where
        R: ?Sized,
        <Self as Iterator>::Item: PartialEq<R>,
    {
        self.prev().map(|v| v == expected)
    }

    /// Returns the value of the condition, wrapped in an `Option`.
    /// In neither case should any values be consumed. On EOF, return `None`.
    ///
    /// `func` takes in the value one element ahead.
    fn peek_cond(&self, func: impl FnOnce(&<Self as Iterator>::Item) -> bool) -> Option<bool> {
        self.peek().map(func)
    }

    /// Returns the value of the condition, wrapped in an `Option`.
    /// In neither case should any values be consumed. On EOF, return `None`.
    ///
    /// `func` takes in the value two elements ahead.
    fn peek2_cond(&self, func: impl FnOnce(&<Self as Iterator>::Item) -> bool) -> Option<bool> {
        self.peek2().map(func)
    }

    /// Returns the value of the condition, wrapped in an `Option`.
    /// In neither case should any values be consumed. On EOF, return `None`.
    ///
    /// `func` takes in the value three elements ahead.
    fn peek3_cond(&self, func: impl FnOnce(&<Self as Iterator>::Item) -> bool) -> Option<bool> {
        self.peek3().map(func)
    }

    /// Returns the value of the condition, wrapped in an `Option`.
    /// In neither case should any values be consumed. On EOF, return `None`.
    ///
    /// `func` takes in the value one element behind.
    fn prev_cond(&self, func: impl FnOnce(&<Self as Iterator>::Item) -> bool) -> Option<bool> {
        self.prev().map(func)
    }
}

/// String Streams
mod string;
pub use string::*;

/// Generic Streams
mod generic;
pub use generic::*;
