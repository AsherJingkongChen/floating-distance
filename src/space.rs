use crate::*;

/// Metric space
pub enum Space {
  /// Cosine space
  Cosine,

  /// Inner product space (Dot product)
  InnerProduct,

  /// L2 space (Euclidean distance)
  L2,
}

impl Space {
  /// Create the corresponding `Metric` by `Space`
  pub fn metric<T>(&self) -> Box<dyn Metric<T>>
  where
    T: Float,
  {
    match self {
      Space::Cosine => Box::from(MetricCosine),
      Space::InnerProduct => Box::from(MetricInnerProduct),
      Space::L2 => Box::from(MetricL2),
    }
  }
}
