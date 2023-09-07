use distance::*;

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
  assert_ne!(spaces[0], spaces_reloaded[1]);
  assert_ne!(spaces[0], spaces_reloaded[2]);
  assert_ne!(spaces[1], spaces_reloaded[2]);
}
