mod util;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, RngCore};
use simd_adler32::imp;

pub fn bench(c: &mut Criterion) {
  let ones = [1; 100_000];
  let zeros = [0; 100_000];
  let mut random = [0; 100_000];

  thread_rng().fill_bytes(&mut random[..]);

  if let Some(update) = imp::avx2::get_imp() {
    util::bench_group(c, "avx2", "ones", &ones, update);
    util::bench_group(c, "avx2", "zeros", &zeros, update);
    util::bench_group(c, "avx2", "random", &random, update);
  }
}

criterion_group!(benches, bench);
criterion_main!(benches);
