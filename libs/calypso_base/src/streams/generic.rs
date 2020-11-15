use super::Stream;
use std::ops::Index;
use std::slice::SliceIndex;
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
        self.num_passed += 1;
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
        self.peek.is_none()
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

    fn peek_eq<R>(&self, expected: &R) -> bool
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>,
    {
        if let Some(elem) = self.peek() {
            elem == expected
        } else {
            false
        }
    }

    fn peek_cond(&self, func: impl FnOnce(&Self::Elem) -> bool) -> bool {
        self.peek().is_some() && func(self.peek().unwrap())
    }

    fn peek2_eq<R>(&self, expected: &R) -> bool
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>,
    {
        if let Some(elem) = self.peek2() {
            elem == expected
        } else {
            false
        }
    }

    fn peek2_cond(&self, func: impl FnOnce(&Self::Elem) -> bool) -> bool {
        self.peek2().is_some() && func(self.peek2().unwrap())
    }

    fn peek3_eq<R>(&self, expected: &R) -> bool
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>,
    {
        if let Some(elem) = self.peek3() {
            elem == expected
        } else {
            false
        }
    }

    fn peek3_cond(&self, func: impl FnOnce(&Self::Elem) -> bool) -> bool {
        self.peek3().is_some() && func(self.peek3().unwrap())
    }

    fn prev_eq<R>(&self, expected: &R) -> bool
    where
        R: ?Sized,
        Self::Elem: PartialEq<R>,
    {
        if let Some(elem) = self.prev() {
            elem == expected
        } else {
            false
        }
    }

    fn prev_cond(&self, func: impl FnOnce(&Self::Elem) -> bool) -> bool {
        self.prev().is_some() && func(self.prev().unwrap())
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
