//! # lineic - Flexible linear interpolator for Rust
//!
//! [![Crates.io](https://img.shields.io/crates/v/lineic.svg)](https://crates.io/crates/lineic/)
//! [![Build Status](https://github.com/rscarson/lineic/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/rscarson/lineic/actions?query=branch%3Amaster)
//! [![docs.rs](https://img.shields.io/docsrs/lineic)](https://docs.rs/lineic/latest/)
//! [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/lineic/master/LICENSE)
//!
//! ## lineic - Flexible linear interpolator for Rust
//!
//! This library provides a simple way to interpolate between values across a range.  
//! It supports N-dimensional values, mixed types, and interpolation across any number of data sets.
//!
//! Inverted ranges work fine, and out of range values are clamped to the provided range.
//!
//! The library is designed to be simple to use, and as flexible as possible;  
//! For use with non-standard types, the library provides a `Numeric` trait that can be implemented.
//!
//! ## Examples
//!
//! The simplest possible use of the library is mapping one range to another  
//! Here we can map values in the range `0.0..=10.0` to the range `30.0..=35.0`
//! ```rust
//! use lineic::interpolators::F32InterpolationBucket;
//! let interpolator = F32InterpolationBucket::new(0.0..=10.0, [30.0], [35.0]);
//! assert_eq!(interpolator.interpolate(5.0), [32.5]);
//! ```
//!
//! -----
//!
//! The target does not have to be a single value - here we interpolate across a pair of RGB values  
//! The result is a smooth gradient from `red` to `green` for values in the range `0.0..=10.0`
//! ```rust
//! use lineic::interpolators::F32InterpolationBucket;
//! let interpolator = F32InterpolationBucket::new(0.0..=10.0, [255.0, 0.0, 0.0], [0.0, 255.0, 0.0]);
//! assert_eq!(interpolator.interpolate(5.0), [127.5, 127.5, 0.0]);
//! ```
//!
//! -----
//!
//! The library can also interpolate smoothly across multiple pairs of values  
//! This example forms a sort of traffic light sequence, interpolating between `red`, `yellow`, and `green`
//!
//! The range is reversed here to demonstrate that the library can handle that
//!
//! ```rust
//! use lineic::interpolators::F32LinearInterpolator;
//! ;
//! let interpolator = F32LinearInterpolator::new(
//!     10.0..=0.0,
//!     &[[0.0, 255.0, 0.0], [255.0, 255.0, 0.0], [255.0, 0.0, 0.0]],
//! );
//! assert_eq!(interpolator.interpolate(5.0), [255.0, 255.0, 0.0]);
//! assert_eq!(interpolator.interpolate(0.0), [255.0, 0.0, 0.0]);
//! ```
//!
//! -----
//!
//! The types for the range and values do not need to the same  
//! Here a `f64` range is used to interpolate across `u8` values
//! ```rust
//! use lineic::LinearInterpolator;
//!
//! let interpolator: LinearInterpolator<'_, 3, f64, u8> =
//!     LinearInterpolator::new(0.0..=10.0, &[[0, 255, 0], [255, 255, 0], [255, 0, 0]]);
//!
//! assert_eq!(interpolator.interpolate(5.0), [255, 255, 0]);
//! assert_eq!(interpolator.interpolate(0.0), [0, 255, 0]);
//! ```
//!
//! By default, you can interpolate across the following types:
//! - `f32` `f64`
//! - `i8` `i16` `i32` `i64` `i128` `isize`
//! - `u8` `u16` `u32` `u64` `u128` `usize`
//!
//! For other types, you can implement the `Numeric` trait.  
//! See `examples/custom_types.rs` for an example of how to do this.
//!
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)] // Module's are not being exported so they are not being repeated

mod bucket;
pub use bucket::InterpolationBucket;

mod interpolator;
pub use interpolator::LinearInterpolator;

mod number;
pub use number::Numeric;

/// This module contains a set of same-type interpolator type aliases for common numeric types.
pub mod interpolators {
    use crate::{InterpolationBucket, LinearInterpolator};

    /// Interpolation bucket mapping f64 ranges to f64 values
    /// For more information, see [`InterpolationBucket`]
    pub type F64InterpolationBucket<const N: usize> = InterpolationBucket<N, f64, f64>;

    /// Linear interpolator for f64 values
    /// For more information, see [`LinearInterpolator`]
    pub type F64LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, f64, f64>;

    /// Interpolation bucket mapping f32 ranges to f32 values
    /// For more information, see [`InterpolationBucket`]
    pub type F32InterpolationBucket<const N: usize> = InterpolationBucket<N, f32, f32>;

