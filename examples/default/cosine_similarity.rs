use floating_distance::*;
use rand::{prelude::*, distributions::*};

pub fn cosine_similarity() {
  let count = 99;
  let mut rng = thread_rng();
  rng.fill_bytes(&mut [0]);
  // let dist = Uniform::new_inclusive(-1.0_f32, 1.0_f32);
  let dist = Bernoulli::new(0.9).unwrap();

  let mut vectors = Vec::<Vec<f32>>::new();

  for _ in 0..count {
    vectors.push(
      dist
        .sample_iter(&mut rng)
        .take(768)
        .map(|v| if v { 1.0 } else { 0.0 })
        .collect::<Vec<f32>>()
    );
  }

  let mut result = (-1_f64, 0, 0);
  for i in 0..count {
    for j in i..count {
      if i == j { continue; }
      let distance = vectors[i].distance_cosine(&vectors[j]);

      // find the one closest to 1.0
      if result.0 < distance {
        result = (distance, i, j);
      }
    }
  }

  println!("{:?}", result);
}