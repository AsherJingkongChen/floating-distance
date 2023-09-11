#![doc = include_str!("../README.md")]

#![allow(non_snake_case)]
#![warn(missing_docs)]
#![cfg_attr(feature = "simd", feature(portable_simd))]
#![feature(generic_const_exprs)]

/// Distance computations
pub mod distance;

/// Specify the algorithms of distance computations
pub mod metric;

pub use crate::distance::*;
pub use crate::metric::*;
