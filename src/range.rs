use crate::Numeric;

/// An inclusive total range that can be used in reverse order
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct ReversibleRange<S: Numeric> {
    /// The starting point of the range
    /// Does not need to be < end
    pub start: S,

    /// The ending point of the range
    /// Does not need to be > start
    pub end: S,
}
impl<S: Numeric> ReversibleRange<S> {
    /// Create a new range from a start and end value
    /// The values do not need to be in order
    pub const fn new(from: S, to: S) -> Self {
        Self {
            start: from,
            end: to,
        }
    }

    /// Check if the range contains the given value
    /// Returns true if value is between the start and end values
    pub fn contains(&self, value: S) -> bool {
        (self.start <= value && value <= self.end) || (self.end <= value && value <= self.start)
    }

    /// Check if the range is empty
    /// Returns true if the start and end values are the same
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }

    /// Get the length of the range
    pub fn len(&self) -> S {
        self.start.abs_diff(self.end)
    }

    /// Check if the range is reversed
    /// Returns true if the start value is greater than the end value
    pub fn is_reversed(&self) -> bool {
        self.start > self.end
    }
}

impl<S> From<[S; 2]> for ReversibleRange<S>
where
    S: Numeric,
{
    fn from(range: [S; 2]) -> Self {
        Self {
            start: range[0],
            end: range[1],
        }
    }
}

impl<S> From<(S, S)> for ReversibleRange<S>
where
    S: Numeric,
{
    fn from(range: (S, S)) -> Self {
        Self {
            start: range.0,
            end: range.1,
        }
    }
}

#[cfg(not(feature = "no_std"))]
impl<S: Numeric> From<std::ops::RangeInclusive<S>> for ReversibleRange<S> {
    fn from(range: std::ops::RangeInclusive<S>) -> Self {
        Self {
            start: *range.start(),
            end: *range.end(),
        }
    }
}
