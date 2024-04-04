use super::*;
use std::marker::PhantomData;

pub trait Hasher {
  fn new() -> Self;
  fn hash(&self, bytes: &[u8]) -> u64;
}

#[derive(Copy, Clone)]
pub struct ByteArray<const SIZE: usize, H: Hasher + Clone> {
  pub v           : [u8; SIZE],
      hasher_type : PhantomData<H>,
} 

impl<const SIZE: usize, H: Hasher + Clone> ByteArray<SIZE, H> {
  pub fn new(bytes: [u8; SIZE]) -> Self {
    return ByteArray::<SIZE, H> {
      v:           bytes,
      hasher_type: PhantomData::<H> {},
    };
  }
}

impl<const SIZE: usize, H: Hasher + Clone> Symbol for ByteArray<SIZE, H> {
  fn zero() -> Self {
    return ByteArray::<SIZE, H>::new(core::array::from_fn::<u8, SIZE, _>(|_| 0));
  }

  fn xor(&self, other: &Self) -> Self where Self: Sized {
    return ByteArray::<SIZE, H>::new(core::array::from_fn::<u8, SIZE, _>(|i| self.v[i] ^ other.v[i]));
  }

  fn hash(&self) -> u64 {
    let h = H::new();
    return h.hash(&self.v);
  }
}
