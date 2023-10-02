use cfg_if::cfg_if;

/// Get the correct simd lane count for the specified scalar type
/// 
/// # Examples
/// ```rust
/// use floating_distance::*;
/// use std::mem::size_of;
/// 
/// const LANES: usize = auto_lane_count!(f32);
/// assert_eq!(LANES, 16);
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
      512 / 8 / std::mem::size_of::<$scalar>()
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
