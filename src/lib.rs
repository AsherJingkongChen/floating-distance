#![allow(non_snake_case)]
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

#![cfg_attr(feature = "simd",
  feature(
    generic_const_exprs,
    iter_array_chunks,
    portable_simd,
  ),
  allow(incomplete_features)
)]
#![cfg_attr(docsrs,
  feature(doc_cfg)
)]

/// It contains the definitions and implementations
/// of distance algorithms
pub mod metric;
pub use metric::*;

/// The floating-point numeric types
pub mod numeric;
pub use numeric::*;

/// Requires to specify `features = ["simd"]` in `Cargo.toml`
/// and compile with Rust nightly version
mod simd;
#[cfg_attr(docsrs, doc(cfg(feature = "simd")))]
pub use simd::*;

/// The vector types
pub mod vector;
pub use vector::*;

