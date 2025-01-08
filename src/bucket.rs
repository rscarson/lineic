use crate::{number::Numeric, ReversibleRange};

/// A value set for interpolation.  
/// Interpolates between 2 sets of values based on a range.
///
/// For interpolating between more than 2 data sets, see [`crate::LinearInterpolator`].
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
    range: ReversibleRange<S>,
    values_lo: [T; N],
    values_hi: [T; N],
}
impl<const N: usize, S: Numeric, T: Numeric> InterpolationBucket<N, S, T> {
    /// Create a new interpolation bucket.  
    /// - `range` is the range of values that this bucket interpolates between.
    /// - `values_lo` is the set of values to interpolate from.
    /// - `values_hi` is the set of values to interpolate to.
    ///
    /// This will enable the bucket to smoothly interpolate from lo to hi for T values in the range.  
    /// Values < range min will be clamped to lo.  
    /// Values > range max will be clamped to hi.
    pub fn new(range: impl Into<ReversibleRange<S>>, values_lo: [T; N], values_hi: [T; N]) -> Self {
        let range = range.into();
        Self {
            range,
            values_lo,
            values_hi,
        }
    }

    /// Create a new interpolation bucket.  
    /// - `range` is the range of values that this bucket interpolates between.
    /// - `values_lo` is the set of values to interpolate from.
    /// - `values_hi` is the set of values to interpolate to.
    ///
    /// This will enable the bucket to smoothly interpolate from lo to hi for T values in the range.  
    /// Values < range min will be clamped to lo.  
    /// Values > range max will be clamped to hi.
    pub const fn new_const(range: (S, S), values_lo: [T; N], values_hi: [T; N]) -> Self {
        let range = ReversibleRange::new(range.0, range.1);
        Self {
            range,
            values_lo,
            values_hi,
        }
    }

    /// Get the range of values that this bucket interpolates between.
    pub fn range(&self) -> &ReversibleRange<S> {
        &self.range
    }

    /// Get the start value of the range.
    pub fn start(&self) -> S {
        self.range.start
    }

    /// Get the end value of the range.
    pub fn end(&self) -> S {
        self.range.end
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
        let start: S = self.start();
        let end = self.end();
        let lo = &self.values_lo;
        let hi = &self.values_hi;

        let len = self.range.len();
        let value = s.clamp(start, end);
        let rel_value = value.abs_diff(start);
        let rel_percent = rel_value.into_f64() / len.into_f64();

        let mut values = *lo;
        for (i, value) in values.iter_mut().enumerate() {
            let diff = lo[i].abs_diff(hi[i]);
            let adj = diff.scale(rel_percent).unwrap_or(T::MAX);

            *value = if lo[i] > hi[i] {
                lo[i].checked_sub(adj).unwrap_or(T::ZERO)
            } else {
                lo[i].checked_add(adj).unwrap_or(T::MAX)
            };
        }

        values
    }

    /// Attempt to retrieve the value within the bucket's range that would produce the given set of values.
    pub fn reverse_interpolate(&self, input: &[T; N]) -> Option<S> {
        const DIFF_FLOOR: f64 = 1e-6; // Percentage difference below which values are considered equal

        let start = self.start();
        let end = self.end();
        let len = self.end().abs_diff(start);

        let mut rel_percent = None;
        for (i, input) in input.iter().enumerate() {
            if *input != input.clamp(self.values_lo[i], self.values_hi[i]) {
                return None; // Out of bounds
            }

            let diff = self.values_lo[i].abs_diff(self.values_hi[i]).into_f64();
            let diff2 = self.values_lo[i].abs_diff(*input).into_f64();
            let min = diff.min(diff2);
            let max = diff.max(diff2);
            let percent = min / max;

            if diff == 0.0 && diff2 == 0.0 {
                continue; // No difference
            }

            if let Some(p) = rel_percent {
                if f64::abs(p - percent) > DIFF_FLOOR {
                    return None; // Not a linear interpolation
                }
            } else {
                rel_percent = Some(percent);
            }
        }

        let mut rel_percent = rel_percent?;

        if self.start() > self.end() {
            rel_percent = 1.0 - rel_percent;
        }

        if start < end {
            start.checked_add(len.scale(rel_percent)?)
        } else {
            end.checked_add(len.scale(rel_percent)?)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_interpolation_bucket() {
        const RED: [u8; 3] = [255, 50, 50];
        const GRN: [u8; 3] = [50, 255, 50];

        let bucket = InterpolationBucket::new((0.0, 1.0), RED, GRN);
        let back_bucket = InterpolationBucket::new((1.0, 0.0), GRN, RED);

        // Interpolate between RED and GRN at 50% of the range
        let interpolated = bucket.interpolate(0.6);
        assert_eq!(interpolated, [132, 173, 50]);
        assert_eq!(bucket.reverse_interpolate(&interpolated), Some(0.6));

        // Backwards interpolation should be ~same as forwards interpolation
        let back_interpolated = back_bucket.interpolate(0.6);
        assert_eq!(back_interpolated, [132, 173, 50]);
        assert_eq!(
            back_bucket.reverse_interpolate(&back_interpolated),
            Some(0.6)
        );
    }
}
