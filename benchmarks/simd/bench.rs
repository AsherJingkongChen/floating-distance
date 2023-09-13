#![feature(test)]

extern crate test;
extern crate utils_benchmarks;

use test::Bencher;
use utils_benchmarks::*;

#[bench]
fn simd(bencher: &mut Bencher) {
  let vectors = generate_vectors(16);
  bencher.iter(|| {
    find_closest_pair_by_cosine(&vectors)
  });
}
