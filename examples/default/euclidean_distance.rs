use floating_distance::{MetricPattern, Space};

pub fn euclidean_distance() {
  let v0: [f32; 2] = [3.0, 10.0];
  let v1: [f32; 2] = [-4.0, 3.0];
  let result = v0.distance(&v1, Space::L2);
  let expectation = 7.0 * 7.0 + 7.0 * 7.0;

  assert_eq!(result, expectation);

  let metric = Space::L2.metric::<f32>();
  let result = metric.compute(&v0, &v1);

  assert_eq!(result, expectation);

  println!("Distance from {:?} to {:?}) is {:?}", v0, v1, result);
}
