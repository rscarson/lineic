<!-- cargo-rdme start -->

# lineic - Flexible linear interpolator for Rust

[![Crates.io](https://img.shields.io/crates/v/lineic.svg)](https://crates.io/crates/lineic/)
[![Build Status](https://github.com/rscarson/lineic/actions/workflows/tests.yml/badge.svg?branch=master)](https://github.com/rscarson/lineic/actions?query=branch%3Amaster)
[![docs.rs](https://img.shields.io/docsrs/lineic)](https://docs.rs/lineic/latest/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/rscarson/lineic/master/LICENSE)

## lineic - Flexible linear interpolator for Rust

This library provides a simple way to interpolate between values across a range.  
It supports N-dimensional values, mixed types, and interpolation across any number of data sets.

Inverted ranges work fine, and out of range values are clamped to the provided range.

The library is designed to be simple to use, and as flexible as possible;  
For use with non-standard types, the library provides a `Numeric` trait that can be implemented.

## Examples

The simplest possible use of the library is mapping one range to another  
Here we can map values in the range `0.0..=10.0` to the range `30.0..=35.0`
```rust
use lineic::interpolators::F32InterpolationBucket;
let interpolator = F32InterpolationBucket::new(0.0..=10.0, [30.0], [35.0]);
assert_eq!(interpolator.interpolate(5.0), [32.5]);
```

-----

The target does not have to be a single value - here we interpolate across a pair of RGB values  
The result is a smooth gradient from `red` to `green` for values in the range `0.0..=10.0`
```rust
use lineic::interpolators::F32InterpolationBucket;
let interpolator = F32InterpolationBucket::new(0.0..=10.0, [255.0, 0.0, 0.0], [0.0, 255.0, 0.0]);
assert_eq!(interpolator.interpolate(5.0), [127.5, 127.5, 0.0]);
```

-----

The library can also interpolate smoothly across multiple pairs of values  
This example forms a sort of traffic light sequence, interpolating between `red`, `yellow`, and `green`

The range is reversed here to demonstrate that the library can handle that

```rust
use lineic::interpolators::F32LinearInterpolator;
;
let interpolator = F32LinearInterpolator::new(
    10.0..=0.0,
    &[[0.0, 255.0, 0.0], [255.0, 255.0, 0.0], [255.0, 0.0, 0.0]],
);
assert_eq!(interpolator.interpolate(5.0), [255.0, 255.0, 0.0]);
assert_eq!(interpolator.interpolate(0.0), [255.0, 0.0, 0.0]);
```

-----

The types for the range and values do not need to the same  
Here a `f64` range is used to interpolate across `u8` values
```rust
use lineic::LinearInterpolator;

let interpolator: LinearInterpolator<'_, 3, f64, u8> =
    LinearInterpolator::new(0.0..=10.0, &[[0, 255, 0], [255, 255, 0], [255, 0, 0]]);

assert_eq!(interpolator.interpolate(5.0), [255, 255, 0]);
assert_eq!(interpolator.interpolate(0.0), [0, 255, 0]);
```

By default, you can interpolate across the following types:
- `f32` `f64`
- `i8` `i16` `i32` `i64` `i128` `isize`
- `u8` `u16` `u32` `u64` `u128` `usize`

For other types, you can implement the `Numeric` trait.  
See `examples/custom_types.rs` for an example of how to do this.

<!-- cargo-rdme end -->
