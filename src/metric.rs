use cfg_if::cfg_if;
use std::iter::zip;
use crate::*;

/// Metric specifications
pub enum Metric {
  /// Cosine similarity
  Cosine,

  /// Euclidean distance, or L2 distance
  Euclidean,

  /// Inner product, or dot product
  InnerProduct,

  /// Manhattan distance, or L1 distance
  Manhattan,
}

macro_rules! metric_iterative_impl {
  ($metric: ident, $map_fn: expr) => {
    pub (crate) fn $metric<T>(v0: &[T], v1: &[T]) -> T
    where
      T: FloatingPoint,
      AutoSimd!(T): FloatingPoints<T>,
      AutoLaneCount!(T): SupportedLaneCount,
    {
      cfg_if! {
      if #[cfg(feature = "simd")] {
        type P<T> = AutoSimd!(T);
        let scalar_map_fn: fn(T, (T, T)) -> T = $map_fn;
        let packed_map_fn: fn(P<T>, (P<T>, P<T>)) -> P<T> = $map_fn;
        if let Some(last_chunk) =
          zip(
            v0.chunks(auto_lane_count!(T)),
            v1.chunks(auto_lane_count!(T)),
          ).rev().next()
        {
          let last_chunk_not_exact =
            last_chunk.0.len() != last_chunk.1.len() ||
            last_chunk.0.len() != auto_lane_count!(T);
          let sum_of_not_exact_chunks =
            if last_chunk_not_exact {
              let items = zip(
                last_chunk.0.iter(),
                last_chunk.1.iter(),
              );
              items.fold(T::default(), |result, item| {
                scalar_map_fn(result, (*item.0, *item.1))
              })
            } else {
              T::default()
            };

          let exact_chunks = zip(
            v0.chunks_exact(auto_lane_count!(T)),
            v1.chunks_exact(auto_lane_count!(T)),
          );
          let sum_of_exact_chunks =
            exact_chunks
              .fold(P::<T>::default(), |result, chunk| {
                packed_map_fn(result, (
                  P::<T>::from_slice(chunk.0),
                  P::<T>::from_slice(chunk.1),
                ))
              })
              .reduce_sum();

          sum_of_not_exact_chunks +
          sum_of_exact_chunks
        } else {
          T::default()
        }
      } else {
        let items = zip(v0.iter(), v1.iter());
        items.fold(T::default(), |result, item| {
          scalar_map_fn(result, (*item.0, *item.1))
        })
      }}
    }
  };
}

pub (crate) fn cosine<T>(v0: &[T], v1: &[T]) -> T
where
  T: FloatingPoint,
  AutoSimd!(T): FloatingPoints<T>,
  AutoLaneCount!(T): SupportedLaneCount,
{
  let dot = inner_product(v0, v1);
  if dot.is_zero() {
    dot
  } else {
    let length = v0.len().min(v1.len());
    let v0 = &v0[0..length];
    let v1 = &v1[0..length];
    let norm_1 = inner_product(v0, v0).sqrt();
    let norm_2 = inner_product(v1, v1).sqrt();
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

metric_iterative_impl!(euclidean, |result, item| {
  let diff = item.0 - item.1;
  result + diff * diff
});

metric_iterative_impl!(inner_product, |result, item| {
  let dot = item.0 * item.1;
  result + dot
});

metric_iterative_impl!(manhattan, |result, item| {
  let diff_abs = (item.0 - item.1).abs();
  result + diff_abs
});
