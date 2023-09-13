use std::mem::size_of;

mod cosine;
mod euclidean;
mod inner_product;
mod manhattan;

pub const MAX_LANE_COUNT_PER_INSTR: usize =
  512 / 8 / size_of::<f64>();
