use calypso_base::span::Span;
use calypso_diagnostic::{diagnostic, error};
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Buffer<'buf> {
    buffer: &'buf [char],
    start: usize,
    current: usize,
}

impl<'buf> Buffer<'buf> {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.buffer.len()
    }

    pub fn peek(&self) -> Option<char> {
        self.buffer.get(self.current).copied()
    }

    pub fn peek_next(&self) -> Option<char> {
        self.buffer.get(self.current + 1).copied()
    }

    pub fn last(&self) -> Option<char> {
        if self.current == 0 {
            None
        } else {
            self.buffer.get(self.current - 1).copied()
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.buffer.get(self.current - 1).copied()
    }

    pub fn backtrack(&mut self) -> Option<char> {
        if self.current == 0 {
            None
        } else {
            self.current -= 1;
            self.buffer.get(self.current).copied()
        }
    }

    pub fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.buffer[self.current] != expected {
            false
        } else {
            self.current += 1;
            true
        }
    }

    pub fn set_start(&mut self, new_start: usize) {
        self.start = new_start;
    }

    pub fn consume(&mut self, expected: char, message: String, eid: u16) -> error::Result<()> {
        if self.match_next(expected) {
            Ok(())
        } else {
            // TODO: source name
            let diagnostic = diagnostic::Diagnostic::new(
                Span::new(self.start, self.current - self.start),
                self.buffer,
                "<anon>".to_string(),
                message,
                eid,
            );
            Err(error::ErrorKind::Diagnostic(diagnostic).into())
        }
    }

    pub fn slice(&self, lower: usize, upper: usize) -> &'buf [char] {
        &self.buffer[lower..upper]
    }
}
