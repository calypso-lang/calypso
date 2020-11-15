use super::Stream;
use std::ops::Index;
use std::slice::Iter;
use std::slice::SliceIndex;

/// A generic stream.
///
/// `T` is `Copy` because using a slice iterator produces references.
/// To avoid possibly expensive cloning every time you wish to iterate,
/// it's required that `T` is `Copy`. If you need to, you can always
/// use a reference type for `T`.
pub struct Streamed<'s, T: Copy> {
    /// A reference to the underlying slice
    elements: &'s [T],
    /// The stream's current char and index
    iter: Iter<'s, T>,
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

impl<'s, T: Copy> Streamed<'s, T> {
    pub fn new(elements: &'s [T]) -> Self {
        let mut iter = elements.iter();
        Self {
            prev: None,
            peek: iter.next().copied(),
            peek2: iter.next().copied(),
            peek3: iter.next().copied(),
            iter,
            elements,
            num_passed: 0,
        }
    }
}

impl<'s, T: Copy> Iterator for Streamed<'s, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev = self.peek.take();
        self.peek = self.peek2.take();
        self.peek2 = self.peek3.take();
        self.peek3 = self.iter.next().copied();
        self.num_passed += 1;
        self.prev
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            self.elements.len() - self.num_passed,
            Some(self.elements.len() - self.num_passed),
        )
    }
}

impl<'s, T: Copy> Stream for Streamed<'s, T> {
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

impl<'s, I, T: Copy> Index<I> for Streamed<'s, T>
where
    I: SliceIndex<[T]>,
{
    type Output = <I as SliceIndex<[T]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.elements.index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This tests `prev`, `next`, `peek`, `peek2`, `peek3`
    #[test]
    fn basic_streaming() {
        let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut stream = Streamed::new(&slice);

        assert!(stream.prev().is_none());
        assert_eq!(stream.next().unwrap(), 1);
        assert_eq!(*stream.prev().unwrap(), 1);
        // now the rest looks like this:
        // _: consumed,
        // _23456789
        // let's test the peek() methood here
        assert_eq!(*stream.peek().unwrap(), 2);
        // let's test the peek2() method here
        assert_eq!(*stream.peek2().unwrap(), 3);
        // slice a bit of the slice and check it
        assert_eq!(&stream[0..3], [1, 2, 3]);
        // the element 3 from here is `4`
        assert_eq!(*stream.peek3().unwrap(), 4);
    }
}
