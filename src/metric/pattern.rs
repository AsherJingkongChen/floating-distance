use crate::*;

/// A trait that provides functions
/// to compute `distance` between two buffers
pub trait MetricPattern {
  /// Compute distance between two buffers
  /// using the `space`
  fn distance(&self, rhs: &Self, space: Space) -> BigFloat;

  /// Compute distance between two buffers
  /// using `Space::Cosine`
  fn distance_cosine(&self, rhs: &Self) -> BigFloat;

  /// Compute distance between two buffers
  /// using `Space::InnerProduct`
  fn distance_inner_product(&self, rhs: &Self) -> BigFloat;

  /// Compute distance between two buffers
  /// using `Space::L2`
  fn distance_l2(&self, rhs: &Self) -> BigFloat;
}

impl<T> MetricPattern for [T]
where
  T: Float,
{
  #[inline]
  fn distance(&self, rhs: &Self, space: Space) -> BigFloat {
    match space {
      Space::Cosine => _MetricCosine_compute(self, rhs),
      Space::InnerProduct => _MetricInnerProduct_compute(self, rhs),
      Space::L2 => _MetricL2_compute(self, rhs),
    }
  }
  fn distance_cosine(&self, rhs: &Self) -> BigFloat {
    _MetricCosine_compute(self, rhs)
  }
  fn distance_inner_product(&self, rhs: &Self) -> BigFloat {
    _MetricInnerProduct_compute(self, rhs)
  }
  fn distance_l2(&self, rhs: &Self) -> BigFloat {
    _MetricL2_compute(self, rhs)
  }
}

impl<T> MetricPattern for Box<[T]>
where
  T: Float,
{
  fn distance(&self, rhs: &Self, space: Space) -> BigFloat {
    self.as_ref().distance(rhs, space)
  }
  fn distance_cosine(&self, rhs: &Self) -> BigFloat {
    _MetricCosine_compute(self, rhs)
  }
  fn distance_inner_product(&self, rhs: &Self) -> BigFloat {
    _MetricInnerProduct_compute(self, rhs)
  }
  fn distance_l2(&self, rhs: &Self) -> BigFloat {
    _MetricL2_compute(self, rhs)
  }
}

impl<T> MetricPattern for Vec<T>
where
  T: Float,
{
  fn distance(&self, rhs: &Self, space: Space) -> BigFloat {
    self.as_slice().distance(rhs, space)
  }
  fn distance_cosine(&self, rhs: &Self) -> BigFloat {
    _MetricCosine_compute(self, rhs)
  }
  fn distance_inner_product(&self, rhs: &Self) -> BigFloat {
    _MetricInnerProduct_compute(self, rhs)
  }
  fn distance_l2(&self, rhs: &Self) -> BigFloat {
    _MetricL2_compute(self, rhs)
  }
}
