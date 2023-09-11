/// Metric specifications
pub enum Metric {
  /// Cosine (Normalized similarity)
  Cosine,

  /// Inner product (Dot product)
  InnerProduct,

  /// L1 (Manhattan distance)
  L1,

  /// L2 (Euclidean distance)
  L2,
}
