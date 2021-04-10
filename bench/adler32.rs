use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::{thread_rng, RngCore};
use simd_adler32::imp::{self, Adler32Imp};

fn bench(c: &mut Criterion) {
  let ones = vec![1; 100_000];
  let zeros = vec![0; 100_000];
  let mut random = vec![0; 100_000];

  thread_rng().fill_bytes(&mut random[..]);

  if let Some(update) = imp::avx2::get_imp() {
    bench_group(c, "avx2-ones", &ones, update);
    bench_group(c, "avx2-zeros", &zeros, update);
    bench_group(c, "avx2-random", &random, update);
  }

  if let Some(update) = imp::ssse3::get_imp() {
    bench_group(c, "ssse3-ones", &ones, update);
    bench_group(c, "ssse3-zeros", &zeros, update);
    bench_group(c, "ssse3-random", &random, update);
  }

  if let Some(update) = imp::sse2::get_imp() {
    bench_group(c, "sse2-ones", &ones, update);
    bench_group(c, "sse2-zeros", &zeros, update);
    bench_group(c, "sse2-random", &random, update);
  }

  bench_group(c, "scalar-ones", &ones, imp::scalar::update);
  bench_group(c, "scalar-zeros", &zeros, imp::scalar::update);
  bench_group(c, "scalar-random", &random, imp::scalar::update);
}

fn bench_group(c: &mut Criterion, name: &str, data: &[u8], imp: Adler32Imp) {
  assert_eq!(data.len(), 100_000);

  c.benchmark_group(name)
    .throughput(Throughput::Bytes(10_000))
    .bench_with_input(format!("{}-10k", name), &data[..10_000], |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    })
    //
    .throughput(Throughput::Bytes(100_000))
    .bench_with_input(format!("{}-100k", name), &data[..100_000], |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
