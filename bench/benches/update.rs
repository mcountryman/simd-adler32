use criterion::{measurement::Measurement, *};
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
use simd_adler32::arch::aarch64;
use simd_adler32::arch::scalar;
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
use simd_adler32::arch::wasm;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use simd_adler32::arch::x86_64;
use std::hint::black_box;

fn run(c: &mut Criterion) {
  bench_group(c, "1MiB", 1024 * 1024);
  bench_group(c, "1KiB", 1024);
}

fn bench_group(c: &mut Criterion, name: &str, size: usize) {
  let input = random(size);
  let mut group = c.benchmark_group(name);

  group.throughput(criterion::Throughput::Bytes(input.len() as _));

  group.bench_with_input("adler2", &input, adler2);
  group.bench_with_input("scalar", &input, update(Some(scalar::update)));

  #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
  {
    #[cfg(target_feature = "neon")]
    group.bench_with_input("neon", &input, update(aarch64::neon::get()));
  }

  #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
  {
    #[cfg(target_feature = "simd128")]
    group.bench_with_input("simd128", &input, update(wasm::simd128::get()));
  }

  #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
  {
    #[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
    group.bench_with_input("avx512", &input, update(x86_64::avx512::get()));
    #[cfg(target_feature = "avx2")]
    group.bench_with_input("avx2", &input, update(x86_64::avx2::get()));
    #[cfg(target_feature = "ssse3")]
    group.bench_with_input("ssse3", &input, update(x86_64::ssse3::get()));
    #[cfg(target_feature = "sse2")]
    group.bench_with_input("sse2", &input, update(x86_64::sse2::get()));
  }

  group.finish();
}

type Update = fn(u16, u16, &[u8]) -> (u16, u16);

fn adler2<M: Measurement>(b: &mut Bencher<'_, M>, bytes: &Vec<u8>) {
  b.iter(|| black_box(adler2::adler32_slice(bytes)))
}

fn update<M>(update: Option<Update>) -> impl FnMut(&mut Bencher<'_, M>, &Vec<u8>)
where
  M: Measurement,
{
  let update = update.unwrap();

  move |b: &mut Bencher<'_, M>, bytes: &Vec<u8>| b.iter(|| black_box(update(1, 0, bytes)))
}

fn random(size: usize) -> Vec<u8> {
  let mut bytes = vec![0; size];

  fastrand::fill(&mut bytes);
  bytes
}

criterion_group!(benches, run);
criterion_main!(benches);
