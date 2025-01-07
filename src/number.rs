// A few lints related to precision loss
// TODO: handle better
#![allow(clippy::cast_possible_truncation)]
//
// This only happens when the user explicitely selects an unsigned type for the range
#![allow(clippy::cast_sign_loss)]
//
//The loss of precision is acceptable, and unavoidable for some user-selected type combos
#![allow(clippy::cast_lossless)]
#![allow(clippy::cast_precision_loss)]

/// Represents a numeric type that can be interpolated across
/// By default, implemented for:
/// - `f32` `f64`
/// - `i8` `i16` `i32` `i64` `i128` `isize`
/// - `u8` `u16` `u32` `u64` `u128` `usize`
pub trait Numeric: Copy + std::fmt::Debug + PartialOrd {
    /// The maximum value for this type
    const MAX: Self;

    /// The zero value for this type
    const ZERO: Self;

    /// The first whole value after zero for this type
    const ONE: Self;

    /// Get the absolute value of this number
    #[must_use]
    fn abs(self) -> Self;

    /// Clamp this number between a minimum and maximum value.  
    /// Differs from `std::cmp::Ord::clamp` in that it must handle cases where `min > max`
    #[must_use]
    fn clamp(self, min: Self, max: Self) -> Self;

    /// Get the distance between two numbers, always as a positive value
    #[must_use]
    fn abs_diff(self, other: Self) -> Self {
        if self > other {
            self.checked_sub(other).unwrap_or(Self::ZERO)
        } else {
            other.checked_sub(self).unwrap_or(Self::ZERO)
        }
    }

    /// Scale this number by a factor of a different numeric type.
    ///
    /// The built-in implementation uses f64 as a common go-between for scaling
    #[must_use]
    fn scale(self, factor: impl Numeric) -> Option<Self> {
        let f = self.into_f64() * factor.into_f64();
        Self::from_f64(f)
    }

    /// Subtract another number from this one, returning None if the operation would overflow
    #[must_use]
    fn checked_sub(self, other: Self) -> Option<Self>;

    /// Add two numbers together, returning None if the operation would overflow
    #[must_use]
    fn checked_add(self, other: Self) -> Option<Self>;

    /// Multiply two numbers together, returning None if the operation would overflow
    #[must_use]
    fn checked_mul(self, other: Self) -> Option<Self>;

    /// Divide two numbers, returning None if the operation would overflow
    #[must_use]
    fn checked_div(self, other: Self) -> Option<Self>;

    /// Convert a usize to this type
    fn from_usize(value: usize) -> Option<Self>;

    /// Convert this number to an f64
    fn into_f64(self) -> f64;

    /// Convert an f64 to this type
    fn from_f64(value: f64) -> Option<Self>;
}

