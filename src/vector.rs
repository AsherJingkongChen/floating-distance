#[cfg(docsrs)]
use crate::*;

/// A type alias of [`slice`],
/// which represents a contiguous block of scalar data.
/// 
/// See also [`Metric::measure`] for
/// measuring distance between two vectors.
pub type Vector<'a, S> = &'a [S];
