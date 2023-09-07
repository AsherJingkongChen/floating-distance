// #[cfg(feature = "simd")]
// use std::simd::{
//   prelude::*,
//   LaneCount,
//   SimdElement,
//   SupportedLaneCount,
// };

//\ declare \//

pub (crate) struct MetricCosine;
pub (crate) struct MetricInnerProduct;
pub (crate) struct MetricL2Norm;

pub trait Metric<T> {
  fn compute(&self, v0: &[T], v1: &[T]) -> f32
  where
    T: Copy + Into<f32>;
}

//\ implement \//

impl<T> Metric<T> for MetricCosine {
  fn compute(&self, v0: &[T], v1: &[T]) -> f32
  where
    T: Copy + Into<f32>,
  {
    _impl_metricCosineCompute(v0, v1)   
  }
}

impl<T> Metric<T> for MetricInnerProduct {
  fn compute(&self, v0: &[T], v1: &[T]) -> f32
  where
    T: Copy + Into<f32>,
  {
    _impl_metricInnerProductCompute(v0, v1)
  }
}

impl<T> Metric<T> for MetricL2Norm {
  fn compute(&self, v0: &[T], v1: &[T]) -> f32
  where
    T: Copy + Into<f32>,
  {
    _impl_metricL2NormCompute(v0, v1)
  }
}

fn _impl_metricCosineCompute<T>(
  v0: &[T],
  v1: &[T],
) -> f32
where
  T: Copy + Into<f32>,
{
  _impl_metricInnerProductCompute(v0, v1) /
  _impl_metricInnerProductCompute(v0, v0).sqrt() /
  _impl_metricInnerProductCompute(v1, v1).sqrt()
}

fn _impl_metricInnerProductCompute<T>(
  v0: &[T],
  v1: &[T],
) -> f32
where
  T: Copy + Into<f32>,
{
  v0.iter()
    .zip(v1.iter())
    .fold(0.0, |a, (&e0, &e1)| {
      let d: f32 = e0.into() * e1.into();
      a + d
    })
}

fn _impl_metricL2NormCompute<T>(
  v0: &[T],
  v1: &[T],
) -> f32
where
  T: Copy + Into<f32>,
{
  v0.iter()
    .zip(v1.iter())
    .fold(0.0, |a, (&e0, &e1)| {
      let d: f32 = e0.into() - e1.into();
      a + d * d
    })
}
