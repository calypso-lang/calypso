use std::fmt::Debug;
use std::ops::Range;

use crate::symbol::Symbol;

/// The location in a slice in which some object spans.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Span {
    lo: u32,
    hi: u32,
}

impl Span {
    /// Create a new span given the low and high indices.
    #[must_use]
    #[inline]
    pub fn new(lo: u32, hi: u32) -> Self {
        Span { lo, hi }
    }

    /// Create a new empty span around an index.
    #[must_use]
    #[inline]
    pub fn new_shrunk(amount: u32) -> Self {
        Span {
            lo: amount,
            hi: amount,
        }
    }

    /// Create a dummy span (that has a span equivalent to the range `0..0`).
    #[must_use]
    #[inline]
    pub fn new_dummy() -> Self {
        Self { lo: 0, hi: 0 }
    }

    /// Get the low index of a span.
    #[must_use]
    #[inline]
    pub fn lo(self) -> u32 {
        self.lo
    }

    /// Set the low index of a span.
    #[must_use]
    #[inline]
    pub fn with_lo(self, lo: u32) -> Self {
        Self { lo, ..self }
    }

    /// Get the high index of a span.
    #[must_use]
    #[inline]
    pub fn hi(self) -> u32 {
        self.hi
    }

    /// Set the high index of a span.
    #[must_use]
    #[inline]
    pub fn with_hi(self, hi: u32) -> Self {
        Self { hi, ..self }
    }

    /// Returns `true` if this is a dummy span
    #[must_use]
    #[inline]
    pub fn is_dummy(self) -> bool {
        self.lo == 0 && self.hi == 0
    }

    /// Returns a new span representing an empty span at the beginning of this span
    #[must_use]
    #[inline]
    pub fn shrink_to_lo(self) -> Span {
        self.with_hi(self.lo)
    }

    /// Returns a new span representing an empty span at the end of this span.
    #[must_use]
    #[inline]
    pub fn shrink_to_hi(self) -> Span {
        self.with_lo(self.hi)
    }

    /// Returns true if if `hi == lo`
    #[must_use]
    #[inline]
    pub fn is_empty(self) -> bool {
        self.hi == self.lo
    }

    /// Returns `self` if `self` is not a dummy span, and `other` otherwise.
    #[must_use]
    #[inline]
    pub fn substitute_dummy(self, other: Span) -> Span {
        if self.is_dummy() {
            other
        } else {
            self
        }
    }

    /// Returns `true` if `self` fully encloses `other`.
    #[must_use]
    #[inline]
    pub fn contains(self, other: Span) -> bool {
        self.lo <= other.lo && other.hi <= self.hi
    }

    /// Returns `true` if `self` touches `other`.
    #[must_use]
    #[inline]
    pub fn overlaps(self, other: Span) -> bool {
        self.lo < other.hi && other.lo < self.hi
    }

    /// Returns a `Span` that would enclose both `self` and `end`.
    ///
    /// ```text
    ///     ____             ___
    ///     self lorem ipsum end
    ///     ^^^^^^^^^^^^^^^^^^^^
    /// ```
    #[must_use]
    #[inline]
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
    #[inline]
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
    #[inline]
    pub fn until(self, end: Span) -> Span {
        Span::new(self.lo, end.lo)
    }

    /// Add the given amount to the high index of a span.
    #[must_use]
    #[inline]
    pub fn add_hi(self, amount: u32) -> Span {
        self.with_hi(self.hi + amount)
    }

    /// Subtract the given amount from the high index of a span.
    #[must_use]
    #[inline]
    pub fn sub_hi(self, amount: u32) -> Span {
        self.with_hi(self.hi - amount)
    }

    /// Add the given amount to the low index of a span.
    #[must_use]
    #[inline]
    pub fn add_lo(self, amount: u32) -> Span {
        self.with_lo(self.lo + amount)
    }

    /// Subtract the given amount from the low index of a span.
    #[must_use]
    #[inline]
    pub fn sub_lo(self, amount: u32) -> Span {
        self.with_lo(self.lo - amount)
    }

    /// Get the length (`hi - lo`) of a span.
    #[must_use]
    #[inline]
    pub fn len(self) -> u32 {
        self.hi - self.lo
    }

    /// Convert to a [`Range`]. (This function present to prevent generics
    /// hell.)
    #[must_use]
    #[inline]
    pub fn into_range(self) -> Range<usize> {
        self.into()
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::new_dummy()
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.lo as usize..span.hi as usize
    }
}

impl From<Span> for Range<u32> {
    fn from(span: Span) -> Self {
        span.lo..span.hi
    }
}

impl From<Range<u32>> for Span {
    fn from(range: Range<u32>) -> Self {
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

    /// Convert a `&'a Spanned<T>` into a `Spanned<&'a T>`.
    #[must_use]
    pub fn as_ref(&'_ self) -> Spanned<&'_ T> {
        Spanned {
            span: self.span,
            value: &self.value,
        }
    }

    /// Map the value of a `Spanned<T>`.
    #[must_use]
    pub fn map<U: Debug>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            span: self.span,
            value: f(self.value),
        }
    }
}

impl<T: PartialEq + Debug> PartialEq<T> for Spanned<T> {
    fn eq(&self, other: &T) -> bool {
        self.value.eq(other)
    }
}

impl<T: Debug> From<(u32, T, u32)> for Spanned<T> {
    fn from((lo, val, hi): (u32, T, u32)) -> Self {
        Spanned::new(Span::new(lo, hi), val)
    }
}

impl<T: Debug> From<(T, Range<u32>)> for Spanned<T> {
    fn from((val, range): (T, Range<u32>)) -> Self {
        Spanned::new(Span::from(range), val)
    }
}

impl ariadne::Span for Span {
    type SourceId = ();

    fn source(&self) -> &Self::SourceId {
        &()
    }

    fn start(&self) -> usize {
        self.lo as usize
    }

    fn end(&self) -> usize {
        self.hi as usize
    }
}

impl chumsky::span::Span for Span {
    type Context = ();

    type Offset = u32;

    fn new(_context: Self::Context, range: Range<Self::Offset>) -> Self {
        Self {
            lo: range.start,
            hi: range.end,
        }
    }

    fn context(&self) -> Self::Context {}

    fn start(&self) -> Self::Offset {
        self.lo
    }

    fn end(&self) -> Self::Offset {
        self.hi
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SpanWithFile(pub Symbol, pub Span);

impl ariadne::Span for SpanWithFile {
    type SourceId = Symbol;

    fn source(&self) -> &Self::SourceId {
        &self.0
    }

    fn start(&self) -> usize {
        self.1.lo as usize
    }

    fn end(&self) -> usize {
        self.1.hi as usize
    }
}

impl From<(Symbol, Span)> for SpanWithFile {
    fn from(value: (Symbol, Span)) -> Self {
        Self(value.0, value.1)
    }
}

impl chumsky::span::Span for SpanWithFile {
    type Context = Symbol;

    type Offset = u32;

    fn new(context: Self::Context, range: Range<Self::Offset>) -> Self {
        Self(
            context,
            Span {
                lo: range.start,
                hi: range.end,
            },
        )
    }

    fn context(&self) -> Self::Context {
        self.0
    }

    fn start(&self) -> Self::Offset {
        self.1.lo
    }

    fn end(&self) -> Self::Offset {
        self.1.hi
    }
}
