#[cfg(not(feature = "simd"))]
pub (crate) use no_simd::*;
#[cfg(feature = "simd")]
pub (crate) use simd::*;
// sse sse2 sse3 sse4.1 sse4.2 sse4a ssse3
// avx avx2
// avx512vl
#[cfg(not(feature = "simd"))]
mod no_simd {
  macro_rules! AutoLaneCount {
    ($T: ty) => { $T }
  }
  macro_rules! AutoSimd {
    ($T: ty) => { $T }
  }

  pub (crate) use AutoLaneCount;
  pub (crate) use AutoSimd;
  pub (crate) trait SimdElement {}
  pub (crate) trait SimdFloat { type Scalar; }
  pub (crate) trait SupportedLaneCount {}
  
  impl<S> SimdElement for S {}
  impl<S> SimdFloat for S { type Scalar = S; }
  impl<S> SupportedLaneCount for S {}
}

#[cfg(feature = "simd")]
mod simd {
  cfg_if! {
    if #[cfg(any(
      target_feature = "avx512vl",
    ))] {
      macro_rules! auto_lane_count {
        ($T: ty) => { 512 / 8 / size_of::<$T>() }
      }
    } else if #[cfg(any(
      target_feature = "avx",
      target_feature = "avx2"
    ))] {
      macro_rules! auto_lane_count {
        ($T: ty) => { 256 / 8 / size_of::<$T>() }
      }
    } else if #[cfg(any(
      target_feature = "sse",
      target_feature = "sse2",
      target_feature = "sse3",
      target_feature = "sse4.1",
      target_feature = "sse4.2",
      target_feature = "sse4a",
      target_feature = "ssse3",
    ))] {
      macro_rules! auto_lane_count {
        ($T: ty) => { 128 / 8 / size_of::<$T>() }
      }
    } else {
      macro_rules! auto_lane_count {
        ($T: ty) => { 128 / 8 / size_of::<$T>() }
      }
    }
  }
  macro_rules! AutoLaneCount {
    ($T: ty) => { LaneCount<{ auto_lane_count!($T) }> }
  }
  macro_rules! AutoSimd {
    ($T: ty) => { Simd<$T, { auto_lane_count!($T) }> }
  }

  pub (crate) use auto_lane_count;
  pub (crate) use AutoLaneCount;
  pub (crate) use AutoSimd;
  pub (crate) use std::{
    mem::size_of,
    simd::{
      LaneCount,
      Simd,
      SimdElement,
      SimdFloat,
      SupportedLaneCount,
    }
  };
  use cfg_if::cfg_if;
}
