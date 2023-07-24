use criterion::{
  black_box, criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup,
  Criterion, Throughput,
};
use rand::{thread_rng, RngCore};
use simd_adler32::imp::{avx2, avx512, scalar, sse2, ssse3, wasm, Adler32Imp};

pub fn bench(c: &mut Criterion) {
  let mut data = [0; 100_000];
  let mut group = c.benchmark_group("variants");

  thread_rng().fill_bytes(&mut data[..]);

  if let Some(update) = avx512::get_imp() {
    bench_variant(&mut group, "avx512", &data, update);
  }

  if let Some(update) = avx2::get_imp() {
    bench_variant(&mut group, "avx2", &data, update);
  }

  if let Some(update) = ssse3::get_imp() {
    bench_variant(&mut group, "ssse3", &data, update);
  }

  if let Some(update) = sse2::get_imp() {
    bench_variant(&mut group, "sse2", &data, update);
  }

  if let Some(update) = wasm::get_imp() {
    bench_variant(&mut group, "wasm", &data, update);
  }

  bench_variant(&mut group, "scalar", &data, scalar::update);
}

fn bench_variant<M>(g: &mut BenchmarkGroup<M>, name: &str, data: &[u8], imp: Adler32Imp)
where
  M: Measurement,
{
  g.throughput(Throughput::Bytes(10)).bench_with_input(
    format!("{}-10b", name),
    &data[..10],
    |b, data| b.iter(|| black_box(imp(1, 0, data))),
  );

  g.throughput(Throughput::Bytes(10_000)).bench_with_input(
    format!("{}-10k", name),
    &data[..10_000],
    |b, data| b.iter(|| black_box(imp(1, 0, data))),
  );

  g.throughput(Throughput::Bytes(100_000)).bench_with_input(
    format!("{}-100k", name),
    &data[..100_000],
    |b, data| b.iter(|| black_box(imp(1, 0, data))),
  );
}

criterion_group!(benches, bench);
criterion_main!(benches);
