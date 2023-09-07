pub use speedy::{Readable, Writable};
use crate::metric::*;

#[derive(
  Debug, Eq, PartialEq,
  Readable, Writable,
)]
pub enum Space {
  Cosine,
  InnerProduct,
  L2Norm,
}

impl Space {
  pub fn metric<T>(&self) -> Box<dyn Metric<T>>
  where
    T: Copy + Into<f32>,
  {
    match self {
      Space::Cosine => Box::from(MetricCosine),
      Space::InnerProduct => Box::from(MetricInnerProduct),
      Space::L2Norm => Box::from(MetricL2Norm),
    }
  }
}

//\ test \//

#[cfg(test)]
mod test {
  use crate::*;

  #[test]
  fn compute() {
    let v0: &[f32] = &[1.0,2.0,3.0];
    let v1: &[f32] = &[3.0,2.0,3.0];
    let m: Box<dyn Metric<f32>> = Space::L2Norm.metric();
    let r = m.compute(v0, v1);
    assert_eq!(r, 4.0);
  }

  #[test]
  fn speedy() {
    let spaces = [
      Space::Cosine,
      Space::InnerProduct,
      Space::L2Norm,
    ];
    let spaces_reloaded =
      &*spaces
        .iter()
        .map(|space| {
          Space::read_from_buffer(
            space
              .write_to_vec()
              .unwrap()
              .as_slice()
          ).unwrap()
        })
        .collect::<Box<[_]>>();

    assert_eq!(spaces, spaces_reloaded);
  }
}
