//  TODO
//
//  1.  Symbol interface:
//    - xor
//    - hash -> u64
//  2.  Coding window
//  3.  Random mapping
//  4.  Encoder
//  5.  Decoder
//
//  Tests:
//    - Encode and decode
//    - Example
//

//  NOTE
//  - Investigate static/dynamic dispatch in regard to
//    the performance when using traits like Symbol.
//  - Hash values are hardcoded to be u64, make it more generic.

use std::vec::Vec;

trait Symbol {
  fn xor(&self, other: &Self) -> Self where Self: Sized;
  fn hash(&self) -> u64;
}

struct HashedSymbol<T: Symbol> {
  symbol: T,
  hash:   u64,
}

struct CodedSymbol<T: Symbol> {
  symbol: HashedSymbol<T>,
  count:  i64,
}

struct Encoder<T: Symbol> {
  window: Vec<T>,
}

struct Decoder<T: Symbol> {
  local:  Vec<T>,
  remote: Vec<T>,
}

impl<T: Symbol> Encoder<T> {
  fn new() -> Self {
    todo!()
  }
  
  fn add_symbol(&mut self, _sym: &dyn Symbol) {
    todo!()
  }

  fn produce_next_coded_symbol(&mut self) -> CodedSymbol<T> {
    todo!()
  }
}

impl<T: Symbol> Decoder<T> {
  fn new() -> Self {
    todo!()
  }
  
  fn add_symbol(&mut self, _sym: &dyn Symbol) {
    todo!()
  }

  fn add_coded_symbol(&mut self, _sym: CodedSymbol<T>) {
    todo!()
  }

  fn try_decode(&mut self) {
    todo!()
  }

  fn decoded(&self) -> bool {
    todo!()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::BTreeSet;
  // use crypto::sha2::Sha256;
  // use std::hash::SipHaisher;

  const TEST_SYMBOL_SIZE: usize = 64;

  type TestSymbol = [u8; TEST_SYMBOL_SIZE];

  fn new_test_symbol(_i: u64) -> TestSymbol {
    todo!()
  }

  impl Symbol for TestSymbol {
    fn xor(&self, _other: &TestSymbol) -> TestSymbol {
      todo!()
    }
  
    fn hash(&self) -> u64 {
      todo!()
    }
  }
  
  #[test]
  fn encode_and_decode() {
    let mut enc: Encoder<TestSymbol> = Encoder::new();
    let mut dec: Decoder<TestSymbol> = Decoder::new();

    let mut local  : BTreeSet<u64> = BTreeSet::new();
    let mut remote : BTreeSet<u64> = BTreeSet::new();

    let nlocal  = 50000;
    let nremote = 50000;
    let ncommon = 100000;

    let mut next_id: u64 = 0;

    for _ in 0..nlocal {
      let s = new_test_symbol(next_id);
      next_id += 1;
      dec.add_symbol(&s);
      local.insert(s.hash());
    }
    for _ in 0..nremote {
      let s = new_test_symbol(next_id);
      next_id += 1;
      enc.add_symbol(&s);
      remote.insert(s.hash());
    }
    for _ in 0..ncommon {
      let s = new_test_symbol(next_id);
      next_id += 1;
      enc.add_symbol(&s);
      dec.add_symbol(&s);
    }

    let mut ncw = 0;

    loop {
      dec.add_coded_symbol(enc.produce_next_coded_symbol());
      ncw += 1;
      dec.try_decode();
      if dec.decoded() {
        break;
      }
    }

    for v in dec.remote.iter() {
      remote.remove(&v.hash());
    }

    for v in dec.local.iter() {
      local.remove(&v.hash());
    }

    assert_eq!(remote.len(), 0);
    assert_eq!(local.len(), 0);
    assert!(dec.decoded());

    println!("{} codewords until fully decoded", ncw);
  }

  #[test]
  fn example() {
    todo!();
  }
}
