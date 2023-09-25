#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

#![cfg_attr(feature = "simd",
  allow(incomplete_features),
  feature(
    generic_const_exprs,
    portable_simd,
  ),
)]
#![cfg_attr(docsrs,
  feature(doc_cfg)
)]

/// Provide distance functions and compare functions
pub mod metric;
pub use metric::*;

/// Floating-point numeric types
pub mod numeric;
pub use numeric::*;

/// Utilities for `simd` feature
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
pub mod simd;
pub use simd::*;

/// Vector type
pub mod vector;
pub use vector::*;
