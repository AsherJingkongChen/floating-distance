use floating_distance::*;

pub fn euclidean_distance() {
  let v0: [f32; 2] = [3.0, 10.0];
  let v1: [f32; 2] = [-4.0, 3.0];

  let result = Metric::Euclidean.measure::<f32>(&v0, &v1);
  let expectation = 7.0 * 7.0 + 7.0 * 7.0;

  assert_eq!(result, expectation);

  println!(
    "Euclidean distance from {v0:?} to {v1:?} is {result:?}"
  );
}
