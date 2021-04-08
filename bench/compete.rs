use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::{thread_rng, RngCore};

fn bench(c: &mut Criterion) {
  let mut random = vec![0; 100_000];
  thread_rng().fill_bytes(&mut random[..]);

  bench_group(c, "adler", &random, |data| {
    let mut adler = adler::Adler32::new();

    adler.write_slice(data);
    adler.checksum()
  });

  bench_group(c, "adler32", &random, |data| {
    let mut adler = adler32::RollingAdler32::new();

    adler.update_buffer(data);
    adler.hash()
  });

  bench_group(c, "simd-adler32", &random, |data| {
    let mut adler = simd_adler32::Adler32::new();

    adler.update(data);
    adler.finalize()
  });
}

fn bench_group<F>(c: &mut Criterion, name: &str, data: &[u8], imp: F)
where
  F: Copy + FnOnce(&[u8]) -> u32,
{
  assert_eq!(data.len(), 100_000);

  c.benchmark_group(name)
    .throughput(Throughput::Bytes(10_000))
    .bench_with_input("10k", &data[..10_000], |b, data| {
      b.iter(|| black_box(imp(data)))
    })
    //
    .throughput(Throughput::Bytes(data.len() as _))
    .bench_with_input("100k", &data[..100_000], |b, data| {
      b.iter(|| black_box(imp(data)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
