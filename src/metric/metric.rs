use crate::*;

/// A trait that provides a function
/// to `compute` distance between two buffers
pub trait Metric<T>
where
  T: Float
{
  /// Compute distance between two buffers
  /// using the `Metric`
  fn compute(&self, v0: &[T], v1: &[T]) -> BigFloat;
}

pub (crate) struct MetricCosine;
pub (crate) struct MetricInnerProduct;
pub (crate) struct MetricL2;

impl<T> Metric<T> for MetricCosine
where
  T: Float,
{
  fn compute(&self, v0: &[T], v1: &[T]) -> BigFloat {
    _MetricCosine_compute(v0, v1)   
  }
}

impl<T> Metric<T> for MetricInnerProduct
where
  T: Float,
{
  fn compute(&self, v0: &[T], v1: &[T]) -> BigFloat {
    _MetricInnerProduct_compute(v0, v1)   
  }
}

impl<T> Metric<T> for MetricL2
where
  T: Float,
{
  fn compute(&self, v0: &[T], v1: &[T]) -> BigFloat {
    _MetricL2_compute(v0, v1)   
  }
}

#[inline]
pub (crate) fn _MetricCosine_compute<T>(
  v0: &[T],
  v1: &[T],
) -> BigFloat
where
  T: Float,
{
  let dot_01 = _MetricInnerProduct_compute(v0, v1);
  let norm_1 = _MetricInnerProduct_compute(v0, v0).sqrt();
  let norm_2 = _MetricInnerProduct_compute(v1, v1).sqrt();
  return
    if dot_01 == 0.0 { 0.0 }
    else { (dot_01 / norm_1 / norm_2).clamp(-1.0, 1.0) }
}

#[inline]
pub (crate) fn _MetricInnerProduct_compute<T>(
  v0: &[T],
  v1: &[T],
) -> BigFloat
where
  T: Float,
{
  v0.iter()
    .zip(v1.iter())
    .fold(0.0, |result, (&e0, &e1)| {
      let dot = e0.into() * e1.into();
      result + dot
    })
}

#[inline]
pub (crate) fn _MetricL2_compute<T>(
  v0: &[T],
  v1: &[T],
) -> BigFloat
where
  T: Float,
{
  v0.iter()
    .zip(v1.iter())
    .fold(0.0, |result, (&e0, &e1)| {
      let diff = e0.into() - e1.into();
      result + diff * diff
    })
}
