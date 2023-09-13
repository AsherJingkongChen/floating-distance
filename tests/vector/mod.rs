use floating_distance::*;

#[test]
fn Vector_distance_using_slice() {
  let v0: &[f32] = &[1.0, 2.0, 3.0, 2.0, 1.0];
  let v1: &[f32] = &[2.0, 1.0, 1.0, 2.0, 3.0];
  let expectations = &[
    (2 + 2 + 3 + 4 + 3) as f32 /
    ((1 + 4 + 9 + 4 + 1) as f32).sqrt() /
    ((4 + 1 + 1 + 4 + 9) as f32).sqrt(),
    (1 + 1 + 4 + 0 + 4) as f32,
    (2 + 2 + 3 + 4 + 3) as f32,
    (1 + 1 + 2 + 0 + 2) as f32,
  ];
  {
    let results = &[
      v0.distance(v1, Metric::Cosine),
      v0.distance(v1, Metric::Euclidean),
      v0.distance(v1, Metric::InnerProduct),
      v0.distance(v1, Metric::Manhattan),
    ];
    assert_eq!(results, expectations);
  }
  {
    let v0 = Box::<[f32]>::from(v0);
    let v1 = Box::<[f32]>::from(v1);
    let results = &[
      v0.distance(&v1, Metric::Cosine),
      v0.distance(&v1, Metric::Euclidean),
      v0.distance(&v1, Metric::InnerProduct),
      v0.distance(&v1, Metric::Manhattan),
    ];
    assert_eq!(results, expectations);
  }
}

#[test]
fn Vector_distance_using_vec() {
  let v0: Vec<f64> = vec![-2.0, 1.0, 3.0, 2.0, 3.0];
  let v1: Vec<f64> = vec![1.0, 4.0, -1.0, 2.0, 4.0];
  let expectations = &[
    (-2 + 4 + -3 + 4 + 12) as f64 /
    ((4 + 1 + 9 + 4 + 9) as f64).sqrt() /
    ((1 + 16 + 1 + 4 + 16) as f64).sqrt(),
    (9 + 9 + 16 + 0 + 1) as f64,
    (-2 + 4 + -3 + 4 + 12) as f64,
    (3 + 3 + 4 + 0 + 1) as f64,
  ];
  let results = &[
    v0.distance(&v1, Metric::Cosine),
    v0.distance(&v1, Metric::Euclidean),
    v0.distance(&v1, Metric::InnerProduct),
    v0.distance(&v1, Metric::Manhattan),
  ];
  assert_eq!(results, expectations);
}
