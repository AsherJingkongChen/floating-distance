use approx::*;
use floating_distance::*;

#[test]
fn cosine_greater_equal_less() {
  let metric = Metric::Cosine;
  let v0: &[f32] = &[-1.0, 2.0, -3.0, -2.0, 2.0, 1.0];
  let v1: &[f32] = &[3.0, -2.0, -2.0, 1.0, -1.0, 2.0];
  let v2: &[f32] = &[-1.0, 3.0, -1.0, 2.0, 2.0, -2.0];
  let d01: f32 = metric.measure::<f32>(v0, v1);
  let d02: f32 = metric.measure::<f32>(v0, v2);

  let d01_expected: f32 = -3.0 / 23.0;
  assert_ulps_eq!(d01, d01_expected);

  let d02_expected: f32 = 8.0 / 23.0;
  assert_ulps_eq!(d02, d02_expected);

  let expectation = Some(Ordering::Greater);
  let result = metric.compare(&d01, &d02);
  assert_eq!(result, expectation);

  let expectation = Some(Ordering::Less);
  let result = metric.compare(&d02, &d01);
  assert_eq!(result, expectation);

  let expectation = Some(Ordering::Equal);
  let result = metric.compare(&d01, &d01);
  assert_eq!(result, expectation);
}

#[test]
fn euclidean_greater_equal_less() {
  let metric = Metric::Euclidean;
  let v0: &[f32] = &[-1.0, 2.0, -3.0, -2.0, 2.0, 1.0];
  let v1: &[f32] = &[3.0, -2.0, -2.0, 1.0, -1.0, 2.0];
  let v2: &[f32] = &[-1.0, 3.0, -1.0, 2.0, 2.0, -2.0];
  let d01: f32 = metric.measure::<f32>(v0, v1);
  let d02: f32 = metric.measure::<f32>(v0, v2);

  let d01_expected: f32 = 52.0;
  assert_eq!(d01, d01_expected);

  let d02_expected: f32 = 30.0;
  assert_eq!(d02, d02_expected);

  let expectation = Some(Ordering::Greater);
  let result = metric.compare(&d01, &d02);
  assert_eq!(result, expectation);

  let expectation = Some(Ordering::Less);
  let result = metric.compare(&d02, &d01);
  assert_eq!(result, expectation);

  let expectation = Some(Ordering::Equal);
  let result = metric.compare(&d01, &d01);
  assert_eq!(result, expectation);
}

#[test]
fn inner_product_greater_equal_less() {
  let metric = Metric::InnerProduct;
  let v0: &[f32] = &[-1.0, 2.0, -3.0, -2.0, 2.0, 1.0];
  let v1: &[f32] = &[3.0, -2.0, -2.0, 1.0, -1.0, 2.0];
  let v2: &[f32] = &[-1.0, 3.0, -1.0, 2.0, 2.0, -2.0];
  let d01: f32 = metric.measure::<f32>(v0, v1);
  let d02: f32 = metric.measure::<f32>(v0, v2);

  let d01_expected: f32 = -3.0;
  assert_eq!(d01, d01_expected);

  let d02_expected: f32 = 8.0;
  assert_eq!(d02, d02_expected);

  let expectation = Some(Ordering::Greater);
  let result = metric.compare(&d01, &d02);
  assert_eq!(result, expectation);

  let expectation = Some(Ordering::Less);
  let result = metric.compare(&d02, &d01);
  assert_eq!(result, expectation);

  let expectation = Some(Ordering::Equal);
  let result = metric.compare(&d01, &d01);
  assert_eq!(result, expectation);
}

#[test]
fn manhattan_greater_equal_less() {
  let metric = Metric::Manhattan;
  let v0: &[f32] = &[-1.0, 2.0, -3.0, -2.0, 2.0, 1.0];
  let v1: &[f32] = &[3.0, -2.0, -2.0, 1.0, -1.0, 2.0];
  let v2: &[f32] = &[-1.0, 3.0, -1.0, 2.0, 2.0, -2.0];
  let d01: f32 = metric.measure::<f32>(v0, v1);
  let d02: f32 = metric.measure::<f32>(v0, v2);

  let d01_expected: f32 = 16.0;
  assert_eq!(d01, d01_expected);

  let d02_expected: f32 = 10.0;
  assert_eq!(d02, d02_expected);

  let expectation = Some(Ordering::Greater);
  let result = metric.compare(&d01, &d02);
  assert_eq!(result, expectation);

  let expectation = Some(Ordering::Less);
  let result = metric.compare(&d02, &d01);
  assert_eq!(result, expectation);

  let expectation = Some(Ordering::Equal);
  let result = metric.compare(&d01, &d01);
  assert_eq!(result, expectation);
}