    /// Linear interpolator for f32 values
    /// For more information, see [`LinearInterpolator`]
    pub type F32LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, f32, f32>;

    /// Interpolation bucket mapping i128 ranges to i128 values
    /// For more information, see [`InterpolationBucket`]
    pub type I128InterpolationBucket<const N: usize> = InterpolationBucket<N, i128, i128>;

    /// Linear interpolator for i128 values
    /// For more information, see [`LinearInterpolator`]
    pub type I128LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, i128, i128>;

    /// Interpolation bucket mapping i64 ranges to i64 values
    /// For more information, see [`InterpolationBucket`]
    pub type I64InterpolationBucket<const N: usize> = InterpolationBucket<N, i64, i64>;

    /// Linear interpolator for i64 values
    /// For more information, see [`LinearInterpolator`]
    pub type I64LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, i64, i64>;

    /// Interpolation bucket mapping i32 ranges to i32 values
    /// For more information, see [`InterpolationBucket`]
    pub type I32InterpolationBucket<const N: usize> = InterpolationBucket<N, i32, i32>;

    /// Linear interpolator for i32 values
    /// For more information, see [`LinearInterpolator`]
    pub type I32LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, i32, i32>;

    /// Interpolation bucket mapping i16 ranges to i16 values
    /// For more information, see [`InterpolationBucket`]
    pub type I16InterpolationBucket<const N: usize> = InterpolationBucket<N, i16, i16>;

    /// Linear interpolator for i16 values
    /// For more information, see [`LinearInterpolator`]
    pub type I16LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, i16, i16>;

    /// Interpolation bucket mapping i8 ranges to i8 values
    /// For more information, see [`InterpolationBucket`]
    pub type I8InterpolationBucket<const N: usize> = InterpolationBucket<N, i8, i8>;

    /// Linear interpolator for i8 values
    /// For more information, see [`LinearInterpolator`]
    pub type I8LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, i8, i8>;

    /// Interpolation bucket mapping isize ranges to isize values
    /// For more information, see [`InterpolationBucket`]
    pub type ISizeInterpolationBucket<const N: usize> = InterpolationBucket<N, isize, isize>;

    /// Linear interpolator for isize values
    /// For more information, see [`LinearInterpolator`]
    pub type ISizeLinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, isize, isize>;

    /// Interpolation bucket mapping u128 ranges to u128 values
    /// For more information, see [`InterpolationBucket`]
    pub type U128InterpolationBucket<const N: usize> = InterpolationBucket<N, u128, u128>;

    /// Linear interpolator for u128 values
    /// For more information, see [`LinearInterpolator`]
    pub type U128LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, u128, u128>;

    /// Interpolation bucket mapping u64 ranges to u64 values
    /// For more information, see [`InterpolationBucket`]
    pub type U64InterpolationBucket<const N: usize> = InterpolationBucket<N, u64, u64>;

    /// Linear interpolator for u64 values
    /// For more information, see [`LinearInterpolator`]
    pub type U64LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, u64, u64>;

    /// Interpolation bucket mapping u32 ranges to u32 values
    /// For more information, see [`InterpolationBucket`]
    pub type U32InterpolationBucket<const N: usize> = InterpolationBucket<N, u32, u32>;

    /// Linear interpolator for u32 values
    /// For more information, see [`LinearInterpolator`]
    pub type U32LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, u32, u32>;

    /// Interpolation bucket mapping u16 ranges to u16 values
    /// For more information, see [`InterpolationBucket`]
    pub type U16InterpolationBucket<const N: usize> = InterpolationBucket<N, u16, u16>;

    /// Linear interpolator for u16 values
    /// For more information, see [`LinearInterpolator`]
    pub type U16LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, u16, u16>;

    /// Interpolation bucket mapping u8 ranges to u8 values
    /// For more information, see [`InterpolationBucket`]
    pub type U8InterpolationBucket<const N: usize> = InterpolationBucket<N, u8, u8>;

    /// Linear interpolator for u8 values
    /// For more information, see [`LinearInterpolator`]
    pub type U8LinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, u8, u8>;

    /// Interpolation bucket mapping usize ranges to usize values
    /// For more information, see [`InterpolationBucket`]
    pub type USizeInterpolationBucket<const N: usize> = InterpolationBucket<N, usize, usize>;

    /// Linear interpolator for usize values
    /// For more information, see [`LinearInterpolator`]
    pub type USizeLinearInterpolator<'a, const N: usize> = LinearInterpolator<'a, N, usize, usize>;
}
