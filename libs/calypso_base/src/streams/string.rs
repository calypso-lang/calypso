use super::Stream;
use std::ops::Index;
use std::slice::SliceIndex;
use std::str::CharIndices;

/// A location in text at which a character is.
/// The first element is the byte index of the character
/// and the second is the character itself.
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
        self.chars_passed += 1;
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

impl<'s, I> Index<I> for StringStream<'s>
where
    I: SliceIndex<str>,
{
    type Output = <I as SliceIndex<str>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.string.index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use proptest::prelude::*;

    /// This tests `prev`, `next`, `peek`, `peek2`, `peek3`
    #[test]
    fn basic_streaming() {
        // I would use the trans flag emoji but it's not supported everywhere (including my IDE) as Unicode 13.0 is pretty new :(
        let string =
            "🌈‍🏳 TRANS RIGHTS ARE HUMAN RIGHTS! THIS THE WORLD SHALL KNOW! 🌈‍🏳 θισ 𝓲𝓼 a test 🇸 🇹 🇷 🇪 🇦 🇲. Thank you!";
        let mut stream = StringStream::new(string);
        let mut curr_idx = 0;

        // pride flag is <rainbow><ZWJ><waving white flag> = U+1f308 U+200d U+1f3f3
        // U+1f308 = rainbow
        assert!(stream.prev().is_none());
        assert_eq!(stream.next().unwrap(), (curr_idx, '\u{1f308}'));
        assert_eq!(*stream.prev().unwrap(), (curr_idx, '\u{1f308}'));
        // now the rest looks like this:
        // _: consumed,
        // _<ZWJ><waving white flag><space>
        curr_idx += '\u{1f308}'.len_utf8();
        // U+200d = ZWJ
        // let's test the peek() methood here
        assert_eq!(*stream.peek().unwrap(), (curr_idx, '\u{200d}'));
        curr_idx += '\u{200d}'.len_utf8();
        // U+1f3f3 = waving white flag
        // let's test the peek2() method here
        assert_eq!(
            *stream.peek2().unwrap(),
            ("\u{1f308}\u{200d}".len(), '\u{1f3f3}')
        );
        curr_idx += '\u{1f3f3}'.len_utf8();
        // slice the whole thing and check it
        assert_eq!(&stream[0..curr_idx], "🌈‍🏳");
        // the character 3 from here is a space
        assert_eq!(*stream.peek3().unwrap(), (curr_idx, ' '));
    }
}
