use std::ops::{Add, Mul, Sub};
use num_traits::Float;
use crate::*;

pub (crate) trait FloatingPoint:
  Default + Float + SimdElement {}

pub (crate) trait FloatingPoints<T>:
  Default + Add<Output = Self> + Mul<Output = Self> +
  Sub<Output = Self> + SimdFloat<Scalar = T> {}

impl FloatingPoint for f32 {}
impl FloatingPoint for f64 {}
impl FloatingPoints<f32> for AutoSimd!(f32) {}
impl FloatingPoints<f64> for AutoSimd!(f64) {}

pub (crate) fn _cosine<T>(v0: &[T], v1: &[T]) -> T
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  let dot = _inner_product(v0, v1);
  if dot.is_zero() {
    dot
  } else {
    let norm_1 = _inner_product(v0, v0).sqrt();
    let norm_2 = _inner_product(v1, v1).sqrt();
    let cosine = dot / norm_1 / norm_2;
    let pos_one = T::one();
    let neg_one = T::one().neg();
    if cosine < neg_one {
      neg_one
    } else if cosine > pos_one {
      pos_one
    } else {
      cosine
    }
  }
}

pub (crate) fn _inner_product<T>(v0: &[T], v1: &[T]) -> T
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  #[cfg(feature = "simd")] {
    let iter_0 = v0.chunks_exact(auto_lane_count!(T));
    let iter_1 = v1.chunks_exact(auto_lane_count!(T));
    let zipped = iter_0.zip(iter_1);
    zipped
      .fold(<AutoSimd!(T)>::default(), |result, (e0, e1)| {
        let e0 = <AutoSimd!(T)>::from_slice(e0);
        let e1 = <AutoSimd!(T)>::from_slice(e1);
        let dot = e0 * e1;
        result + dot
      })
      .reduce_sum()
  }
  #[cfg(not(feature = "simd"))] {
    let iter_0 = v0.iter();
    let iter_1 = v1.iter();
    let zipped = iter_0.zip(iter_1);
    zipped
      .fold(T::default(), |result, (&e0, &e1)| {
        let dot = e0 * e1;
        result + dot
      })
  }
}

pub (crate) fn _l1<T>(v0: &[T], v1: &[T]) -> T
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  v0.iter()
    .zip(v1.iter())
    .fold(T::default(), |result, (&e0, &e1)| {
      let diff = e0 - e1;
      result + diff.abs()
    })
}

pub (crate) fn _l2<T>(v0: &[T], v1: &[T]) -> T
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  v0.iter()
    .zip(v1.iter())
    .fold(T::default(), |result, (&e0, &e1)| {
      let diff = e0 - e1;
      result + diff * diff
    })
}
