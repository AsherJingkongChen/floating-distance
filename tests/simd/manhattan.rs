use floating_distance::*;
use crate::simd::*;

#[test]
fn manhattan_exact_exact() {
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
  let expectation: f32 =
    0.0 + 0.0 + 0.0 + 1.0 +
    2.0 + 3.0 + 0.0 + 2.0 +
    1.0 + 0.0 + 1.0 + 0.0 +
    0.0 + 3.0 + 0.0 + 3.0_f32;
  let result: f32 = v0.distance(v1, Metric::Manhattan);
  assert_eq!(result, expectation);
}

#[test]
fn manhattan_exact_noexact() {
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
  let expectation: f32 =
    0.0 + 0.0 + 0.0 + 1.0 +
    2.0 + 3.0 + 0.0 + 2.0 +
    1.0 + 0.0 + 1.0 + 0.0 +
    0.0_f32;
  let result: f32 = v0.distance(v1, Metric::Manhattan);
  assert_eq!(result, expectation);
}

#[test]
fn manhattan_noexact_noexact_not_equal() {
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
  let expectation: f32 =
    0.0 + 0.0 + 0.0 + 1.0 +
    2.0 + 3.0 + 0.0 + 2.0 +
    1.0 + 0.0 + 1.0 + 0.0 +
    0.0_f32;
  let result: f32 = v0.distance(v1, Metric::Manhattan);
  assert_eq!(result, expectation);
}

#[test]
fn manhattan_noexact_noexact_is_equal() {
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
  let expectation: f32 =
    0.0 + 0.0 + 0.0 + 1.0 +
    2.0 + 3.0 + 0.0 + 2.0 +
    1.0 + 0.0 + 1.0 + 0.0 +
    0.0_f32;
  let result: f32 = v0.distance(v1, Metric::Manhattan);
  assert_eq!(result, expectation);
}
