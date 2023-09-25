use cfg_if::cfg_if;
use std::iter::zip;
use crate::*;

impl Metric {
  /// Measure distance between two vectors
  /// 
  /// # Examples
  /// ```rust
  /// use floating_distance::*;
  /// 
  /// let v0: &[f32; 4] = &[1.0, -2.0, -3.5, 99.0];
  /// let v1: &[f32; 3] = &[-3.0, -5.0, -5.3];
  /// let metric = Metric::Manhattan;
  /// let result = metric.measure::<f32>(v0, v1);
  /// let expectation = 4.0 + 3.0 + 1.8;
  /// 
  /// assert_eq!(result, expectation);
  /// 
  /// let v0: [f64; 2] = [3.0, 10.0];
  /// let v1: [f64; 2] = [-4.0, 3.0];
  /// let result = Metric::Euclidean.measure::<f64>(&v0, &v1);
  /// let expectation = 7.0 * 7.0 + 7.0 * 7.0;
  /// 
  /// assert_eq!(result, expectation);
  /// ```
  /// 
  /// # Details
  /// The longer vector is truncated
  /// to align with the shorter one.
  pub fn measure<S>(&self, v0: Vector<S>, v1: Vector<S>) -> S
  where
    AutoLaneCount<S>: AutoSupportedLaneCount,
    AutoSimd<S>: FloatingPoints<S>,
    S: FloatingPoint,
    [(); auto_lane_count!(S)]:,
  {
    match self {
      Metric::Cosine => measure_cosine(v0, v1),
      Metric::InnerProduct => measure_inner_product(v0, v1),
      Metric::Manhattan => measure_manhattan(v0, v1),
      Metric::Euclidean => measure_euclidean(v0, v1),
    }
  }
}

macro_rules! measure_iterative_impl {
  ($name: ident, $map_fn: expr,) => {
  fn $name<S>(v0: Vector<S>, v1: Vector<S>) -> S
    where
      AutoLaneCount<S>: AutoSupportedLaneCount,
      AutoSimd<S>: FloatingPoints<S>,
      S: FloatingPoint,
      [(); auto_lane_count!(S)]:,
    {
      let scalar_map_fn: fn(S, (S, S)) -> S = $map_fn;

      cfg_if! {
      if #[cfg(feature = "simd")] {
        type P<S> = AutoSimd<S>;
        let packed_map_fn: fn(P<S>, (P<S>, P<S>)) -> P<S> = $map_fn;

        if let Some(last_chunk) =
          zip(
            v0.chunks(auto_lane_count!(S)),
            v1.chunks(auto_lane_count!(S)),
          ).rev().next()
        {
          let last_chunk_not_exact =
            last_chunk.0.len() != last_chunk.1.len() ||
            last_chunk.0.len() != auto_lane_count!(S);
          let sum_of_not_exact_chunks =
            if last_chunk_not_exact {
              let items = zip(
                last_chunk.0.iter(),
                last_chunk.1.iter(),
              );
              items.fold(S::default(), |result, item| {
                scalar_map_fn(result, (*item.0, *item.1))
              })
            } else {
              S::default()
            };

          let exact_chunks = zip(
            v0.chunks_exact(auto_lane_count!(S)),
            v1.chunks_exact(auto_lane_count!(S)),
          );
          let sum_of_exact_chunks =
            exact_chunks
              .fold(P::<S>::default(), |result, chunk| {
                packed_map_fn(result, (
                  P::<S>::from_slice(chunk.0),
                  P::<S>::from_slice(chunk.1),
                ))
              })
              .reduce_sum();

          sum_of_not_exact_chunks +
          sum_of_exact_chunks

        } else {
          S::default()
        }

      } else {
        let items = zip(v0.iter(), v1.iter());
        items.fold(S::default(), |result, item| {
          scalar_map_fn(result, (*item.0, *item.1))
        })
      }
      }
    }
  };
}

fn measure_cosine<S>(v0: Vector<S>, v1: Vector<S>) -> S
where
  AutoLaneCount<S>: AutoSupportedLaneCount,
  AutoSimd<S>: FloatingPoints<S>,
  S: FloatingPoint,
  [(); auto_lane_count!(S)]:,
{
  let dot = measure_inner_product(v0, v1);
  if dot.is_zero() {
    dot
  } else {
    let length = v0.len().min(v1.len());
    let v0 = &v0[0..length];
    let v1 = &v1[0..length];
    let norm_1 = measure_inner_product(v0, v0).sqrt();
    let norm_2 = measure_inner_product(v1, v1).sqrt();
    let cosine = dot / norm_1 / norm_2;
    let pos_one = S::one();
    let neg_one = S::one().neg();
    if cosine < neg_one {
      neg_one
    } else if cosine > pos_one {
      pos_one
    } else {
      cosine
    }
  }
}

measure_iterative_impl!(
  measure_euclidean,
  |result, item| {
    let diff = item.0 - item.1;
    result + diff * diff
  },
);

measure_iterative_impl!(
  measure_inner_product,
  |result, item| {
    let dot = item.0 * item.1;
    result + dot
  },
);

measure_iterative_impl!(
  measure_manhattan,
  |result, item| {
    let diff_abs = (item.0 - item.1).abs();
    result + diff_abs
  },
);
