//  NOTE
//  - Investigate static/dynamic dispatch in regard to
//    the performance when using traits like Symbol.
//  - Hash values are hardcoded to be u64, make it more generic.
//  - SipHasher is deprecated. Maybe replace it with a different hasher.

use std::vec::Vec;

pub mod sketch;
pub mod testing;

#[cfg(test)]
mod tests;

pub trait Symbol {
  fn zero() -> Self;
  fn xor(&self, other: &Self) -> Self;
  fn hash(&self) -> u64;
}

#[derive(Clone, Copy)]
pub enum Direction {
  ADD    = 1,
  REMOVE = -1,
}

#[derive(Clone, Copy)]
pub enum Error {
  InvalidDegree = 1,
  InvalidSize   = 2,
  DecodeFailed  = 3,
}


#[derive(Clone, Copy)]
pub struct SymbolMapping {
  source_idx : i64,
  coded_idx  : i64,
}

#[derive(Clone, Copy)]
pub struct RandomMapping {
  pub prng     : u64,
  pub last_idx : i64,
}

#[derive(Clone, Copy)]
pub struct HashedSymbol<T: Symbol + Copy> {
  pub symbol : T,
  pub hash   : u64,
}

#[derive(Clone, Copy)]
pub struct CodedSymbol<T: Symbol + Copy> {
  pub symbol : T,
  pub hash   : u64,
  pub count  : i64,
}

pub struct Encoder<T: Symbol + Copy> {
  pub symbols  : Vec<HashedSymbol<T>>,
      mappings : Vec<RandomMapping>,
      queue    : Vec<SymbolMapping>,
      next_idx : i64,
}

pub struct Decoder<T: Symbol + Copy> {
      coded       : Vec<CodedSymbol<T>>,
  pub local       : Encoder<T>,
  pub remote      : Encoder<T>,
      window      : Encoder<T>,
      decodable   : Vec<i64>,
      num_decoded : i64,
}

impl RandomMapping {
  pub fn next_index(&mut self) -> i64 {
    let r = self.prng.wrapping_mul(0xda942042e4dd58b5);
    self.prng = r;
    self.last_idx +=
      (((self.last_idx as f64) + 1.5) *
       (((1i64 << 32) as f64) / f64::sqrt((r as f64) + 1.0) - 1.0)
      ).ceil() as i64;
    return self.last_idx;
  }
}

impl<T: Symbol + Copy> CodedSymbol<T> {
  pub fn apply(&mut self, sym: &HashedSymbol<T>, direction: Direction) {
    self.symbol  = self.symbol.xor(&sym.symbol);
    self.hash   ^= sym.hash;
    self.count  += direction as i64;
  }
}

impl<T: Symbol + Copy> Encoder<T> {
  pub fn new() -> Self {
    return Encoder::<T> {
      symbols  : Vec::<HashedSymbol<T>>::new(),
      mappings : Vec::<RandomMapping>::new(),
      queue    : Vec::<SymbolMapping>::new(),
      next_idx : 0,
    };
  }


  pub fn reset(&mut self) {
    self.symbols.clear();
    self.mappings.clear();
    self.queue.clear();
    self.next_idx = 0;
  }

  pub fn add_hashed_symbol_with_mapping(&mut self, sym: &HashedSymbol<T>, mapp: &RandomMapping) {
    self.symbols.push(*sym);
    self.mappings.push(*mapp);

    self.queue.push(SymbolMapping {
      source_idx : (self.symbols.len() as i64) - 1,
      coded_idx  : mapp.last_idx,
    });

    //  Fix tail
    //
    let mut cur : usize = self.queue.len() - 1;
    while cur > 0 {
      let parent = (cur - 1) / 2;
      if cur == parent || self.queue[parent].coded_idx <= self.queue[cur].coded_idx {
        break;
      }
      self.queue.swap(parent, cur);
      cur = parent;
    }
  }

  pub fn add_hashed_symbol(&mut self, sym: &HashedSymbol<T>) {
    self.add_hashed_symbol_with_mapping(sym, &RandomMapping {
      prng     : sym.hash,
      last_idx : 0,
    });
  }

  pub fn add_symbol(&mut self, sym: &T) {
    self.add_hashed_symbol(&HashedSymbol::<T> {
      symbol : *sym,
      hash   : sym.hash(),
    });
  }

