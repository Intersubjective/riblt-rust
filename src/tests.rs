use super::*;
use std::{fs::*, io::Write};

#[test]
fn print_mapping() {
  let mut m = RandomMapping {
    prng     : 1234567891,
    last_idx : 0,
  };

  let mut f = File::create("mapping.txt").unwrap();

  for _ in 0..20000 {
    write!(&mut f, "{}\n", m.next_index()).unwrap();
  }
}
