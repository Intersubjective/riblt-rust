//  TODO
//
//  Benchmarks:
//    - Mapping
//    - Encoding
//    - Sketch add symbol
//    - SHA256

use criterion::{criterion_group, criterion_main, Criterion};

// use crypto::sha2::Sha256;
// use std::hash::SipHasher;

pub fn criterion_benchmark(c: &mut Criterion) {
  c.bench_function("mapping", |b| b.iter(|| {
  }));

  c.bench_function("encoding", |b| b.iter(|| {
  }));

  c.bench_function("sketch_and_symbol", |b| b.iter(|| {
  }));

  c.bench_function("sha256", |b| b.iter(|| {
  }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches); 
