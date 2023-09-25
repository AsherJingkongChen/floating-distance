/// Distance function specifications
pub enum Metric {
  /// Cosine similarity
  /// 
  /// `Dot(U, V) = Sum(U[i] * V[i])`
  /// 
  /// `Distance => Cos(U, V) = Dot(U, V) / (Dot(U, U) * Dot(V, V))`
  Cosine,

  /// Euclidean distance, or L2 distance
  /// 
  /// `Distance => L2(U, V) = Sum((U[i] - V[i]) ^ 2)`
  Euclidean,

  /// Inner product, or dot product
  /// 
  /// `Distance => Dot(U, V) = Sum(U[i] * V[i])`
  InnerProduct,

  /// Manhattan distance, or L1 distance
  /// 
  /// `Distance => L1(U, V) = Sum(Abs(U[i] - V[i]))`
  Manhattan,
}
