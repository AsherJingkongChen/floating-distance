//\ declare \//

pub (crate) struct MetricCosine;
pub (crate) struct MetricInnerProduct;
pub (crate) struct MetricL2Norm;

/// A trait that provides a function
/// to `compute` the distance between two buffers
pub trait Metric<T> {
  /// Compute the distance between two buffers
  /// using the `Metric`
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
    _MetricCosine_compute(v0, v1)   
  }
}

impl<T> Metric<T> for MetricInnerProduct {
  fn compute(&self, v0: &[T], v1: &[T]) -> f32
  where
    T: Copy + Into<f32>,
  {
    _MetricInnerProduct_compute(v0, v1)
  }
}

impl<T> Metric<T> for MetricL2Norm {
  fn compute(&self, v0: &[T], v1: &[T]) -> f32
  where
    T: Copy + Into<f32>,
  {
    _MetricL2Norm_compute(v0, v1)
  }
}

#[inline]
pub (crate) fn _MetricCosine_compute<T>(
  v0: &[T],
  v1: &[T],
) -> f32
where
  T: Copy + Into<f32>,
{
  _MetricInnerProduct_compute(v0, v1) /
  _MetricInnerProduct_compute(v0, v0).sqrt() /
  _MetricInnerProduct_compute(v1, v1).sqrt()
}

#[inline]
pub (crate) fn _MetricInnerProduct_compute<T>(
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

#[inline]
pub (crate) fn _MetricL2Norm_compute<T>(
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
