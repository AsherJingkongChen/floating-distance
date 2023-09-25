use cfg_if::cfg_if;

/// Get the correct simd lane count for the specified scalar type
/// 
/// # Examples
/// ```rust
/// use floating_distance::*;
/// use std::mem::size_of;
/// 
/// const LANES: usize = auto_lane_count!(f32);
/// assert!(LANES / size_of::<f32>() >= 1);
/// assert!(LANES % size_of::<f32>() == 0);
/// ```
/// 
/// # Details
/// It relies on `#[cfg(target_feature)]`.
/// The result is evaluated in compile time.
#[macro_export]
macro_rules! auto_lane_count {
  ($scalar: ty) => {{
    cfg_if::cfg_if! {
    if #[cfg(feature = "simd")] {
      cfg_if::cfg_if! {
      if #[cfg(any(
        target_feature = "avx512vl",
      ))] {
        512 / 8 / std::mem::size_of::<$scalar>()
      } else if #[cfg(any(
        target_feature = "avx",
        target_feature = "avx2",
      ))] {
        256 / 8 / std::mem::size_of::<$scalar>()
      } else if #[cfg(any(
        target_feature = "sse",
        target_feature = "sse2",
        target_feature = "sse3",
        target_feature = "sse4.1",
        target_feature = "sse4.2",
        target_feature = "sse4a",
        target_feature = "ssse3",
      ))] {
        128 / 8 / std::mem::size_of::<$scalar>()
      } else {
        compile_error!("\
  The target architecture does not support SIMD. \
  You can check out the documentation of \
  the floating-distance crate for more information.");
        std::mem::size_of::<$scalar>() /
        std::mem::size_of::<$scalar>()
      }
      }
    } else {
      1
    }
    }
  }}
}

cfg_if! {
if #[cfg(feature = "simd")] {
  pub (crate) use std::simd::{
    LaneCount,
    Simd,
    SimdElement,
    SimdFloat,
    SupportedLaneCount,
  };

  /// [`std::simd::LaneCount`]
  pub type AutoLaneCount<S> = LaneCount<{ auto_lane_count!(S) }>;
  /// [`std::simd::Simd`]
  pub type AutoSimd<S> = Simd<S, { auto_lane_count!(S) }>;
  /// [`std::simd::SimdElement`]
  pub trait AutoSimdElement: SimdElement {}
  /// [`std::simd::SimdFloat`]
  pub trait AutoSimdFloat: SimdFloat {}
  /// [`std::simd::SupportedLaneCount`]
  pub trait AutoSupportedLaneCount: SupportedLaneCount {}

  impl<S: SimdElement> AutoSimdElement for S {}
  impl<S: SimdFloat> AutoSimdFloat for S {}
  impl<S: SupportedLaneCount> AutoSupportedLaneCount for S {}

} else {
  /// Stub for [`std::simd::LaneCount`]
  pub type AutoLaneCount<S> = S;
  /// Stub for [`std::simd::Simd`]
  pub type AutoSimd<S> = S;
  /// Stub for [`std::simd::SimdElement`]
  pub trait AutoSimdElement {}
  /// Stub for [`std::simd::SimdFloat`]
  pub trait AutoSimdFloat {
    /// Stub for [`std::simd::SimdFloat::Scalar`]
    type Scalar;
  }
  /// Stub for [`std::simd::SupportedLaneCount`]
  pub trait AutoSupportedLaneCount {}

  impl<S> AutoSimdElement for S {}
  impl<P> AutoSimdFloat for P { type Scalar = P; }
  impl<S> AutoSupportedLaneCount for AutoLaneCount<S> {}
}
}
