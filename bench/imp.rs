//! Module includes benchmarks for individual adler32 implementations when CPU support
//! is detected.
//!
//! Each implementation will be benchmarked on the following inputs.
//!
//! * 10k ones
//! * 10k zeros
//! * 10k random
//! * 100k ones
//! * 100k zeros
//! * 100k random

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::{thread_rng, RngCore};
use simd_adler32::imp::{self, Adler32Imp};

pub fn bench(c: &mut Criterion) {
  let ones = [1; 100_000];
  let zeros = [0; 100_000];
  let mut random = [0; 100_000];

  thread_rng().fill_bytes(&mut random[..]);

  bench_group(c, "scalar", "ones", &ones, imp::scalar::update);
  bench_group(c, "scalar", "zeros", &zeros, imp::scalar::update);
  bench_group(c, "scalar", "random", &random, imp::scalar::update);

  if let Some(update) = imp::ssse3::get_imp() {
    bench_group(c, "ssse3", "ones", &ones, update);
    bench_group(c, "ssse3", "zeros", &zeros, update);
    bench_group(c, "ssse3", "random", &random, update);
  }

  if let Some(update) = imp::sse2::get_imp() {
    bench_group(c, "sse2", "ones", &ones, update);
    bench_group(c, "sse2", "zeros", &zeros, update);
    bench_group(c, "sse2", "random", &random, update);
  }

  if let Some(update) = imp::avx2::get_imp() {
    bench_group(c, "avx2", "ones", &ones, update);
    bench_group(c, "avx2", "zeros", &zeros, update);
    bench_group(c, "avx2", "random", &random, update);
  }

  if let Some(update) = imp::avx512::get_imp() {
    bench_group(c, "avx512", "ones", &ones, update);
    bench_group(c, "avx512", "zeros", &zeros, update);
    bench_group(c, "avx512", "random", &random, update);
  }
}

pub fn bench_group(
  c: &mut Criterion,
  group: &str,
  name: &str,
  data: &[u8],
  imp: Adler32Imp,
) {
  assert_eq!(data.len(), 100_000);

  c.benchmark_group(format!("imp/{}", group))
    // 10k benchmark (takes first 10k of 100k input)
    .throughput(Throughput::Bytes(10_000))
    .bench_with_input(format!("{}-10k", name), &data[..10_000], |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    })
    // 100k benchmark
    .throughput(Throughput::Bytes(100_000))
    .bench_with_input(format!("{}-100k", name), &data[..100_000], |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
