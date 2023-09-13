use floating_distance::*;
use crate::simd::*;

#[test]
fn cosine_exact_exact() {
  const LEN: (usize, usize) = (
    2 * MAX_LANE_COUNT_PER_INSTR,
    2 * MAX_LANE_COUNT_PER_INSTR,
  );
  let v0: &[f32; LEN.0] = &[
    3.0, 4.0, 1.0, 3.0, 1.0, 1.0, 4.0, 3.0,
    4.0, 1.0, 3.0, 1.0, 1.0, 4.0, 1.0, 4.0,
  ];
  let v1: &[f32; LEN.1] = &[
    3.0, 4.0, 1.0, 4.0, 3.0, 4.0, 4.0, 1.0,
    3.0, 1.0, 4.0, 1.0, 1.0, 1.0, 1.0, 1.0,
  ];
  let expectation: f32 = (
    9.0 + 16.0 + 1.0 + 12.0 +
    3.0 + 4.0 + 16.0 + 3.0 +
    12.0 + 1.0 + 12.0 + 1.0 +
    1.0 + 4.0 + 1.0 + 4.0_f32
  ) / (
    9.0 + 16.0 + 1.0 + 9.0 +
    1.0 + 1.0 + 16.0 + 9.0 +
    16.0 + 1.0 + 9.0 + 1.0 +
    1.0 + 16.0 + 1.0 + 16.0_f32
  ).sqrt() / (
    9.0 + 16.0 + 1.0 + 16.0 +
    9.0 + 16.0 + 16.0 + 1.0 +
    9.0 + 1.0 + 16.0 + 1.0 +
    1.0 + 1.0 + 1.0 + 1.0_f32
  ).sqrt();
  let result: f32 = v0.distance(v1, Metric::Cosine);
  assert_eq!(result, expectation);
}

#[test]
fn cosine_exact_noexact() {
  const LEN: (usize, usize) = (
    2 * MAX_LANE_COUNT_PER_INSTR,
    2 * MAX_LANE_COUNT_PER_INSTR - 3,
  );
  let v0: &[f32; LEN.0] = &[
    3.0, 4.0, 1.0, 3.0, 1.0, 1.0, 4.0, 3.0,
    4.0, 1.0, 3.0, 1.0, 1.0, 4.0, 1.0, 4.0,
  ];
  let v1: &[f32; LEN.1] = &[
    3.0, 4.0, 1.0, 4.0, 3.0, 4.0, 4.0, 1.0,
    3.0, 1.0, 4.0, 1.0, 1.0,
  ];
  let expectation: f32 = (
    9.0 + 16.0 + 1.0 + 12.0 +
    3.0 + 4.0 + 16.0 + 3.0 +
    12.0 + 1.0 + 12.0 + 1.0 +
    1.0_f32
  ) / (
    9.0 + 16.0 + 1.0 + 9.0 +
    1.0 + 1.0 + 16.0 + 9.0 +
    16.0 + 1.0 + 9.0 + 1.0 +
    1.0_f32
  ).sqrt() / (
    9.0 + 16.0 + 1.0 + 16.0 +
    9.0 + 16.0 + 16.0 + 1.0 +
    9.0 + 1.0 + 16.0 + 1.0 +
    1.0_f32
  ).sqrt();
  let result: f32 = v0.distance(v1, Metric::Cosine);
  assert_eq!(result, expectation);
}

#[test]
fn cosine_noexact_noexact_not_equal() {
  const LEN: (usize, usize) = (
    2 * MAX_LANE_COUNT_PER_INSTR - 1,
    2 * MAX_LANE_COUNT_PER_INSTR - 3,
  );
  let v0: &[f32; LEN.0] = &[
    3.0, 4.0, 1.0, 3.0, 1.0, 1.0, 4.0, 3.0,
    4.0, 1.0, 3.0, 1.0, 1.0, 4.0, 1.0,
  ];
  let v1: &[f32; LEN.1] = &[
    3.0, 4.0, 1.0, 4.0, 3.0, 4.0, 4.0, 1.0,
    3.0, 1.0, 4.0, 1.0, 1.0,
  ];
  let expectation: f32 = (
    9.0 + 16.0 + 1.0 + 12.0 +
    3.0 + 4.0 + 16.0 + 3.0 +
    12.0 + 1.0 + 12.0 + 1.0 +
    1.0_f32
  ) / (
    9.0 + 16.0 + 1.0 + 9.0 +
    1.0 + 1.0 + 16.0 + 9.0 +
    16.0 + 1.0 + 9.0 + 1.0 +
    1.0_f32
  ).sqrt() / (
    9.0 + 16.0 + 1.0 + 16.0 +
    9.0 + 16.0 + 16.0 + 1.0 +
    9.0 + 1.0 + 16.0 + 1.0 +
    1.0_f32
  ).sqrt();
  let result: f32 = v0.distance(v1, Metric::Cosine);
  assert_eq!(result, expectation);
}

#[test]
fn cosine_noexact_noexact_is_equal() {
  const LEN: (usize, usize) = (
    2 * MAX_LANE_COUNT_PER_INSTR - 3,
    2 * MAX_LANE_COUNT_PER_INSTR - 3,
  );
  let v0: &[f32; LEN.0] = &[
    3.0, 4.0, 1.0, 3.0, 1.0, 1.0, 4.0, 3.0,
    4.0, 1.0, 3.0, 1.0, 1.0,
  ];
  let v1: &[f32; LEN.1] = &[
    3.0, 4.0, 1.0, 4.0, 3.0, 4.0, 4.0, 1.0,
    3.0, 1.0, 4.0, 1.0, 1.0,
  ];
  let expectation: f32 = (
    9.0 + 16.0 + 1.0 + 12.0 +
    3.0 + 4.0 + 16.0 + 3.0 +
    12.0 + 1.0 + 12.0 + 1.0 +
    1.0_f32
  ) / (
    9.0 + 16.0 + 1.0 + 9.0 +
    1.0 + 1.0 + 16.0 + 9.0 +
    16.0 + 1.0 + 9.0 + 1.0 +
    1.0_f32
  ).sqrt() / (
    9.0 + 16.0 + 1.0 + 16.0 +
    9.0 + 16.0 + 16.0 + 1.0 +
    9.0 + 1.0 + 16.0 + 1.0 +
    1.0_f32
  ).sqrt();
  let result: f32 = v0.distance(v1, Metric::Cosine);
  assert_eq!(result, expectation);
}