macro_rules! auto_impl_u {
    ($t:ty) => {
        impl Numeric for $t {
            const MAX: Self = <$t>::MAX;
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn abs(self) -> Self {
                self
            }

            fn clamp(self, min: Self, max: Self) -> Self {
                if min < max {
                    std::cmp::Ord::clamp(self, min, max)
                } else {
                    std::cmp::Ord::clamp(self, max, min)
                }
            }

            fn checked_sub(self, other: Self) -> Option<Self> {
                self.checked_sub(other)
            }

            fn checked_add(self, other: Self) -> Option<Self> {
                self.checked_add(other)
            }

            fn checked_mul(self, other: Self) -> Option<Self> {
                self.checked_mul(other)
            }

            fn checked_div(self, other: Self) -> Option<Self> {
                self.checked_div(other)
            }

            fn from_usize(value: usize) -> Option<Self> {
                Self::try_from(value).ok()
            }

            fn into_f64(self) -> f64 {
                self as f64
            }

            fn from_f64(value: f64) -> Option<Self> {
                if value <= <$t>::MAX as f64 && value >= 0.0 {
                    Some(value as Self)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! auto_impl_i {
    ($t:ty) => {
        impl Numeric for $t {
            const MAX: Self = <$t>::MAX;
            const ZERO: Self = 0;
            const ONE: Self = 1;

            fn abs(self) -> Self {
                <$t>::abs(self)
            }

            fn clamp(self, min: Self, max: Self) -> Self {
                if min < max {
                    std::cmp::Ord::clamp(self, min, max)
                } else {
                    std::cmp::Ord::clamp(self, max, min)
                }
            }

            fn checked_sub(self, other: Self) -> Option<Self> {
                self.checked_sub(other)
            }

            fn checked_add(self, other: Self) -> Option<Self> {
                self.checked_add(other)
            }

            fn checked_mul(self, other: Self) -> Option<Self> {
                self.checked_mul(other)
            }

            fn checked_div(self, other: Self) -> Option<Self> {
                self.checked_div(other)
            }

            fn from_usize(value: usize) -> Option<Self> {
                Self::try_from(value).ok()
            }

            fn into_f64(self) -> f64 {
                self as f64
            }

            fn from_f64(value: f64) -> Option<Self> {
                if value <= <$t>::MAX as f64 && value >= <$t>::MIN as f64 {
                    Some(value as Self)
                } else {
                    None
                }
            }
        }
    };
}

macro_rules! auto_impl_f {
    ($t:ty) => {
        impl Numeric for $t {
            const MAX: Self = <$t>::MAX;
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;

            fn abs(self) -> Self {
                <$t>::abs(self)
            }

            fn clamp(self, min: Self, max: Self) -> Self {
                if min < max {
                    <$t>::clamp(self, min, max)
                } else {
                    <$t>::clamp(self, max, min)
                }
            }

            fn checked_sub(self, other: Self) -> Option<Self> {
                Some(self - other)
            }

            fn checked_add(self, other: Self) -> Option<Self> {
                Some(self + other)
            }

            fn checked_mul(self, other: Self) -> Option<Self> {
                Some(self * other)
            }

            fn checked_div(self, other: Self) -> Option<Self> {
                Some(self / other)
            }

            fn from_usize(value: usize) -> Option<Self> {
                if value <= <$t>::MAX as usize {
                    Some(value as Self)
                } else {
                    None
                }
            }

            fn into_f64(self) -> f64 {
                self as f64
            }

            fn from_f64(value: f64) -> Option<Self> {
                Some(value as Self)
            }
        }
    };
}

auto_impl_f!(f32);
auto_impl_f!(f64);
auto_impl_i!(i8);
auto_impl_i!(i16);
auto_impl_i!(i32);
auto_impl_i!(i64);
auto_impl_i!(i128);
auto_impl_i!(isize);
auto_impl_u!(u8);
auto_impl_u!(u16);
auto_impl_u!(u32);
auto_impl_u!(u64);
auto_impl_u!(u128);
auto_impl_u!(usize);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_abs() {
        // floats
        assert_eq!(Numeric::abs(-1.0f32), 1.0);
        assert_eq!(Numeric::abs(1.0f32), 1.0);

        // signed integers
        assert_eq!(Numeric::abs(-1i8), 1);
        assert_eq!(Numeric::abs(1i8), 1);

        // unsigned integers
        assert_eq!(Numeric::abs(1u8), 1);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_clamp() {
        // floats
        assert_eq!(Numeric::clamp(1.0f64, 0.0, 2.0), 1.0);
        assert_eq!(Numeric::clamp(1.0f32, -1.0, 2.0), 1.0);

        // signed integers
        assert_eq!(Numeric::clamp(1i8, 0, 2), 1);
        assert_eq!(Numeric::clamp(1i8, -1, 2), 1);

        // unsigned integers
        assert_eq!(Numeric::clamp(1u8, 0, 2), 1);
    }

    #[test]
    #[allow(clippy::float_cmp)]
    fn test_scale() {
        // floats
        assert_eq!(Numeric::scale(1.0f64, 2.0f64), Some(2.0f64));
        assert_eq!(Numeric::scale(1.0f32, 0.5f32), Some(0.5f32));

        // signed integers
        assert_eq!(Numeric::scale(1i8, 2), Some(2));
        assert_eq!(Numeric::scale(2i8, 0.5), Some(1));

        // unsigned integers
        assert_eq!(Numeric::scale(1u8, 2), Some(2));
        assert_eq!(Numeric::scale(2u8, 0.5), Some(1));
    }
}
