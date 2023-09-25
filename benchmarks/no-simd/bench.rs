#![feature(test)]

extern crate test;
extern crate benchmarks_utils;

use test::Bencher;
use benchmarks_utils::*;

#[bench]
fn no_simd(bencher: &mut Bencher) {
  let vectors = generate_vectors(16);
  bencher.iter(|| {
    find_closest_pair_by_cosine(&vectors)
  });
}
