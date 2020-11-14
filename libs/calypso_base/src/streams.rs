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

    /// Keep consuming elements while the condition is true.
    ///
    /// If `func` returns true for the element, consume it.
    /// Repeat until `func` returns false for an element.
    ///
    /// Func takes in the element first, then the number of
    /// elements consumed so far.
    fn gorge_while(&mut self, func: impl FnMut(&Self::Elem, usize) -> bool);

    /// Keep consuming elements while the element is equal to `expected`.
    fn gorge_while_eq<R>(&mut self, expected: &R)
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>;
}

// === String Streams === //
use std::ops::Index;
use std::slice::SliceIndex;
use std::str::CharIndices;

pub type CharLoc = (usize, char);

#[derive(Debug, Clone)]
/// A stream emitting tuples of byte locations and characters from a string slice.
pub struct StringStream<'s> {
    /// A reference to the underlying string
    string: &'s str,
    /// The cached number of characters in the string
    num_chars: usize,
    /// The stream's current char and index
    indices: CharIndices<'s>,
    /// The number of characters passed so far.
    chars_passed: usize,
    /// 1ch peek
    peek: Option<CharLoc>,
    /// 2ch peek
    peek2: Option<CharLoc>,
    /// 3ch peek
    peek3: Option<CharLoc>,
    /// 1ch backwards peek
    prev: Option<CharLoc>,
}

impl<'s> StringStream<'s> {
    pub fn new(string: &'s str) -> Self {
        let mut indices = string.char_indices();

        Self {
            prev: None,
            peek: indices.next(),
            peek2: indices.next(),
            peek3: indices.next(),
            num_chars: string.chars().count(),
            indices,
            string,
            chars_passed: 0,
        }
    }
}

impl<'s> Iterator for StringStream<'s> {
    type Item = CharLoc;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev = self.peek.take();
        self.peek = self.peek2.take();
        self.peek2 = self.peek3.take();
        self.peek3 = self.indices.next();
        self.prev
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.num_chars - self.chars_passed,
            Some(self.num_chars - self.chars_passed),
        )
    }
}

impl<'s> Stream for StringStream<'s> {
    type Elem = CharLoc;

    fn is_at_end(&self) -> bool {
        self.chars_passed >= self.num_chars
    }

    fn peek(&self) -> Option<&Self::Elem> {
        self.peek.as_ref()
    }

    fn peek2(&self) -> Option<&Self::Elem> {
        self.peek2.as_ref()
    }

    fn peek3(&self) -> Option<&Self::Elem> {
        self.peek3.as_ref()
    }

    fn prev(&self) -> Option<&Self::Elem> {
        self.prev.as_ref()
    }

    fn next_if(&mut self, func: impl FnOnce(&Self::Elem) -> bool) -> Option<Self::Elem> {
        if func(self.peek()?) {
            self.next()
        } else {
            None
        }
    }

    fn next_if_eq<R>(&mut self, expected: &R) -> Option<Self::Elem>
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>,
    {
        if self.peek()? == expected {
            self.next()
        } else {
            None
        }
    }

    fn gorge_while(&mut self, mut func: impl FnMut(&Self::Elem, usize) -> bool) {
        let mut count = 0;
        while self.peek().is_some() && func(self.peek().unwrap(), count) {
            self.next();
            count += 1;
        }
    }

    fn gorge_while_eq<R>(&mut self, expected: &R)
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>,
    {
        while self.next_if_eq(expected).is_some() {}
    }
}

impl<'s, I> Index<I> for StringStream<'s>
where
    I: SliceIndex<str>,
{
    type Output = <I as SliceIndex<str>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.string.index(index)
    }
}

// == General Streams == //
use std::vec::IntoIter;

pub struct Streamed<'s, T: Clone> {
    /// A reference to the underlying slice
    elements: &'s [T],
    /// The stream's current char and index
    iter: IntoIter<T>,
    /// The number of elements passed so far.
    num_passed: usize,
    /// 1elem peek
    peek: Option<T>,
    /// 2elem peek
    peek2: Option<T>,
    /// 3elem peek
    peek3: Option<T>,
    /// 1elem backwards peek
    prev: Option<T>,
}

impl<'s, T: Clone> Streamed<'s, T> {
    pub fn new(elements: &'s [T], mut iter: IntoIter<T>) -> Self {
        Self {
            prev: None,
            peek: iter.next(),
            peek2: iter.next(),
            peek3: iter.next(),
            iter,
            elements,
            num_passed: 0,
        }
    }
}

impl<'s, T: Clone> Iterator for Streamed<'s, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev = self.peek.take();
        self.peek = self.peek2.take();
        self.peek2 = self.peek3.take();
        self.peek3 = self.iter.next();
        self.prev.clone()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.elements.len() - self.num_passed,
            Some(self.elements.len() - self.num_passed),
        )
    }
}

impl<'s, T: Clone> Stream for Streamed<'s, T> {
    type Elem = T;

    fn is_at_end(&self) -> bool {
        self.num_passed >= self.elements.len()
    }

    fn peek(&self) -> Option<&Self::Elem> {
        self.peek.as_ref()
    }

    fn peek2(&self) -> Option<&Self::Elem> {
        self.peek2.as_ref()
    }

    fn peek3(&self) -> Option<&Self::Elem> {
        self.peek3.as_ref()
    }

    fn prev(&self) -> Option<&Self::Elem> {
        self.prev.as_ref()
    }

    fn next_if(&mut self, func: impl FnOnce(&Self::Elem) -> bool) -> Option<Self::Elem> {
        if func(self.peek()?) {
            self.next()
        } else {
            None
        }
    }

    fn next_if_eq<R>(&mut self, expected: &R) -> Option<Self::Elem>
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>,
    {
        if self.peek()? == expected {
            self.next()
        } else {
            None
        }
    }

    fn gorge_while(&mut self, mut func: impl FnMut(&Self::Elem, usize) -> bool) {
        let mut count = 0;
        while self.peek().is_some() && func(self.peek().unwrap(), count) {
            self.next();
            count += 1;
        }
    }

    fn gorge_while_eq<R>(&mut self, expected: &R)
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>,
    {
        while self.next_if_eq(expected).is_some() {}
    }
}

impl<'s, I, T: Clone> Index<I> for Streamed<'s, T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.elements.index(index)
    }
}
