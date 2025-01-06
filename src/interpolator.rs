use crate::{number::Numeric, InterpolationBucket};
use std::{borrow::Cow, ops::RangeInclusive};

/// A linear interpolator for a set of values.
/// Interpolates between a series of discrete value sets based on a range.
///
/// For example a traffic light system could be represented as:
/// ```rust
/// use lineic::LinearInterpolator;
///
/// const RED: [u8; 3] = [0xB8, 0x1D, 0x13];
/// const YLW: [u8; 3] = [0xEF, 0xB7, 0x00];
/// const GRN: [u8; 3] = [0x00, 0x84, 0x50];
///
/// let interpolator = LinearInterpolator::new(0.0..=100.0, &[RED, YLW, GRN]);
///
/// /*
/// The result will be a linear interpolation between:
/// 0..=50 => RED->YLW
/// 50..=100 => YLW->GRN
/// */
///
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub struct LinearInterpolator<'a, const N: usize, S: Numeric, T: Numeric> {
    buckets: Cow<'a, [InterpolationBucket<N, S, T>]>,
    is_reversed: bool,
}
impl<'a, const N: usize, S: Numeric, T: Numeric> LinearInterpolator<'a, N, S, T> {
    /// Create a new linear interpolator with the given range and value sets.
    ///
    /// # Panics
    /// Panics if the number of value sets is too large to be represented by type S
    pub fn new(range: RangeInclusive<S>, value_sets: &[[T; N]]) -> Self {
        Self::try_new(range, value_sets).expect("Number of value sets too large to fit in type S")
    }

    /// Create a new linear interpolator with the given range and value sets.
    ///
    /// Returns None if the number of value sets is too large to be represented by type S
    pub fn try_new(range: RangeInclusive<S>, value_sets: &[[T; N]]) -> Option<Self> {
        if value_sets.is_empty() {
            let is_reversed = *range.start() > *range.end();
            let buckets = Cow::Owned(vec![InterpolationBucket::new(
                range,
                [T::ZERO; N],
                [T::ZERO; N],
            )]);
            return Some(Self {
                buckets,
                is_reversed,
            });
        }

        let capacity = value_sets.len() - 1;
        let mut buckets = Vec::with_capacity(capacity);
        let is_reversed = *range.start() > *range.end();

        // Noop interpolation
        if capacity == 0 {
            let values = value_sets[0];
            buckets.push(InterpolationBucket::new(range, values, values));
            let buckets = Cow::Owned(buckets);
            return Some(Self {
                buckets,
                is_reversed,
            });
        }

        let len = range.start().abs_diff(*range.end());
        let step_by = len.checked_div(S::from_usize(capacity)?)?;

        let mut start = *range.start();
        for i in 0..capacity {
            let is_last = i == value_sets.len() - 2;

            let end = if is_last {
                *range.end()
            } else if is_reversed {
                start.checked_sub(step_by).unwrap_or(S::ZERO)
            } else {
                start.checked_add(step_by).unwrap_or(S::MAX)
            };
            let range = start..=end;

            let values_lo = value_sets[i];
            let values_hi = value_sets[i + 1];

            buckets.push(InterpolationBucket::new(range, values_lo, values_hi));
            start = end;
        }

        let buckets = Cow::Owned(buckets);
        Some(Self {
            buckets,
            is_reversed,
        })
    }

    /// Create a new linear interpolator from a raw slice of buckets.
    ///
    /// # Safety
    /// The buckets must form a contiguous range of values.
    /// The buckets must be sorted by range
    /// If the ranges are sorted in descending order, `is_reversed` should be true.
    pub const unsafe fn new_from_raw(
        buckets: &'a [InterpolationBucket<N, S, T>],
        is_reversed: bool,
    ) -> Self {
        let buckets = Cow::Borrowed(buckets);
        Self {
            buckets,
            is_reversed,
        }
    }

    /// Get the set of discrete interpolations this interpolator will use.
    #[must_use]
    pub fn buckets(&self) -> &[InterpolationBucket<N, S, T>] {
        &self.buckets
    }

    /// Returns the bucket that contains the given value.
    pub fn get_bucket(&self, s: S) -> &InterpolationBucket<N, S, T> {
        let rev = self.is_reversed;
        let mut slice = self.buckets();

        // Binary search for the bucket that contains the value
        while slice.len() > 1 {
            let mid = slice.len() / 2;
            let mid_bucket = &slice[mid];
            if (!rev && s >= *mid_bucket.start()) || (rev && s <= *mid_bucket.start()) {
                slice = &slice[mid..];
            } else {
                slice = &slice[..mid];
            }
        }

        &slice[0]
    }

    /// Interpolate between the value sets based on the given value.
    pub fn interpolate(&self, s: S) -> [T; N] {
        let bucket = self.get_bucket(s);
        bucket.interpolate(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    #[allow(clippy::unreadable_literal)]
    fn test_new() {
        let interpolator =
            LinearInterpolator::new(0.0..=100.0, &[[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]);
        assert_eq!(interpolator.buckets().len(), 2);
        assert_eq!(
            interpolator.buckets()[0],
            InterpolationBucket::new(0.0..=50.0, [0.0, 0.0], [1.0, 1.0])
        );
        assert_eq!(
            interpolator.buckets()[1],
            InterpolationBucket::new(50.0..=100.0, [1.0, 1.0], [2.0, 2.0])
        );

        let interpolator = LinearInterpolator::new(
            100.0..=0.0,
            &[[0.0, 0.0], [1.0, 1.0], [2.0, 2.0], [3.0, 3.0]],
        );
        assert_eq!(interpolator.buckets().len(), 3);
        assert_eq!(
            interpolator.buckets()[0],
            InterpolationBucket::new(100.0..=66.66666666666666, [0.0, 0.0], [1.0, 1.0])
        );

        let empty = LinearInterpolator::<0, f64, f64>::new(0.0..=0.0, &[]);
        assert_eq!(empty.interpolate(0.0), []);
    }

    #[test]
    #[allow(clippy::unreadable_literal)]
    fn test_get_bucket() {
        let interpolator =
            LinearInterpolator::new(0.0..=100.0, &[[0.0, 0.0], [1.0, 1.0], [2.0, 2.0]]);
        assert_eq!(
            interpolator.get_bucket(0.0),
            &InterpolationBucket::new(0.0..=50.0, [0.0, 0.0], [1.0, 1.0])
        );
        assert_eq!(
            interpolator.get_bucket(50.0),
            &InterpolationBucket::new(50.0..=100.0, [1.0, 1.0], [2.0, 2.0])
        );
        assert_eq!(
            interpolator.get_bucket(100.0),
            &InterpolationBucket::new(50.0..=100.0, [1.0, 1.0], [2.0, 2.0])
        );

        let interpolator = LinearInterpolator::new(
            100.0..=0.0,
            &[[0.0, 0.0], [1.0, 1.0], [2.0, 2.0], [3.0, 3.0]],
        );

        assert_eq!(
            interpolator.get_bucket(100.0),
            &InterpolationBucket::new(100.0..=66.66666666666666, [0.0, 0.0], [1.0, 1.0])
        );

        assert_eq!(
            interpolator.get_bucket(20.0),
            &InterpolationBucket::new(33.33333333333332..=0.0, [2.0, 2.0], [3.0, 3.0])
        );
    }
}
