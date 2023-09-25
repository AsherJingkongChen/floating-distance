use floating_distance::*;

#[test]
fn all_handle_zeros_and_ones() {
  let zeros: &[f32] = &[0.0; 144];
  let ones: &[f32] = &[1.0; 144];
  let expectations: &[f32] = &[
    0.0, 0.0, 0.0, 0.0,
    0.0, 144.0, 0.0, 144.0,
    1.0, 0.0, 144.0, 0.0,
  ];
  let results = &[
    Metric::Cosine.measure::<f32>(zeros, zeros),
    Metric::Euclidean.measure::<f32>(zeros, zeros),
    Metric::InnerProduct.measure::<f32>(zeros, zeros),
    Metric::Manhattan.measure::<f32>(zeros, zeros),
    Metric::Cosine.measure::<f32>(zeros, ones),
    Metric::Euclidean.measure::<f32>(zeros, ones),
    Metric::InnerProduct.measure::<f32>(zeros, ones),
    Metric::Manhattan.measure::<f32>(zeros, ones),
    Metric::Cosine.measure::<f32>(ones, ones),
    Metric::Euclidean.measure::<f32>(ones, ones),
    Metric::InnerProduct.measure::<f32>(ones, ones),
    Metric::Manhattan.measure::<f32>(ones, ones),
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
    Metric::Cosine.measure::<f32>(empty.0, empty.1),
    Metric::Euclidean.measure::<f32>(empty.0, empty.1),
    Metric::InnerProduct.measure::<f32>(empty.0, empty.1),
    Metric::Manhattan.measure::<f32>(empty.0, empty.1),
  ];
  assert_eq!(results, expectations);
}

#[test]
fn cosine_clamp() {
  let v0: &[f32] = &[
    0.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0,
    0.0, 1.0, 1.0, 0.0,
    1.0, 1.0, 0.0, 1.0,
  ];
  let v1: &[f32] = &[
    0.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0,
    0.0, 1.0, 1.0, 0.0,
    1.0, 1.0, 0.0, 1.0,
  ];
  let result = Metric::Cosine.measure::<f32>(v0, v1);
  let expectation = 1.0;
  assert_eq!(result, expectation);

  let v0: &[f32] = &[
    0.0, -1.0, -1.0, -1.0,
    -1.0, -1.0, -1.0, -1.0,
    0.0, -1.0, -1.0, 0.0,
    -1.0, -1.0, 0.0, -1.0,
  ];
  let v1: &[f32] = &[
    0.0, 1.0, 1.0, 1.0,
    1.0, 1.0, 1.0, 1.0,
    0.0, 1.0, 1.0, 0.0,
    1.0, 1.0, 0.0, 1.0,
  ];
  let result = Metric::Cosine.measure::<f32>(v0, v1);
  let expectation = -1.0;
  assert_eq!(result, expectation);
}
