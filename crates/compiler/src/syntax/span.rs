use std::fmt;
use std::ops::Range;

use crate::symbol::{self, Symbol};

/// The location in a slice in which some object spans.
// TODO(@ThePuzzlemaker): optimize this using interning
#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Span {
    lo: u32,
    hi: u32,
    pub file: Symbol,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}..{}", self.file, self.lo, self.hi)
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.file != other.file {
            return None;
        }
        match self.lo.partial_cmp(&other.lo) {
            Some(std::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.hi.partial_cmp(&other.hi) {
            Some(std::cmp::Ordering::Equal) => Some(std::cmp::Ordering::Equal),
            ord => ord,
        }
    }
}

impl Span {
    /// Create a new span given the low and high indices.
    #[must_use]
    #[inline]
    pub fn new(lo: u32, hi: u32, file: Symbol) -> Self {
        Span { lo, hi, file }
    }

    /// Create a new empty span around an index.
    #[must_use]
    #[inline]
    pub fn new_shrunk(amount: u32, file: Symbol) -> Self {
        Span {
            lo: amount,
            hi: amount,
            file,
        }
    }

    /// Create a dummy span (that has a span equivalent to the range `0..0`).
    #[must_use]
    #[inline]
    pub fn new_dummy() -> Self {
        Self {
            lo: 0,
            hi: 0,
            file: *symbol::special::EMPTY,
        }
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
        if self.is_dummy() { other } else { self }
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
            self.file,
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
        Span::new(self.hi, end.lo, self.file)
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
        Span::new(self.lo, end.lo, self.file)
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

impl From<(Range<u32>, Symbol)> for Span {
    fn from((range, file): (Range<u32>, Symbol)) -> Self {
        Span::new(range.start, range.end, file)
    }
}

impl ariadne::Span for Span {
    type SourceId = Symbol;

    fn source(&self) -> &Self::SourceId {
        &self.file
    }

    fn start(&self) -> usize {
        self.lo as usize
    }

    fn end(&self) -> usize {
        self.hi as usize
    }
}