  pub fn apply_window(&mut self, sym: &CodedSymbol<T>, direction: Direction) -> CodedSymbol<T> {
    let mut next_sym = *sym;

    if self.queue.is_empty() {
      self.next_idx += 1;
      return next_sym;
    }

    while self.queue[0].coded_idx == self.next_idx {
      next_sym.apply(&self.symbols[self.queue[0].source_idx as usize], direction);
      self.queue[0].coded_idx = self.mappings[self.queue[0].source_idx as usize].next_index();

      //  Fix head
      //
      let mut cur : usize = 0;
      loop {
        let mut child = cur * 2 + 1;
        if child >= self.queue.len() {
          break;
        }
        let right_child = child + 1;
        if right_child < self.queue.len() && self.queue[right_child].coded_idx < self.queue[child].coded_idx {
          child = right_child;
        }
        if self.queue[cur].coded_idx <= self.queue[child].coded_idx {
          break;
        }
        self.queue.swap(cur, child);
        cur = child;
      }
    }

    self.next_idx += 1;
    return next_sym;
  }

  pub fn produce_next_coded_symbol(&mut self) -> CodedSymbol<T> {
    return self.apply_window(&CodedSymbol::<T> {
      symbol : T::zero(),
      hash   : 0,
      count  : 0,
    }, Direction::ADD);
  }
}

impl<T: Symbol + Copy> Decoder<T> {
  pub fn new() -> Self {
    return Decoder::<T> {
      coded       : Vec::<CodedSymbol<T>>::new(),
      local       : Encoder::<T>::new(),
      remote      : Encoder::<T>::new(),
      window      : Encoder::<T>::new(),
      decodable   : Vec::<i64>::new(),
      num_decoded : 0,
    };
  }

  pub fn reset(&mut self) {
    self.coded.clear();
    self.local.reset();
    self.remote.reset();
    self.window.reset();
    self.decodable.clear();
    self.num_decoded = 0;
  }

  pub fn add_symbol(&mut self, sym: &T) {
    self.window.add_hashed_symbol(&HashedSymbol::<T> {
      symbol : *sym,
      hash   : sym.hash(),
    });
  }

  pub fn add_coded_symbol(&mut self, sym: &CodedSymbol<T>) {
    let mut next_sym = self.window.apply_window(sym,       Direction::REMOVE);
    next_sym         = self.remote.apply_window(&next_sym, Direction::REMOVE);
    next_sym         = self.local .apply_window(&next_sym, Direction::ADD);

    self.coded.push(next_sym);

    if (    (next_sym.count == 1 || next_sym.count == -1)
         && (next_sym.hash == next_sym.symbol.hash())
       ) || (next_sym.count == 0 && next_sym.hash == 0) {
      self.decodable.push((self.coded.len() as i64) - 1);
    }
  }

  fn apply_new_symbol(&mut self, sym: &HashedSymbol<T>, direction: Direction) -> RandomMapping {
    let mut mapp = RandomMapping {
      prng:     sym.hash,
      last_idx: 0,
    };

    while mapp.last_idx < (self.coded.len() as i64) {
      let n = mapp.last_idx as usize;
      self.coded[n].apply(&sym, direction);

      if (self.coded[n].count == -1 || self.coded[n].count == 1) &&
         self.coded[n].hash == self.coded[n].symbol.hash() {
        self.decodable.push(n as i64);
      }

      mapp.next_index();
    }

    return mapp;
  }

  pub fn try_decode(&mut self) -> Result<(), Error> {
    let mut didx : usize = 0;

    // self.decodable.len() will increase in apply_new_symbol
    //
    while didx < self.decodable.len() {
      let cidx = self.decodable[didx] as usize;
      let sym  = self.coded[cidx];

      match sym.count {
        1 => {
          let new_sym = HashedSymbol::<T> {
            symbol : T::zero().xor(&sym.symbol),
            hash   : sym.hash
          };

          let mapp = self.apply_new_symbol(&new_sym, Direction::REMOVE);
          self.remote.add_hashed_symbol_with_mapping(&new_sym, &mapp);
          self.num_decoded += 1;
        },

        -1 => {
          let new_sym = HashedSymbol::<T> {
            symbol : T::zero().xor(&sym.symbol),
            hash   : sym.hash
          };

          let mapp = self.apply_new_symbol(&new_sym, Direction::ADD);
          self.local.add_hashed_symbol_with_mapping(&new_sym, &mapp);
          self.num_decoded += 1;
        },

        0 => {
          self.num_decoded += 1;
        },

        _ => {
          return Err(Error::InvalidDegree);
        }
      }

      didx += 1;
    }

    self.decodable.clear();

    return Ok(());        
  }

  pub fn decoded(&self) -> bool {
    return self.num_decoded == (self.coded.len() as i64);
  }

  pub fn get_remote_symbols(&self) -> Vec<HashedSymbol<T>> {
    return self.remote.symbols.clone();
  }

  pub fn get_local_symbols(&self) -> Vec<HashedSymbol<T>> {
    return self.local.symbols.clone();
  }
}
