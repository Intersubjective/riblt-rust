//  Dynamic byte array implementation for the Symbol trait.
//  Required for the cross-language API.

use super::*;
use std::vec::Vec;
use std::marker::PhantomData;

pub trait Hasher {
  fn new() -> Self;
  fn hash(&self, bytes: &[u8]) -> u64;
}

#[derive(Clone)]
pub struct ByteArray<H: Hasher + Clone> {
  pub v           : Vec<u8>,
      hasher_type : PhantomData<H>,
} 

impl<H: Hasher + Clone> ByteArray<H> {
  pub fn new() -> Self {
    return ByteArray::<H> {
      v           : Vec::<u8>::new(),
      hasher_type : PhantomData::<H> {},
    };
  }

  pub fn from_slice(bytes: &[u8]) -> Self {
    let mut s = ByteArray::<H>::new();
    s.v.extend_from_slice(bytes);
    return s;
  }
}

impl<H: Hasher + Clone> Symbol for ByteArray<H> {
  fn zero(symbol_size: usize) -> Self {
    let mut s = ByteArray::<H>::new();
    s.v.reserve_exact(symbol_size);
    for _ in 0..symbol_size {
      s.v.push(0);
    }
    return s;
  }

  fn xor(&self, other: &Self) -> Self {
    if self.v.len() != other.v.len() {
      panic!();
    }
    let mut s = ByteArray::<H>::new();
    s.v.reserve_exact(self.v.len());
    for i in 0..self.v.len() {
      s.v.push(self.v[i] ^ other.v[i]);
    }
    return s;
  }

  fn hash(&self) -> u64 {
    let h = H::new();
    return h.hash(&self.v);
  }
}
