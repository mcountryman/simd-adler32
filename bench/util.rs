use criterion::{black_box, Criterion, Throughput};
use simd_adler32::imp::Adler32Imp;

pub fn bench_group(
  c: &mut Criterion,
  group: &str,
  name: &str,
  data: &[u8],
  imp: Adler32Imp,
) {
  assert_eq!(data.len(), 100_000);

  c.benchmark_group(group)
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
