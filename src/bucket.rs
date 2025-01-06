use crate::number::Numeric;
use std::ops::RangeInclusive;

/// A value set for interpolation.
/// Interpolates between 2 sets of values based on a range.
///
/// For interpolating between more than 2 data sets, see [`LinearInterpolator`].
///
/// # Example
/// ```rust
/// use lineic::InterpolationBucket;
///
/// const RED: [u8; 3] = [0xB8, 0x1D, 0x13];
/// const GRN: [u8; 3] = [0x00, 0x84, 0x50];
///
/// let bucket = InterpolationBucket::new(0.0..=100.0, RED, GRN);
///
/// // Interpolate between RED and GRN at 50% of the range
/// let interpolated = bucket.interpolate(50.0);
/// ```
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct InterpolationBucket<const N: usize, S: Numeric, T: Numeric> {
    range: RangeInclusive<S>,
    values_lo: [T; N],
    values_hi: [T; N],
}
impl<const N: usize, S: Numeric, T: Numeric> InterpolationBucket<N, S, T> {
    /// Create a new interpolation bucket.
    /// `range` is the range of values that this bucket interpolates between.
    /// `values_lo` is the set of values to interpolate from.
    /// `values_hi` is the set of values to interpolate to.
    ///
    /// This will enable the bucket to smoothly interpolate from lo to hi for T values in the range.
    /// Values < range min will be clamped to lo.
    /// Values > range max will be clamped to hi.
    pub const fn new(range: RangeInclusive<S>, values_lo: [T; N], values_hi: [T; N]) -> Self {
        Self {
            range,
            values_lo,
            values_hi,
        }
    }

    /// Get the range of values that this bucket interpolates between.
    pub fn range(&self) -> &RangeInclusive<S> {
        &self.range
    }

    /// Get the start and end values of the range.
    pub fn start(&self) -> &S {
        self.range.start()
    }

    /// Get the start and end values of the range.
    pub fn end(&self) -> &S {
        self.range.end()
    }

    /// Get the set of values to interpolate from.
    pub fn values_lo(&self) -> &[T; N] {
        &self.values_lo
    }

    /// Get the set of values to interpolate to.
    pub fn values_hi(&self) -> &[T; N] {
        &self.values_hi
    }

    /// Interpolate between the 2 value sets of this bucket at the given `t` value.
    /// This will return a new set of values that are interpolated between `values_lo` and `values_hi` based on `t`'s position in the bucket's range.
    pub fn interpolate(&self, s: S) -> [T; N] {
        let start: S = *self.start();
        let end = *self.end();
        let lo = &self.values_lo;
        let hi = &self.values_hi;

        let len = start.abs_diff(end);
        let value = s.clamp(start, end);
        let rel_value = value.abs_diff(start);
        let rel_percent = rel_value.into_f64() / len.into_f64();

        let mut values = *lo;
        for (i, value) in values.iter_mut().enumerate() {
            let diff = lo[i].abs_diff(hi[i]);
            let adj = diff.scale(rel_percent);

            *value = if lo[i] > hi[i] {
                lo[i].checked_sub(adj).unwrap_or(T::ZERO)
            } else {
                lo[i].checked_add(adj).unwrap_or(T::MAX)
            };
        }

        values
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpolation_bucket() {
        const RED: [u8; 3] = [255, 50, 50];
        const GRN: [u8; 3] = [50, 255, 50];

        let bucket = InterpolationBucket::new(0.0..=1.0, RED, GRN);
        let back_bucket = InterpolationBucket::new(1.0..=0.0, GRN, RED);

        // Interpolate between RED and GRN at 50% of the range
        let interpolated = bucket.interpolate(0.6);
        assert_eq!(interpolated, [132, 173, 50]);

        // Backwards interpolation should be ~same as forwards interpolation
        let back_interpolated = back_bucket.interpolate(0.6);
        assert_eq!(back_interpolated, [132, 173, 50]);
    }
}
