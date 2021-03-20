use std::fmt::Debug;
use std::ops::Range;

/// The location in a slice in which some object spans.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Span {
    lo: usize,
    hi: usize,
}

impl Span {
    #[must_use]
    pub fn new(lo: usize, hi: usize) -> Self {
        Span { lo, hi }
    }

    #[must_use]
    pub fn new_shrunk(amount: usize) -> Self {
        Span {
            lo: amount,
            hi: amount,
        }
    }

    /// Create a dummy span (that has a span equivalent to the range `0..0`).
    #[must_use]
    pub fn new_dummy() -> Self {
        Self { lo: 0, hi: 0 }
    }

    #[must_use]
    pub fn lo(&self) -> usize {
        self.lo
    }

    #[must_use]
    pub fn with_lo(self, lo: usize) -> Self {
        Self { lo, ..self }
    }

    #[must_use]
    pub fn hi(&self) -> usize {
        self.hi
    }

    #[must_use]
    pub fn with_hi(self, hi: usize) -> Self {
        Self { hi, ..self }
    }

    /// Returns `true`  if this is a dummy span
    #[must_use]
    pub fn is_dummy(&self) -> bool {
        self.lo == 0 && self.hi == 0
    }

    /// Returns a new span representing an empty span at the beginning of this span
    #[must_use]
    pub fn shrink_to_lo(&self) -> Span {
        self.with_hi(self.lo)
    }

    /// Returns a new span representing an empty span at the end of this span.
    #[must_use]
    pub fn shrink_to_hi(self) -> Span {
        self.with_lo(self.hi)
    }

    /// Returns true if if `hi == lo`
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.hi == self.lo
    }

    /// Returns `self` if `self` is not a dummy span, and `other` otherwise.
    #[must_use]
    pub fn substitute_dummy(self, other: Span) -> Span {
        if self.is_dummy() {
            other
        } else {
            self
        }
    }

    /// Returns `true` if `self` fully encloses `other`.
    #[must_use]
    pub fn contains(self, other: Span) -> bool {
        self.lo <= other.lo && other.hi <= self.hi
    }

    /// Returns `true` if `self` touches `other`.
    #[must_use]
    pub fn overlaps(self, other: Span) -> bool {
        self.lo < other.hi && other.lo < other.hi
    }

    /// Returns a `Span` that would enclose both `self` and `end`.
    ///
    /// ```text
    ///     ____             ___
    ///     self lorem ipsum end
    ///     ^^^^^^^^^^^^^^^^^^^^
    /// ```
    #[must_use]
    pub fn to(self, end: Span) -> Span {
        Span::new(
            std::cmp::min(self.lo, end.lo),
            std::cmp::max(self.hi, end.hi),
        )
    }

    /// Returns a `Span` between the end of `self` to the beginning of `end`.
    ///
    /// ```text
    ///     ____             ___
    ///     self lorem ipsum end
    ///         ^^^^^^^^^^^^^
    /// ```
    #[must_use]
    pub fn between(self, end: Span) -> Span {
        Span::new(self.hi, end.lo)
    }

    /// Returns a `Span` from the beginning of `self` until the beginning of `end`.
    ///
    /// ```text
    ///     ____             ___
    ///     self lorem ipsum end
    ///     ^^^^^^^^^^^^^^^^^
    /// ```
    #[must_use]
    pub fn until(self, end: Span) -> Span {
        Span::new(self.lo, end.lo)
    }

    #[must_use]
    pub fn add_hi(self, amount: usize) -> Span {
        self.with_hi(self.hi() + amount)
    }

    #[must_use]
    pub fn sub_hi(self, amount: usize) -> Span {
        self.with_hi(self.hi() - amount)
    }

    #[must_use]
    pub fn add_lo(self, amount: usize) -> Span {
        self.with_lo(self.lo() + amount)
    }

    #[must_use]
    pub fn sub_lo(self, amount: usize) -> Span {
        self.with_lo(self.lo() - amount)
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::new_dummy()
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.lo..span.hi
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Span::new(range.start, range.end)
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
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
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
impl<T: PartialEq + Debug> PartialEq<T> for Spanned<T> {
    fn eq(&self, other: &T) -> bool {
        self.value.eq(other)
    }
}
