use std::fmt::Debug;

#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Span {
    start: usize,
    length: usize,
}

impl Span {
    pub fn start(&self) -> usize {
        self.start
    }

    pub fn length(&self) -> usize {
        self.length
    }

    pub fn new(start: usize, length: usize) -> Self {
        let end = start + length;
        if start > end {
            panic!("A span's start can not be after its end!");
        }

        Self { start, length }
    }

    pub fn is_valid_for<T>(&self, buffer: &[T]) -> bool {
        self.start <= buffer.len() && self.start + self.length <= buffer.len()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Spanned<T>
where
    T: Debug,
{
    span: Span,
    value: T,
}

impl<T> Spanned<T>
where
    T: Debug,
{
    pub fn new(span: Span, value: T) -> Self {
        Self { span, value }
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}
