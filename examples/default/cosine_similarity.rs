use floating_distance::*;
use rand::{prelude::*, distributions::*};

pub fn cosine_similarity() {
  let count = 100;
  let mut rng = thread_rng();
  // let dist = Uniform::new_inclusive(-1.0_f32, 1.0_f32);
  let dist = Bernoulli::new(0.9).unwrap();

  let mut vectors = Vec::<Vec<f32>>::new();

  for _ in 0..count {
    vectors.push(
      dist
        .sample_iter(&mut rng)
        .take(384)
        .map(|v| if v { 1.0 } else { 0.0 })
        .collect::<Vec<f32>>()
    );
  }

  let mut result = (-1_f32, 0, 0);
  for i in 0..count {
    for j in i..count {
      if i == j { continue; }
      let distance = Metric::Cosine.measure::<f32>(
        &vectors[i], &vectors[j]
      );

      // find the one closest to 1.0
      if result.0 < distance {
        result = (distance, i, j);
      }
    }
  }

  println!("(score, i, j) = {:?}", result);
}