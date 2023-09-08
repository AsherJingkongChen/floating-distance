#![doc = include_str!("../README.md")]

#![allow(non_snake_case)]
#![warn(missing_docs)]

/// Floating-point numeric types
pub mod float;

/// Functions to compute distance
pub mod metric;

/// Specify the metric space of distance computations
pub mod space;

pub use crate::float::*;
pub use crate::metric::*;
pub use crate::space::*;
