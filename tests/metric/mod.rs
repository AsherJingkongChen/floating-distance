use floating_distance::*;

#[test]
fn Metric_compute() {
  let v0: &[f32] = &[1.0, 2.0, 3.0, 2.0, 1.0];
  let v1: &[f32] = &[2.0, 1.0, 1.0, 2.0, 3.0];
  let metrics = [
    Space::Cosine.metric::<f32>(),
    Space::InnerProduct.metric::<f32>(),
    Space::L2.metric::<f32>(),
  ];
  let expectations = &[
    (2 + 2 + 3 + 4 + 3) as f64 /
    ((1 + 4 + 9 + 4 + 1) as f64).sqrt() /
    ((4 + 1 + 1 + 4 + 9) as f64).sqrt(),
    (2 + 2 + 3 + 4 + 3) as f64,
    (1 + 1 + 4 + 0 + 4) as f64,
  ];
  let results =
    &*metrics
      .iter()
      .map(|m| m.compute(v0, v1))
      .collect::<Box<[_]>>();
  assert_eq!(results, expectations);
}

#[test]
fn MetricPattern_distance_using_slice() {
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
      v0.distance(v1, Space::Cosine),
      v0.distance(v1, Space::InnerProduct),
      v0.distance(v1, Space::L2),
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
      v0.distance(&v1, Space::Cosine),
      v0.distance(&v1, Space::InnerProduct),
      v0.distance(&v1, Space::L2),
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
fn MetricPattern_distance_using_vec() {
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
    v0.distance(&v1, Space::Cosine),
    v0.distance(&v1, Space::InnerProduct),
    v0.distance(&v1, Space::L2),
  ];
  let result_aliases = &[
    v0.distance_cosine(&v1),
    v0.distance_inner_product(&v1),
    v0.distance_l2(&v1),
  ];
  assert_eq!(results, expectations);
  assert_eq!(results, result_aliases);
}
