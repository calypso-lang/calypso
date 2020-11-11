use calypso_base::span::Span;
use calypso_diagnostic::{diagnostic::Diagnostic, error};

/// A wrapper around a `char` slice, providing helper functions for common activites.
#[derive(Debug, Clone)]
pub struct Buffer<'buf> {
    buffer: &'buf [char],
    start: usize,
    current: usize,
}

impl<'buf> Buffer<'buf> {
    /// Create a new `Buffer` from a `&[char]`.
    pub fn new(buffer: &'buf [char]) -> Self {
        Self {
            buffer,
            start: 0,
            current: 0,
        }
    }

    /// The start of the current span.
    pub fn start(&self) -> usize {
        self.start
    }

    /// The current location of the cursor.
    pub fn current(&self) -> usize {
        self.current
    }

    /// Get a reference to the internal buffer.
    pub fn buffer(&self) -> &'buf [char] {
        self.buffer
    }

    /// Check if the cursor is at or after the end of the input.
    pub fn is_at_end(&self) -> bool {
        self.current >= self.buffer.len()
    }

    /// Peek at the value under the cursor, without moving it.
    pub fn peek(&self) -> Option<char> {
        self.buffer.get(self.current).copied()
    }

    /// Peek at the value after the cursor, without moving it.
    pub fn peek_next(&self) -> Option<char> {
        self.buffer.get(self.current + 1).copied()
    }

    /// Peek at the value 2 characters after the cursor, without moving it.
    pub fn peek_2(&self) -> Option<char> {
        self.buffer.get(self.current + 2).copied()
    }

    /// Peek at the value just before the cursor, without moving it.
    pub fn last(&self) -> Option<char> {
        if self.current == 0 {
            None
        } else {
            self.buffer.get(self.current - 1).copied()
        }
    }

    /// Look at the value under the cursor, then move it forward.
    pub fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.buffer.get(self.current - 1).copied()
    }

    /// Move the cursor backward, then look at the character under the cursor.
    pub fn backtrack(&mut self) -> Option<char> {
        if self.current == 0 {
            None
        } else {
            self.current -= 1;
            self.buffer.get(self.current).copied()
        }
    }

    /// Returns `true` and move the cursor forward if the character under the
    /// cursor matches `expected`, otherwise return `false` without moving the
    /// cursor. If the cursor is at or after the end of the input, `false` is
    /// returned and the cursor is not moved.
    pub fn match_next(&mut self, expected: char) -> bool {
        let ch = self.peek();
        if ch.is_none() {
            return false;
        }
        if self.is_at_end() || ch.unwrap() != expected {
            false
        } else {
            self.advance();
            true
        }
    }

    /// Returns `true` and move the cursor forward if `predicate` returns true when
    /// given the character under the cursor, otherwise return `false` without moving
    /// the cursor. If the cursor is at or after the end of the input, `false` is
    /// returned and the cursor is not moved.
    pub fn match_next_if(&mut self, mut predicate: impl FnMut(char) -> bool) -> bool {
        let ch = self.peek();
        if ch.is_none() {
            return false;
        }
        if self.is_at_end() || !predicate(ch.unwrap()) {
            false
        } else {
            self.advance();
            true
        }
    }

    /// Set the start of the current span to `new_start`
    pub fn set_start(&mut self, new_start: usize) {
        self.start = new_start;
    }

    /// Set the start of the current span to the current location of the cursor.
    pub fn current_to_start(&mut self) {
        self.start = self.current;
    }

    /// Check if the next character is `expected`. If not, `diagnostic_gen` is run, taking
    /// in the current [`Span`](calypso_base::span::Span) of the buffer, returning a
    /// [`Diagnostic`](calypso_diagnostic::diagnostic::Diagnostic) that is returned as a
    /// [`Result`](calypso_diagnostic::error::Result).
    pub fn consume(
        &mut self,
        expected: char,
        mut diagnostic_gen: impl FnMut(Span) -> Diagnostic,
    ) -> error::Result<()> {
        if self.match_next(expected) {
            self.advance();
            Ok(())
        } else {
            Err(diagnostic_gen(Span::new(self.start, self.current - self.start)).into())
        }
    }

    /// Check if `predicate` returns a true value when given the character under the cursor.
    /// If not, `diagnostic_gen` is run, taking in the current [`Span`](calypso_base::span::Span)
    /// of the buffer, returning a [`Diagnostic`](calypso_diagnostic::diagnostic::Diagnostic) that
    /// is returned as a [`Result`](calypso_diagnostic::error::Result).
    pub fn consume_if(
        &mut self,
        predicate: impl FnMut(char) -> bool,
        mut diagnostic_gen: impl FnMut(Span) -> Diagnostic,
    ) -> error::Result<()> {
        if self.match_next_if(predicate) {
            self.advance();
            Ok(())
        } else {
            Err(error::ErrorKind::Diagnostic(diagnostic_gen(Span::new(
                self.start,
                self.current - self.start,
            )))
            .into())
        }
    }

    /// Keep consuming characters until a character that is not
    /// `expected` is found.
    pub fn gorge(&mut self, expected: char) {
        loop {
            if !self.match_next(expected) {
                break;
            }
        }
    }

    /// Keep consuming characters until `predicate` does not return `true`.
    ///
    /// `predicate` takes the character and the number of characters found so far.
    pub fn gorge_while(&mut self, mut predicate: impl FnMut(char, usize) -> bool) {
        let mut count = 0;
        loop {
            let ch = self.peek();
            if ch.is_none() {
                break;
            }
            if self.is_at_end() || !predicate(ch.unwrap(), count) {
                break;
            } else {
                self.advance();
                count += 1;
            }
        }
    }

    /// Slice the buffer at a certain range.
    pub fn slice(&self, lower: usize, upper: usize) -> &'buf [char] {
        &self.buffer[lower..upper]
    }
}
