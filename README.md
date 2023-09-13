# Measure distance between floating-point vectors in Rust
[<img alt="crates.io" src="https://img.shields.io/crates/v/floating-distance.svg?color=fe7d37&logo=rust" height="20">](https://crates.io/crates/floating-distance)
[<img alt="docs.rs" src="https://docs.rs/floating-distance/badge.svg" height="20">](https://docs.rs/floating-distance/)
[<img alt="coverage" src="https://img.shields.io/codecov/c/github/AsherJingkongChen/floating-distance?logo=codecov" height="20">](https://app.codecov.io/gh/AsherJingkongChen/floating-distance)
[<img alt="GitHub Actions" src="https://github.com/AsherJingkongChen/floating-distance/actions/workflows/main.yml/badge.svg" height="20">](https://github.com/AsherJingkongChen/floating-distance/actions/workflows/main.yml)

## Hello Rustaceans!
This is a Rust library. Here you can do:
1. Add this crate as a dependency of your Rust project:
```shell
cargo add floating-distance
```
2. Check out the [documentation](https://docs.rs/floating-distance) and
[source codes](https://github.com/AsherJingkongChen/floating-distance.git) (click the badges above for more information)
3. Clone the GitHub repository and run the examples:
```shell
cargo run --example default
```

## Examples
1. Measure the cosine similarity between two vectors `v0` and `v1`
```rust
use floating_distance::*;

fn main() {
  let v0: &[f32] = &[1.0, 2.0, 2.0, 1.0, 2.0, 1.0, 1.0];
  let v1: &[f32] = &[2.0, 1.0, 1.0, 1.0, 2.0, 1.0, 2.0];
  let metric = Metric::Cosine;
  let result = v0.distance(v1, metric);
  let expectation: f32 = 14.0 / (4.0 * 4.0);

  assert_eq!(result, expectation);
}
```

## feature = ["simd"]
### What's special?
SIMD is the acronym for **S**ingle **I**nstruction **M**ultiple **D**ata

Modern CPUs have special instructions. We can use them to accelerate vector computations!

### How to start?
You can enable `simd` feature in this crate by the following steps:
1. Specify `features = ["simd"]` in `Cargo.toml` manifest:
```ini
[dependencies]
floating-distance = { version = "*", features = ["simd"] }
```
2. Compile with Rust nightly version. You can add this to `rust-toolchain.toml`, which is next to `Cargo.toml`:
```ini
[toolchain]
channel = "nightly"
```
3. Choose the SIMD instruction sets which are supported by the target architecture. You can use `RUSTFLAGS` environment variable like these:
```shell
RUSTFLAGS="-C target-feature=+sse4.1" cargo build
```
```shell
RUSTFLAGS="-C target-feature=+avx2" cargo build --release
```
You can find all target features of Rust by this:
```shell
rustc --print target-features
```
The table shows how this library interprets target features:

Data width | Target features
--- | ---
128-bits | `sse`, `sse2`, `sse3`, `sse4.1`, `sse4.2`, `sse4a`, `ssse3`
256-bits | `avx`, `avx2`
512-bits | `avx512vl`

### How great is it?
*TODO*

### Note
This feature is built by experimental features of Rust
