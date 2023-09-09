use floating_distance::*;

#[test]
fn DistancePattern_distance_using_slice() {
  let v0: &[f32] = &[1.0, 2.0, 3.0, 2.0, 1.0];
  let v1: &[f32] = &[2.0, 1.0, 1.0, 2.0, 3.0];
  let expectations = &[
    (2 + 2 + 3 + 4 + 3) as f64 /
    ((1 + 4 + 9 + 4 + 1) as f64).sqrt() /
    ((4 + 1 + 1 + 4 + 9) as f64).sqrt(),
    (2 + 2 + 3 + 4 + 3) as f64,
    (1 + 1 + 4 + 0 + 4) as f64,
  ];
  {
    let results = &[
      v0.distance(v1, Metric::Cosine),
      v0.distance(v1, Metric::InnerProduct),
      v0.distance(v1, Metric::L2),
    ];
    let result_aliases = &[
      v0.distance_cosine(v1),
      v0.distance_inner_product(v1),
      v0.distance_l2(v1),
    ];
    assert_eq!(results, expectations);
    assert_eq!(results, result_aliases);
  }
  {
    let v0 = Box::<[f32]>::from(v0);
    let v1 = Box::<[f32]>::from(v1);
    let results = &[
      v0.distance(&v1, Metric::Cosine),
      v0.distance(&v1, Metric::InnerProduct),
      v0.distance(&v1, Metric::L2),
    ];
    let result_aliases = &[
      v0.distance_cosine(&v1),
      v0.distance_inner_product(&v1),
      v0.distance_l2(&v1),
    ];
    assert_eq!(results, expectations);
    assert_eq!(results, result_aliases);
  }
}

#[test]
fn DistancePattern_distance_using_vec() {
  let v0: Vec<f32> = vec![2.0, 1.0, 3.0, 2.0, 3.0];
  let v1: Vec<f32> = vec![1.0, 2.0, 1.0, 2.0, 1.0];
  let expectations = &[
    (2 + 2 + 3 + 4 + 3) as f64 /
    ((4 + 1 + 9 + 4 + 9) as f64).sqrt() /
    ((1 + 4 + 1 + 4 + 1) as f64).sqrt(),
    (2 + 2 + 3 + 4 + 3) as f64,
    (1 + 1 + 4 + 0 + 4) as f64,
  ];
  let results = &[
    v0.distance(&v1, Metric::Cosine),
    v0.distance(&v1, Metric::InnerProduct),
    v0.distance(&v1, Metric::L2),
  ];
  let result_aliases = &[
    v0.distance_cosine(&v1),
    v0.distance_inner_product(&v1),
    v0.distance_l2(&v1),
  ];
  assert_eq!(results, expectations);
  assert_eq!(results, result_aliases);
}


#[test]
fn DistancePattern_distance_handle_zeros() {
  let zeros: &[f32] = &[0.0; 144];
  let ones: &[f32] = &[1.0; 144];
  let expectations: &[f64] = &[
    0.0, 0.0, 0.0,
    0.0, 0.0, 144.0,
    1.0, 144.0, 0.0,
  ];
  let results = &[
    zeros.distance(zeros, Metric::Cosine),
    zeros.distance(zeros, Metric::InnerProduct),
    zeros.distance(zeros, Metric::L2),
    zeros.distance(ones, Metric::Cosine),
    zeros.distance(ones, Metric::InnerProduct),
    zeros.distance(ones, Metric::L2),
    ones.distance(ones, Metric::Cosine),
    ones.distance(ones, Metric::InnerProduct),
    ones.distance(ones, Metric::L2),
  ];
  assert_eq!(results, expectations);
}
