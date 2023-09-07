#![cfg_attr(feature = "simd", feature(portable_simd, array_chunks))]
#![allow(non_snake_case)]

//\ declare \//

pub mod metric;
pub mod space;

//\ export \//

pub use crate::metric::*;
pub use crate::space::*;
