#![feature(test)]

extern crate floating_distance;
extern crate rand;

use floating_distance::*;
use rand::{prelude::*, distributions::*};

pub fn find_closest_pair_by_cosine(
  vectors: &Vec<Vec<f32>>,
) -> (f32, usize, usize) {
  let mut result = (-1_f32, 0, 0);
  for i in 0..vectors.len() {
    for j in i..vectors.len() {
      if i == j { continue; }
      let distance =
        vectors[i].distance(&vectors[j], Metric::Cosine);

      // find the one closest to 1.0
      if result.0 < distance {
        result = (distance, i, j);
      }
    }
  }
  result
}

pub fn generate_vectors(count: usize) -> Vec<Vec<f32>> {
  let mut rng = thread_rng();
  rng.fill_bytes(&mut [0]);
  let dist = Uniform::new_inclusive(-1.0, 1.0);

  let mut vectors = Vec::<Vec<f32>>::new();

  for _ in 0..count {
    vectors.push(
      dist
        .sample_iter(&mut rng)
        .take(768)
        .collect::<Vec<f32>>()
    );
  }
  vectors
}
