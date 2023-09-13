use cfg_if::cfg_if;
extern crate std;

#[cfg(feature = "simd")]
cfg_if! {
if #[cfg(any(
  target_feature = "avx512vl",
))] {
  macro_rules! auto_lane_count {
    ($T: ty) => { 512 / 8 / size_of::<$T>() }
  }
} else if #[cfg(any(
  target_feature = "avx",
  target_feature = "avx2",
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
  compile_error!("\
The target architecture does not support SIMD. \
You can check out the documentation of \
the floating-distance crate for more information.");
  macro_rules! auto_lane_count {
    ($T: ty) => { size_of::<$T>() / size_of::<$T>() }
  }
}}

#[cfg(feature = "simd")]
pub (crate) use auto_lane_count;

cfg_if! {
if #[cfg(feature = "simd")] {
  macro_rules! AutoLaneCount {
    ($T: ty) => { LaneCount<{ auto_lane_count!($T) }> }
  }
  macro_rules! AutoSimd {
    ($T: ty) => { Simd<$T, { auto_lane_count!($T) }> }
  }

  pub (crate) use std::{
    mem::size_of,
    simd::{
      LaneCount,
      Simd,
      SupportedLaneCount,
    }
  };

  #[doc(hidden)]
  pub use std::simd::{
    SimdElement as AutoSimdElement,
    SimdFloat as AutoSimdFloat
  };
} else {
  macro_rules! AutoLaneCount {
    ($T: ty) => { PhantomData<$T> }
  }
  macro_rules! AutoSimd {
    ($T: ty) => { $T }
  }

  pub (crate) use std::marker::PhantomData;

  /// Stub of [`std::simd::SimdElement`]
  pub trait AutoSimdElement {}
  /// Stub of [`std::simd::SimdFloat`]
  pub trait AutoSimdFloat {
    /// Stub of [`std::simd::SimdFloat::Scalar`]
    type Scalar;
  }
  pub (crate) trait SupportedLaneCount {}

  impl<S> AutoSimdElement for S {}
  impl<S> AutoSimdFloat for S { type Scalar = S; }
  impl<S> SupportedLaneCount for S {} 
}}

pub (crate) use AutoLaneCount;
pub (crate) use AutoSimd;
