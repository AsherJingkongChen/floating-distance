pub use speedy::{Readable, Writable};
use crate::*;

/// Specify the metric space for distance computations
#[derive(
  Debug, Eq, PartialEq,
  Readable, Writable,
)]
pub enum Space {
  Cosine,
  InnerProduct,
  L2Norm,
}

impl Space {
  /// Create the corresponding `Metric` by `Space`
  pub fn metric<T>(&self) -> Box<dyn Metric<T>>
  where
    T: Copy + Into<f32>,
  {
    match self {
      Space::Cosine => Box::from(MetricCosine),
      Space::InnerProduct => Box::from(MetricInnerProduct),
      Space::L2Norm => Box::from(MetricL2Norm),
    }
  }
}
