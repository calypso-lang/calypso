use super::Stream;

/// A generic stream that wraps an existing iterator.
pub struct StreamedIter<T: Clone, I: Iterator<Item = T>> {
    /// The wrapped iterator
    iter: I,
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

impl<T: Clone, I: Iterator<Item = T>> StreamedIter<T, I> {
    pub fn new(mut iter: I) -> Self {
        Self {
            prev: None,
            peek: iter.next(),
            peek2: iter.next(),
            peek3: iter.next(),
            iter,
            num_passed: 0,
        }
    }
}

impl<T: Clone, I: Iterator<Item = T>> Iterator for StreamedIter<T, I> {
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
        self.iter.size_hint()
    }
}

impl<T: Clone, I: Iterator<Item = T>> Stream for StreamedIter<T, I> {
    fn is_at_end(&self) -> bool {
        self.peek.is_none()
    }

    fn peek(&self) -> Option<&<Self as Iterator>::Item> {
        self.peek.as_ref()
    }

    fn peek2(&self) -> Option<&<Self as Iterator>::Item> {
        self.peek2.as_ref()
    }

    fn peek3(&self) -> Option<&<Self as Iterator>::Item> {
        self.peek3.as_ref()
    }

    fn prev(&self) -> Option<&<Self as Iterator>::Item> {
        self.prev.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// This tests `prev`, `next`, `peek`, `peek2`, `peek3`
    #[test]
    fn basic_streaming() {
        let slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut stream = StreamedIter::new(slice.into_iter());

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
        // the element 3 from here is `4`
        assert_eq!(*stream.peek3().unwrap(), 4);
    }
}
