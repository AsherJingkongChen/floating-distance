use floating_distance::*;

#[test]
fn DistancePattern_distance_using_slice() {
  let v0: &[f32] = &[1.0, 2.0, 3.0, 2.0, 1.0];
  let v1: &[f32] = &[2.0, 1.0, 1.0, 2.0, 3.0];
  let expectations = &[
    (2 + 2 + 3 + 4 + 3) as f32 /
    ((1 + 4 + 9 + 4 + 1) as f32).sqrt() /
    ((4 + 1 + 1 + 4 + 9) as f32).sqrt(),
    (2 + 2 + 3 + 4 + 3) as f32,
    (1 + 1 + 2 + 0 + 2) as f32,
    (1 + 1 + 4 + 0 + 4) as f32,
  ];
  {
    let results = &[
      v0.distance(v1, Metric::Cosine),
      v0.distance(v1, Metric::InnerProduct),
      v0.distance(v1, Metric::L1),
      v0.distance(v1, Metric::L2),
    ];
    let result_aliases = &[
      v0.distance_cosine(v1),
      v0.distance_inner_product(v1),
      v0.distance_l1(v1),
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
      v0.distance(&v1, Metric::L1),
      v0.distance(&v1, Metric::L2),
    ];
    let result_aliases = &[
      v0.distance_cosine(&v1),
      v0.distance_inner_product(&v1),
      v0.distance_l1(&v1),
      v0.distance_l2(&v1),
    ];
    assert_eq!(results, expectations);
    assert_eq!(results, result_aliases);
  }
}

#[test]
fn DistancePattern_distance_using_vec() {
  let v0: Vec<f64> = vec![-2.0, 1.0, 3.0, 2.0, 3.0];
  let v1: Vec<f64> = vec![1.0, 4.0, -1.0, 2.0, 4.0];
  let expectations = &[
    (-2 + 4 + -3 + 4 + 12) as f64 /
    ((4 + 1 + 9 + 4 + 9) as f64).sqrt() /
    ((1 + 16 + 1 + 4 + 16) as f64).sqrt(),
    (-2 + 4 + -3 + 4 + 12) as f64,
    (3 + 3 + 4 + 0 + 1) as f64,
    (9 + 9 + 16 + 0 + 1) as f64,
  ];
  let results = &[
    v0.distance(&v1, Metric::Cosine),
    v0.distance(&v1, Metric::InnerProduct),
    v0.distance(&v1, Metric::L1),
    v0.distance(&v1, Metric::L2),
  ];
  let result_aliases = &[
    v0.distance_cosine(&v1),
    v0.distance_inner_product(&v1),
    v0.distance_l1(&v1),
    v0.distance_l2(&v1),
  ];
  assert_eq!(results, expectations);
  assert_eq!(results, result_aliases);
}


#[test]
fn DistancePattern_distance_handle_zeros() {
  let zeros: &[f32] = &[0.0; 144];
  let ones: &[f32] = &[1.0; 144];
  let expectations: &[f32] = &[
    0.0, 0.0, 0.0, 0.0,
    0.0, 0.0, 144.0, 144.0,
    1.0, 144.0, 0.0, 0.0,
  ];
  let results = &[
    zeros.distance(zeros, Metric::Cosine),
    zeros.distance(zeros, Metric::InnerProduct),
    zeros.distance(zeros, Metric::L1),
    zeros.distance(zeros, Metric::L2),
    zeros.distance(ones, Metric::Cosine),
    zeros.distance(ones, Metric::InnerProduct),
    zeros.distance(ones, Metric::L1),
    zeros.distance(ones, Metric::L2),
    ones.distance(ones, Metric::Cosine),
    ones.distance(ones, Metric::InnerProduct),
    ones.distance(ones, Metric::L1),
    ones.distance(ones, Metric::L2),
  ];
  assert_eq!(results, expectations);
}

#[test]
fn DistancePattern_distance_handle_outbound_cosine() {
  let v0: &[f32] = &[0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
  let v1: &[f32] = &[0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
  let result = v0.distance_cosine(v1);
  let expectation: f32 = 1.0;
  assert_eq!(result, expectation);

  let v0: &[f32] = &[0.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, -1.0, 0.0, -1.0, -1.0, 0.0, -1.0, -1.0, 0.0, -1.0];
  let v1: &[f32] = &[0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
  let result = v0.distance_cosine(v1);
  let expectation: f32 = -1.0;

  assert_eq!(result, expectation);
}
