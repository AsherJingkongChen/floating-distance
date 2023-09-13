use floating_distance::*;

pub fn manhattan_distance() {
  let v0: &[f32; 4] = &[1.0, -2.0, -3.5, 99.0];
  let v1: &[f32; 3] = &[-3.0, -5.0, -5.3];

  let result = v0.distance(v1, Metric::Manhattan);
  let expectation = 4.0 + 3.0 + 1.8;

  assert_eq!(result, expectation);

  println!("\
Manhattan distance from {v0:?} to {v1:?} is {result:?}, \
the former is truncated",
  );
}
