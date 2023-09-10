# Compute distance between floating-point vectors in rust
[<img alt="crates.io" src="https://img.shields.io/crates/v/floating-distance.svg?color=fe7d37&logo=rust" height="20">](https://crates.io/crates/floating-distance)
[<img alt="coverage" src="https://img.shields.io/codecov/c/github/AsherJingkongChen/floating-distance?logo=codecov" height="20">](https://app.codecov.io/gh/AsherJingkongChen/floating-distance)
[<img alt="GitHub actions" src="https://github.com/AsherJingkongChen/floating-distance/actions/workflows/main.yml/badge.svg" height="20">](https://github.com/AsherJingkongChen/floating-distance/actions/workflows/main.yml)

## Documentations
[Docs.rs](https://docs.rs/floating-distance)

## Examples
```rust
use floating_distance::*;

fn main() {
  let v0: &[f32] = &[1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0];
  let v1: &[f32] = &[2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 2.0];
  let metric = Metric::Cosine;
  let result = v0.distance(v1, metric);
  let expectation: f64 = 14.0 / (4.0 * 4.0);

  assert_eq!(result, expectation);
}
```
