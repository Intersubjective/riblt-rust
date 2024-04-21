use super::*;
use super::testing::*;
use std::collections::BTreeSet;
use std::{fs::*, io::Write};

#[test]
fn example() {
  let alice : [TestU64; 10] = [1, 2, 3, 4, 5, 6, 7, 8,  9, 10];
  let bob   : [TestU64; 10] = [1, 3, 4, 5, 6, 7, 8, 9, 10, 11];

  let mut enc = Encoder::<TestU64>::new();
  for x in alice {
    enc.add_symbol(&x);
  }

  let mut dec = Decoder::<TestU64>::new();
  for x in bob {
    dec.add_symbol(&x);
  }

  let mut cost = 0;

  loop {
    let s = enc.produce_next_coded_symbol();
    cost += 1;
    dec.add_coded_symbol(&s);
    assert!(!dec.try_decode().is_err());
    if dec.decoded() {
      break;
    }
  }

  //  2 is exclusive to Alice
  assert_eq!(dec.remote.symbols[0].symbol, 2);

  //  11 is exclusive to Bob
  assert_eq!(dec.local.symbols[0].symbol, 11);

  assert_eq!(cost, 2);
} 

#[test]
fn encode_and_decode() {
  let mut enc = Encoder::<TestSymbol>::new();
  let mut dec = Decoder::<TestSymbol>::new();

  let mut local  = BTreeSet::<u64>::new();
  let mut remote = BTreeSet::<u64>::new();

  let nlocal  = 5000;  // 50000;
  let nremote = 5000;  // 50000;
  let ncommon = 10000; // 100000;

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
    dec.add_coded_symbol(&enc.produce_next_coded_symbol());
    ncw += 1;
    assert!(!dec.try_decode().is_err());
    if dec.decoded() {
      break;
    }
 }

  for v in dec.remote.symbols.iter() {
    remote.remove(&v.hash);
  }

  for v in dec.local.symbols.iter() {
    local.remove(&v.hash);
  }

  assert_eq!(remote.len(), 0);
  assert_eq!(local.len(), 0);
  assert!(dec.decoded());

  println!("{} codewords until fully decoded", ncw);
}

#[test]
fn reset() {
  let alice_0 : [TestU64; 10] = [1, 2, 3, 4, 5, 6, 7, 8,  9, 10];
  let bob_0   : [TestU64; 10] = [1, 3, 4, 5, 6, 7, 8, 9, 10, 11];
  let alice_1 : [TestU64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 10, 11];
  let bob_1   : [TestU64; 10] = [1, 2, 4, 5, 6, 7, 8, 9, 10, 11];

  let mut enc = Encoder::<TestU64>::new();
  for x in alice_0 {
    enc.add_symbol(&x);
  }

  let mut dec = Decoder::<TestU64>::new();
  for x in bob_0 {
    dec.add_symbol(&x);
  }

  let mut cost = 0;

  loop {
    let s = enc.produce_next_coded_symbol();
    cost += 1;
    dec.add_coded_symbol(&s);
    assert!(!dec.try_decode().is_err());
    if dec.decoded() {
      break;
    }
  }

  enc.reset();
  dec.reset();

  for x in alice_1 {
    enc.add_symbol(&x);
  }

  for x in bob_1 {
    dec.add_symbol(&x);
  }

  cost = 0;

  loop {
    let s = enc.produce_next_coded_symbol();
    cost += 1;
    dec.add_coded_symbol(&s);
    assert!(!dec.try_decode().is_err());
    if dec.decoded() {
      break;
    }
  }

  //  3 is exclusive to Alice
  assert_eq!(dec.remote.symbols[0].symbol, 3);

  //  9 is exclusive to Bob
  assert_eq!(dec.local.symbols[0].symbol, 9);

  assert_eq!(cost, 2);
} 

#[test]
fn get_symbols() {
  let alice : [TestU64; 10] = [1, 2, 3, 4, 5, 6, 7, 8,  9, 10];
  let bob   : [TestU64; 10] = [1, 3, 4, 5, 6, 7, 8, 9, 10, 11];

  let mut enc = Encoder::<TestU64>::new();
  for x in alice {
    enc.add_symbol(&x);
  }

  let mut dec = Decoder::<TestU64>::new();
  for x in bob {
    dec.add_symbol(&x);
  }

  let mut cost = 0;

  loop {
    let s = enc.produce_next_coded_symbol();
    cost += 1;
    dec.add_coded_symbol(&s);
    assert!(!dec.try_decode().is_err());
    if dec.decoded() {
      break;
    }
  }

  let remote = dec.get_remote_symbols();
  let local  = dec.get_local_symbols();

  //  2 is exclusive to Alice
  assert_eq!(remote[0].symbol, 2);

  //  11 is exclusive to Bob
  assert_eq!(local[0].symbol, 11);

  assert_eq!(cost, 2);
}

#[test]
fn print_mapping() {
  let mut m = RandomMapping {
    prng     : 1234567891,
    last_idx : 0,
  };

  let mut f = File::create("mapping.txt").unwrap();

  for _ in 0..20000 {
    write!(&mut f, "{}", m.next_index()).unwrap();
  }
}
