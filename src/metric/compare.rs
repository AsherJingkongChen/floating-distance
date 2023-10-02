#[doc(hidden)]
pub use std::cmp::Ordering;
use crate::*;

impl Metric {
  /// Compare two distance. Less is closer.
  /// 
  /// # Examples
  /// ```rust
  /// use floating_distance::*;
  /// 
  /// let v0: &[f32] = &[1.0, -2.0];
  /// let v1: &[f32] = &[-3.0, 5.0];
  /// let v2: &[f32] = &[-2.5, 3.0];
  /// let metric = Metric::Manhattan;
  /// let d01 = metric.measure::<f32>(v0, v1);
  /// let d02 = metric.measure::<f32>(v0, v2);
  /// assert_eq!(d01, 11.0);
  /// assert_eq!(d02, 8.5);
  /// 
  /// let result = metric.compare(&d01, &d02);
  /// assert_eq!(result, Some(Ordering::Greater));
  /// 
  /// let result = metric.compare(&d02, &d01);
  /// assert_eq!(result, Some(Ordering::Less));
  /// 
  /// let result = metric.compare(&d01, &d01);
  /// assert_eq!(result, Some(Ordering::Equal));
  /// ```
  pub fn compare<S>(&self, d0: &S, d1: &S) -> Option<Ordering>
  where
    S: FloatingPoint,
  {
    match self {
      Metric::Cosine => (-*d0).partial_cmp(&-*d1),
      Metric::Euclidean => d0.partial_cmp(d1),
      Metric::InnerProduct => (-*d0).partial_cmp(&-*d1),
      Metric::Manhattan => d0.partial_cmp(d1),
    }
  }
}
