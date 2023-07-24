use criterion::{
  black_box, criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup,
  Criterion, Throughput,
};
use rand::{thread_rng, RngCore};

pub fn bench(c: &mut Criterion) {
  let mut data = vec![0; 100_000];
  let mut group = c.benchmark_group("alts");

  thread_rng().fill_bytes(&mut data[..]);

  bench_alt(&mut group, "adler", &data, |data| {
    let mut adler = adler::Adler32::new();

    adler.write_slice(data);
    adler.checksum()
  });

  bench_alt(&mut group, "adler32", &data, |data| {
    let mut adler = adler32::RollingAdler32::new();

    adler.update_buffer(data);
    adler.hash()
  });

  bench_alt(&mut group, "simd-adler32", &data, |data| {
    let mut adler = simd_adler32::Adler32::new();

    adler.write(data);
    adler.finish()
  });
}

fn bench_alt<M, F>(g: &mut BenchmarkGroup<M>, name: &str, data: &[u8], mut imp: F)
where
  M: Measurement,
  F: FnMut(&[u8]) -> u32,
{
  g.throughput(Throughput::Bytes(10)).bench_with_input(
    format!("{}-10b", name),
    &data[..10],
    |b, data| b.iter(|| black_box(imp(data))),
  );

  g.throughput(Throughput::Bytes(10_000)).bench_with_input(
    format!("{}-10k", name),
    &data[..10_000],
    |b, data| b.iter(|| black_box(imp(data))),
  );

  g.throughput(Throughput::Bytes(100_000)).bench_with_input(
    format!("{}-100k", name),
    &data[..100_000],
    |b, data| b.iter(|| black_box(imp(data))),
  );
}

criterion_group!(benches, bench);
criterion_main!(benches);
