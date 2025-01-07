//! This example shows how to interpolate over a non-standard type  
//! Here we create a `NumericUnicode` type that allows us to interpolate across unicode code points.
use lineic::LinearInterpolator;

fn main() {
    //
    // An interpolator using the NumericUnicode type
    let interpolator: LinearInterpolator<1, u8, NumericUnicode> =
        LinearInterpolator::new(0..=26, &[['a'.into()], ['z'.into()]]);

    //
    // Perform an interpolation
    let result = interpolator.interpolate(17);
    println!(
        "The 17th letter of the alphabet is: {:?}",
        NumericUnicode::to_string(&result)
    );
}

/// A type that allows iterating over unicode code points without hitting invalid code points.
/// This is just an example type used to demo the custom types feature.
#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash)]
struct NumericUnicode(u32);
impl NumericUnicode {
    /// Convert a slice of NumericUnicode values into a string
    fn to_string(values: &[Self]) -> String {
        values.iter().map(|v| char::from(*v)).collect()
    }
}
impl From<char> for NumericUnicode {
    fn from(value: char) -> NumericUnicode {
        NumericUnicode(value as u32)
    }
}
impl From<NumericUnicode> for char {
    fn from(value: NumericUnicode) -> char {
        // Is the inner value a valid unicode code point?
        if let Some(c) = std::char::from_u32(value.0) {
            c
        } else {
            // Pin to nearest valid unicode code point
            let mut distance = 1;
            loop {
                let below = value.0.checked_sub(distance);
                if let Some(c) = below.and_then(std::char::from_u32) {
                    break c;
                }

                let above = value.0.checked_add(distance);
                if let Some(c) = above.and_then(std::char::from_u32) {
                    break c;
                }

                distance += 1;
            }
        }
    }
}

// Implement the Numeric trait for NumericUnicode, so we can interpolate between them.
impl lineic::Numeric for NumericUnicode {
    // Some constants the interpolator will use
    const MAX: Self = NumericUnicode(u32::MAX);
    const ZERO: Self = NumericUnicode(0);
    const ONE: Self = NumericUnicode(1);

    // Get the absolute value of this number
    fn abs(self) -> Self {
        self
    }

    // Clamp this number between a minimum and maximum value
    // Without panicking if max < min
    fn clamp(self, min: Self, max: Self) -> Self {
        Self(if min > max {
            std::cmp::Ord::clamp(self.0, min.0, max.0)
        } else {
            std::cmp::Ord::clamp(self.0, max.0, min.0)
        })
    }

    //
    // These are methods instead of a trait requirement for better stdlib compatibility
    //

    fn from_usize(value: usize) -> Option<Self> {
        u32::try_from(value).ok().map(Self)
    }

    fn into_f64(self) -> f64 {
        self.0 as f64
    }

    fn from_f64(value: f64) -> Option<Self> {
        if value < u32::MAX as f64 && value >= 0.0 {
            Some(Self(value as u32))
        } else {
            None
        }
    }

    //
    // Checked arithmetic operations
    //

    fn checked_sub(self, other: Self) -> Option<Self> {
        self.0.checked_sub(other.0).map(Self)
    }

    fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    fn checked_mul(self, other: Self) -> Option<Self> {
        self.0.checked_mul(other.0).map(Self)
    }

    fn checked_div(self, other: Self) -> Option<Self> {
        self.0.checked_div(other.0).map(Self)
    }
}
