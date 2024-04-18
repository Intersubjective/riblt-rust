use super::*;

#[test]
fn mapping() {
  let mut m = RandomMapping {
    prng     : 1234567891,
    last_idx : 0,
  };

  for _ in 0..1000 {
    m.next_index();
  }
}
