use floating_distance::*;

#[test]
fn all_handle_zeros_and_ones() {
  let zeros: &[f32] = &[0.0; 144];
  let ones: &[f32] = &[1.0; 144];
  let expectations: &[f32] = &[
    0.0, 0.0, 0.0, 0.0,
    0.0, 144.0, 0.0, 144.0,
    1.0,  0.0, 144.0, 0.0,
  ];
  let results = &[
    zeros.distance(zeros, Metric::Cosine),
    zeros.distance(zeros, Metric::Euclidean),
    zeros.distance(zeros, Metric::InnerProduct),
    zeros.distance(zeros, Metric::Manhattan),
    zeros.distance(ones, Metric::Cosine),
    zeros.distance(ones, Metric::Euclidean),
    zeros.distance(ones, Metric::InnerProduct),
    zeros.distance(ones, Metric::Manhattan),
    ones.distance(ones, Metric::Cosine),
    ones.distance(ones, Metric::Euclidean),
    ones.distance(ones, Metric::InnerProduct),
    ones.distance(ones, Metric::Manhattan),
  ];
  assert_eq!(results, expectations);
}

#[test]
fn all_handle_empty() {
  let empty: (&[f32], &[f32]) = (&[], &[]);
  let expectations: &[f32] = &[
    0.0, 0.0, 0.0, 0.0,
  ];
  let results = &[
    empty.0.distance(empty.1, Metric::Cosine),
    empty.0.distance(empty.1, Metric::Euclidean),
    empty.0.distance(empty.1, Metric::InnerProduct),
    empty.0.distance(empty.1, Metric::Manhattan),
  ];
  assert_eq!(results, expectations);
}

#[test]
fn cosine_clamp() {
  let v0: &[f32] = &[0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
  let v1: &[f32] = &[0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
  let result = v0.distance(v1, Metric::Cosine);
  let expectation: f32 = 1.0;
  assert_eq!(result, expectation);

  let v0: &[f32] = &[0.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 0.0, -1.0, -1.0, 0.0, -1.0, -1.0, 0.0, -1.0];
  let v1: &[f32] = &[0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
  let result = v0.distance(v1, Metric::Cosine);
  let expectation: f32 = -1.0;

  assert_eq!(result, expectation);
}
