use crate::*;

pub (crate) fn _cosine<T>(v0: &[T], v1: &[T]) -> BigFloat
where
  T: Float,
{
  let dot_01 = _inner_product(v0, v1);
  return
    if dot_01 == 0.0 { 0.0 }
    else {
      let norm_1 = _inner_product(v0, v0).sqrt();
      let norm_2 = _inner_product(v1, v1).sqrt();
      return (dot_01 / norm_1 / norm_2).clamp(-1.0, 1.0)
    }
}

pub (crate) fn _inner_product<T>(v0: &[T], v1: &[T]) -> BigFloat
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

pub (crate) fn _l2<T>(v0: &[T], v1: &[T]) -> BigFloat
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
