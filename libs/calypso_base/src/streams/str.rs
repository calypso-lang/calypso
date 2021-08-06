use std::ops::Index;
use std::ops::Range;
use std::slice::SliceIndex;
use std::str::CharIndices;

use super::Stream;
use crate::span::Spanned;

impl From<(usize, char)> for Spanned<char> {
    fn from(loc: (usize, char)) -> Spanned<char> {
        let lo = loc.0;
        let ch = loc.1;
        let hi = lo + ch.len_utf8();
        Spanned::new((lo..hi).into(), ch)
    }
}

#[derive(Debug, Clone)]
/// A stream emitting tuples of byte locations and characters from a string slice.
pub struct StringStream<'s> {
    /// A reference to the underlying string
    string: &'s str,
    /// The cached number of characters in the string, as calculating that is O(n).
    num_chars: usize,
    /// The stream's current char and index
    indices: CharIndices<'s>,
    /// The number of characters passed so far.
    chars_passed: usize,
    /// 1ch peek
    peek: Option<Spanned<char>>,
    /// 2ch peek
    peek2: Option<Spanned<char>>,
    /// 3ch peek
    peek3: Option<Spanned<char>>,
    /// 1ch backwards peek
    prev: Option<Spanned<char>>,
}

impl<'s> StringStream<'s> {
    #[must_use]
    pub fn new(string: &'s str) -> Self {
        let mut indices = string.char_indices();

        Self {
            prev: None,
            peek: indices.next().map(|v| v.into()),
            peek2: indices.next().map(|v| v.into()),
            peek3: indices.next().map(|v| v.into()),
            num_chars: string.chars().count(),
            indices,
            string,
            chars_passed: 0,
        }
    }

    pub fn slice(&self, range: impl Into<Range<usize>>) -> &'s str {
        &self.string[range.into()]
    }
}

impl<'s> Iterator for StringStream<'s> {
    type Item = Spanned<char>;

    fn next(&mut self) -> Option<Self::Item> {
        self.prev = self.peek.take();
        self.peek = self.peek2.take();
        self.peek2 = self.peek3.take();
        self.peek3 = self.indices.next().map(|v| v.into());
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

impl<'s, I> Index<I> for StringStream<'s>
where
    I: SliceIndex<str>,
{
    type Output = <I as SliceIndex<str>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.string.index(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::span::{Span, Spanned};

    /// This tests `prev`, `next`, `peek`, `peek2`, `peek3`
    #[test]
    fn basic_streaming() {
        // I would use the trans flag emoji but it's not supported everywhere (including my IDE) as Unicode 13.0 is pretty new :(
        let string =
            "\u{1f308}\u{200d}\u{1f3f3} TRANS RIGHTS ARE HUMAN RIGHTS! THIS THE WORLD SHALL KNOW! \u{1f308}\u{200d}\u{1f3f3} \u{3b8}\u{3b9}\u{3c3} \u{1d4f2}\u{1d4fc} a test \u{1f1f8} \u{1f1f9} \u{1f1f7} \u{1f1ea} \u{1f1e6} \u{1f1f2}. Thank you!";
        let mut stream = StringStream::new(string);
        let mut curr_span = Span::default();

        // pride flag is <rainbow><ZWJ><waving white flag> = U+1f308 U+200d U+1f3f3
        // U+1f308 = rainbow
        curr_span = curr_span.add_hi('\u{1f308}'.len_utf8());
        assert!(stream.prev().is_none());
        assert_eq!(stream.next().unwrap(), Spanned::new(curr_span, '\u{1f308}'));
        assert_eq!(
            *stream.prev().unwrap(),
            Spanned::new(curr_span, '\u{1f308}')
        );
        curr_span = curr_span.shrink_to_hi().add_hi('\u{200d}'.len_utf8());
        // now the rest looks like this:
        // _: consumed,
        // _<ZWJ><waving white flag><space>
        // U+200d = ZWJ
        // let's test the peek() methood here
        assert_eq!(*stream.peek().unwrap(), Spanned::new(curr_span, '\u{200d}'));
        curr_span = curr_span.shrink_to_hi().add_hi('\u{1f3f3}'.len_utf8());
        // U+1f3f3 = waving white flag
        // let's test the peek2() method here
        assert_eq!(
            *stream.peek2().unwrap(),
            Spanned::new(curr_span, '\u{1f3f3}')
        );
        curr_span = curr_span.shrink_to_hi().add_hi(' '.len_utf8());
        // slice the whole thing and check it
        assert_eq!(&stream[0..curr_span.hi() - 1], "\u{1f308}\u{200d}\u{1f3f3}");
        // the character 3 from here is a space
        assert_eq!(*stream.peek3().unwrap(), Spanned::new(curr_span, ' '));
    }
}
