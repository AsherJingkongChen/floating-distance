use num_traits::Float;
use std::ops::{Add, Mul, Sub};
use crate::*;

/// The floating-point scalar type
pub trait FloatingPoint:
  AutoSimdElement + Default + Float {}

/// The floating-point packed type
pub trait FloatingPoints<T>:
  AutoSimdFloat<Scalar = T> + Default +
  Add<Output = Self> + Mul<Output = Self> +
  Sub<Output = Self> {}

impl FloatingPoint for f32 {}
impl FloatingPoint for f64 {}

impl FloatingPoints<f32> for AutoSimd<f32> {}
impl FloatingPoints<f64> for AutoSimd<f64> {}
