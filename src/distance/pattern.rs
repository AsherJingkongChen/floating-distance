use crate::*;

/// A trait that provides functions
/// to compute `distance` between two vectors
pub trait DistancePattern<T> {
  /// Compute distance between two vectors
  /// using the `metric`
  /// 
  /// # Examples
  /// ```
  /// use floating_distance::{DistancePattern, Metric};
  /// 
  /// let v0: &[f32] = &[1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0];
  /// let v1: &[f32] = &[2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 2.0];
  /// let metric = Metric::Cosine;
  /// let result = v0.distance(v1, metric);
  /// let expectation: f32 = 14.0 / (4.0 * 4.0);
  /// 
  /// assert_eq!(result, expectation);
  /// 
  /// let v0: [f64; 2] = [3.0, 10.0];
  /// let v1: [f64; 2] = [-4.0, 3.0];
  /// let metric = Metric::L2;
  /// let result = v0.distance(&v1, metric);
  /// let expectation: f64 = 7.0 * 7.0 + 7.0 * 7.0;
  /// 
  /// assert_eq!(result, expectation);
  /// ```
  fn distance(&self, other: &Self, metric: Metric) -> T;

  /// Compute distance between two vectors
  /// using `Metric::Cosine`
  /// 
  /// Alias to `.distance(other, Metric::Cosine)`
  fn distance_cosine(&self, other: &Self) -> T;

  /// Compute distance between two vectors
  /// using `Metric::InnerProduct`
  /// 
  /// Alias to `.distance(other, Metric::InnerProduct)`
  fn distance_inner_product(&self, other: &Self) -> T;

  /// Compute distance between two vectors
  /// using `Metric::L1`
  /// 
  /// Alias to `.distance(other, Metric::L1)`
  fn distance_l1(&self, other: &Self) -> T;

  /// Compute distance between two vectors
  /// using `Metric::L2`
  /// 
  /// Alias to `.distance(other, Metric::L2)`
  fn distance_l2(&self, other: &Self) -> T;
}

impl<T> DistancePattern<T> for [T]
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  fn distance(&self, other: &Self, metric: Metric) -> T {
    _distance(self, other, metric)
  }
  fn distance_cosine(&self, other: &Self) -> T {
    _cosine(self, other)
  }
  fn distance_inner_product(&self, other: &Self) -> T {
    _inner_product(self, other)
  }
  fn distance_l1(&self, other: &Self) -> T {
    _l1(self, other)
  }
  fn distance_l2(&self, other: &Self) -> T {
    _l2(self, other)
  }
}

impl<T> DistancePattern<T> for Box<[T]>
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  fn distance(&self, other: &Self, metric: Metric) -> T {
    _distance(self, other, metric)
  }
  fn distance_cosine(&self, other: &Self) -> T {
    _cosine(self, other)
  }
  fn distance_inner_product(&self, other: &Self) -> T {
    _inner_product(self, other)
  }
  fn distance_l1(&self, other: &Self) -> T {
    _l1(self, other)
  }
  fn distance_l2(&self, other: &Self) -> T {
    _l2(self, other)
  }
}

impl<T> DistancePattern<T> for Vec<T>
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  fn distance(&self, other: &Self, metric: Metric) -> T {
    _distance(self, other, metric)
  }
  fn distance_cosine(&self, other: &Self) -> T {
    _cosine(self, other)
  }
  fn distance_inner_product(&self, other: &Self) -> T {
    _inner_product(self, other)
  }
  fn distance_l1(&self, other: &Self) -> T {
    _l1(self, other)
  }
  fn distance_l2(&self, other: &Self) -> T {
    _l2(self, other)
  }
}

fn _distance<T>(v0: &[T], v1: &[T], metric: Metric) -> T
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  match metric {
    Metric::Cosine => _cosine(v0, v1),
    Metric::InnerProduct => _inner_product(v0, v1),
    Metric::L1 => _l1(v0, v1),
    Metric::L2 => _l2(v0, v1),
  }
}
