use floating_distance::{Vector, Metric};

pub fn euclidean_distance() {
  let v0: [f32; 2] = [3.0, 10.0];
  let v1: [f32; 2] = [-4.0, 3.0];
  let result = v0.distance(&v1, Metric::Euclidean);
  let expectation = 7.0 * 7.0 + 7.0 * 7.0;

  assert_eq!(result, expectation);

  println!(
    "Euclidean distance from {v0:?} to {v1:?} is {result:?}"
  );
}
