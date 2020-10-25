use std::fmt::Debug;

/// The location in a string in which some object spans.
///
/// # Example
///
/// ```rust
/// # use calypso_base::span::Span;
/// let span = Span::new(1, 5);
/// assert_eq!(span.start(), 1);
/// assert_eq!(span.length(), 5);
/// assert_eq!(span.end(), 1 + 5);
/// ```
#[derive(Debug, Default, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Span {
    start: usize,
    length: usize,
}

impl Span {
    /// Get the start of the span.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Get the length of the span.
    pub fn length(&self) -> usize {
        self.length
    }

    /// Get the index where span ends (`start + length`)
    pub fn end(&self) -> usize {
        self.start + self.length
    }

    /// Create a new span from a start and a length
    pub fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    /// Check if a span is within the range of `buffer`.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use calypso_base::span::Span;
    /// let input = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    /// // This span is valid for the input.
    /// let valid_span = Span::new(0, 3);
    /// // This span has an invalid start index, so it's not valid for the input.
    /// let invalid_span_start = Span::new(7, 3);
    /// // This span has an invalid length, so it's not valid for the input.
    /// let invalid_span_length = Span::new(0, 10);
    /// assert!(valid_span.is_valid_for(&input));
    /// assert!(!invalid_span_start.is_valid_for(&input));
    /// assert!(!invalid_span_length.is_valid_for(&input));
    /// ```
    pub fn is_valid_for<T>(&self, buffer: &[T]) -> bool {
        self.start <= buffer.len() && self.end() <= buffer.len()
    }
}

/// An object that's associated with a [`Span`](Span)
///
/// # Example
///
/// ```rust
/// # use calypso_base::span::{Span, Spanned};
/// let spanned = Spanned::new(Span::new(0, 1), 42);
/// // Since we're asserting more than one thing, we can't do `spanned.value_owned()`, as that would anger the borrow checker.
/// // Instead, since integers are `Copy`, we can just dereference the `&{integer}` we get back from `spanned.value()`.
/// assert_eq!(*spanned.value(), 42);
/// assert_eq!(spanned.span(), Span::new(0, 1));
/// ```
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
    /// Associate a span with a value.
    pub fn new(span: Span, value: T) -> Self {
        Self { span, value }
    }

    /// Get a reference to the value.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Get the owned value.
    pub fn value_owned(self) -> T {
        self.value
    }

    /// Get the value's [`Span`](Span)
    pub fn span(&self) -> Span {
        self.span
    }
}
