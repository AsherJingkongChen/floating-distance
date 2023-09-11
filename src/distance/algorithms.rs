use num_traits::Float;

pub (crate) fn _cosine<T>(v0: &[T], v1: &[T]) -> T
where
  T: Float,
{
  let dot = _inner_product(v0, v1);
  if dot.is_zero() {
    dot
  } else {
    let norm_1 = _inner_product(v0, v0).sqrt();
    let norm_2 = _inner_product(v1, v1).sqrt();
    let cosine = dot / norm_1 / norm_2;
    let pos_one = T::one();
    let neg_one = T::one().neg();
    if cosine < neg_one {
      neg_one
    } else if cosine > pos_one {
      pos_one
    } else {
      cosine
    }
  }
}

pub (crate) fn _inner_product<T>(v0: &[T], v1: &[T]) -> T
where
  T: Float,
{
  v0.iter()
    .zip(v1.iter())
    .fold(T::zero(), |result, (&e0, &e1)| {
      let dot = e0 * e1;
      result + dot
    })
}

pub (crate) fn _l1<T>(v0: &[T], v1: &[T]) -> T
where
  T: Float,
{
  v0.iter()
    .zip(v1.iter())
    .fold(T::zero(), |result, (&e0, &e1)| {
      let diff = e0 - e1;
      result + diff.abs()
    })
}

pub (crate) fn _l2<T>(v0: &[T], v1: &[T]) -> T
where
  T: Float,
{
  v0.iter()
    .zip(v1.iter())
    .fold(T::zero(), |result, (&e0, &e1)| {
      let diff = e0 - e1;
      result + diff * diff
    })
}
