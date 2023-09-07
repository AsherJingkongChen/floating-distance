use distance::*;

#[test]
fn metric_compute_1() {
  let v0: &[f32] = &[1.0, 2.0, 3.0, 2.0, 1.0];
  let v1: &[f32] = &[2.0, 1.0, 1.0, 2.0, 3.0];
  let metrics = [
    Space::Cosine.metric::<f32>(),
    Space::InnerProduct.metric::<f32>(),
    Space::L2Norm.metric::<f32>(),
  ];
  let expectations = &[
    (2 + 2 + 3 + 4 + 3) as f32 /
    ((1 + 4 + 9 + 4 + 1) as f32).sqrt() /
    ((4 + 1 + 1 + 4 + 9) as f32).sqrt(),
    (2 + 2 + 3 + 4 + 3) as f32,
    (1 + 1 + 4 + 0 + 4) as f32,
  ];
  let results =
    &*metrics
      .iter()
      .map(|m| m.compute(v0, v1))
      .collect::<Box<[_]>>();
  assert_eq!(results, expectations);
}