use criterion::{
  black_box, criterion_group, criterion_main, measurement::Measurement, Bencher,
  Criterion, Throughput,
};
use rand::{thread_rng, RngCore};
use simd_adler32::imp::{self, Adler32Imp};

fn bench(c: &mut Criterion) {
  let mut random = vec![0; 1024 * 1024];

  thread_rng().fill_bytes(&mut random[..]);

  bench_group(c, "adler", &random, |a, b, data| {
    (adler::adler32_slice(data) as u16, 0)
  });

  bench_group(c, "adler32", &random, |a, b, data| {
    let mut adler = adler32::RollingAdler32::new();

    adler.update_buffer(data);
    (adler.hash() as u16, 0)
  });

  if let Some(update) = imp::avx2::get_imp() {
    bench_group(c, "simd-adler32", &random, update);
  }
}

fn bench_group(c: &mut Criterion, name: &str, data: &[u8], imp: Adler32Imp) {
  assert_eq!(data.len(), 1024 * 1024);

  c.benchmark_group(name)
    .throughput(Throughput::Bytes(100))
    .bench_with_input(format!("{}-100", name), &data[..100], |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    })
    //
    .throughput(Throughput::Bytes(1000))
    .bench_with_input(format!("{}-1k", name), &data[..1000], |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    })
    //
    .throughput(Throughput::Bytes(data.len() as _))
    .bench_with_input(format!("{}-1m", name), &data, |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
