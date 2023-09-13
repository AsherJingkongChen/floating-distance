use crate::*;

/// Distance can be measured between two vectors
pub trait Vector<T> {
  /// Measure distance between two vectors
  /// using the [`metric`](Metric) flags
  /// 
  /// # Examples
  /// ```
  /// use floating_distance::{Metric, Vector};
  /// use approx::*;
  /// 
  /// let v0: &[f32] = &[1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0];
  /// let v1: &[f32] = &[2.0, 1.0, 1.0, 0.0, 3.0, 1.0, 3.0];
  /// let metric = Metric::Cosine;
  /// let result = v0.distance(v1, metric);
  /// let expectation: f32 = 16.0 / (4.0 * 5.0);
  /// 
  /// assert_eq!(result, expectation);
  /// 
  /// let v0: [f64; 2] = [3.0, 10.7];
  /// let v1: [f64; 2] = [-4.0, 3.0];
  /// let metric = Metric::Euclidean;
  /// let result = v0.distance(&v1, metric);
  /// let expectation: f64 = 7.0 * 7.0 + 7.7 * 7.7;
  /// 
  /// assert_relative_eq!(result, expectation);
  /// ```
  /// 
  /// # Note
  /// The longer vector is truncated to align the length
  /// with the shorter one's
  fn distance(&self, other: &Self, metric: Metric) -> T;
}

impl<T> Vector<T> for [T]
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  fn distance(&self, other: &Self, metric: Metric) -> T {
    _distance(self, other, metric)
  }
}

impl<T> Vector<T> for Box<[T]>
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  fn distance(&self, other: &Self, metric: Metric) -> T {
    _distance(self, other, metric)
  }
}

impl<T> Vector<T> for Vec<T>
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  fn distance(&self, other: &Self, metric: Metric) -> T {
    _distance(self, other, metric)
  }
}

fn _distance<T>(v0: &[T], v1: &[T], metric: Metric) -> T
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  match metric {
    Metric::Cosine => cosine(v0, v1),
    Metric::InnerProduct => inner_product(v0, v1),
    Metric::Manhattan => manhattan(v0, v1),
    Metric::Euclidean => euclidean(v0, v1),
  }
}
