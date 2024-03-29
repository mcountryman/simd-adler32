use criterion::{
  black_box, criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup,
  Criterion, Throughput,
};
use rand::{thread_rng, RngCore};
use simd_adler32::{arch::*, update::Adler32Update};

pub fn bench(c: &mut Criterion) {
  let mut data = [1; 100_000];
  let mut group = c.benchmark_group("variants");

  thread_rng().fill_bytes(&mut data[..]);

  cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {

      if let Some(update) = x86::avx512::get_update_if_supported() {
        bench_variant(&mut group, "avx512", &data, update);
      }

      if let Some(update) = x86::avx2::get_update_if_supported() {
        bench_variant(&mut group, "avx2", &data, update);
      }

      if let Some(update) = x86::ssse3::get_update_if_supported() {
        bench_variant(&mut group, "ssse3", &data, update);
      }

      if let Some(update) = x86::sse2::get_update_if_supported() {
        bench_variant(&mut group, "sse2", &data, update);
      }

    } else if #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))] {

      if let Some(update) = wasm::get_update_if_supported() {
        bench_variant(&mut group, "wasm", &data, update);
      }

    }
  }

  bench_variant(&mut group, "scalar", &data, scalar::update);
}

fn bench_variant<M>(
  g: &mut BenchmarkGroup<M>,
  name: &str,
  data: &[u8],
  update: Adler32Update,
) where
  M: Measurement,
{
  g.throughput(Throughput::Bytes(10)).bench_with_input(
    format!("{}-10b", name),
    &data[..10],
    |b, data| b.iter(|| black_box(update(1, 0, data))),
  );

  g.throughput(Throughput::Bytes(10_000)).bench_with_input(
    format!("{}-10k", name),
    &data[..10_000],
    |b, data| b.iter(|| black_box(update(1, 0, data))),
  );

  g.throughput(Throughput::Bytes(100_000)).bench_with_input(
    format!("{}-100k", name),
    &data[..100_000],
    |b, data| b.iter(|| black_box(update(1, 0, data))),
  );
}

criterion_group!(benches, bench);
criterion_main!(benches);
