#![doc = include_str!("../README.md")]

#![allow(non_snake_case)]
#![warn(missing_docs)]

/// Floating-point numeric types
pub mod float;

/// Distance computations
pub mod distance;

/// Specify the algorithms of distance computations
pub mod metric;

pub use crate::float::*;
pub use crate::distance::*;
pub use crate::metric::*;
