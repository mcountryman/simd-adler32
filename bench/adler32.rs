use criterion::{
  black_box, criterion_group, criterion_main, measurement::Measurement, Bencher,
  Criterion, Throughput,
};
use rand::{thread_rng, RngCore};
use simd_adler32::imp::{self, Adler32Imp};

fn bench(c: &mut Criterion) {
  let ones = [1; 1024 * 1024];
  let zeros = [0; 1024 * 1024];
  let mut random = [0; 1024 * 1024];

  thread_rng().fill_bytes(&mut random[..]);

  bench_group(c, "scalar-ones", ones, imp::scalar::update);
  bench_group(c, "scalar-zeros", zeros, imp::scalar::update);
  bench_group(c, "scalar-random", random, imp::scalar::update);

  bench_group(c, "ssse3-ones", ones, imp::ssse3::update);
  bench_group(c, "ssse3-zeros", zeros, imp::ssse3::update);
  bench_group(c, "ssse3-random", random, imp::ssse3::update);
}

fn bench_group(c: &mut Criterion, name: &str, data: [u8; 1024 * 1024], imp: Adler32Imp) {
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
    .bench_with_input(format!("{}-1m", name), &data[..], |b, data| {
      b.iter(|| black_box(imp(1, 0, data)))
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
