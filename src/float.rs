/// The widest floating-point type for any computation
pub type BigFloat = f64;

/// For safety, all floats are coerced into the widest type
pub trait Float: Copy + Into<BigFloat> {}

impl Float for f32 {}
impl Float for f64 {}
