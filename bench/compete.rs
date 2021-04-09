use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use rand::{thread_rng, RngCore};

fn bench(c: &mut Criterion) {
  let mut random = vec![0; 100_000];
  thread_rng().fill_bytes(&mut random[..]);

  c.benchmark_group("10k")
    .throughput(Throughput::Bytes(10_000))
    .bench_with_input("adler", &random[..10_000], |b, data| {
      b.iter(|| {
        let mut adler = adler::Adler32::new();

        adler.write_slice(data);
        adler.checksum()
      })
    })
    .bench_with_input("adler32", &random[..10_000], |b, data| {
      b.iter(|| {
        let mut adler = adler32::RollingAdler32::new();

        adler.update_buffer(data);
        adler.hash()
      })
    })
    .bench_with_input("simd-adler32", &random[..10_000], |b, data| {
      b.iter(|| {
        let mut adler = simd_adler32::Adler32::new();

        adler.write(data);
        adler.finish()
      })
    });

  c.benchmark_group("100k")
    .throughput(Throughput::Bytes(100_000))
    .bench_with_input("adler", &random[..100_000], |b, data| {
      b.iter(|| {
        let mut adler = adler::Adler32::new();

        adler.write_slice(data);
        adler.checksum()
      })
    })
    .bench_with_input("adler32", &random[..100_000], |b, data| {
      b.iter(|| {
        let mut adler = adler32::RollingAdler32::new();

        adler.update_buffer(data);
        adler.hash()
      })
    })
    .bench_with_input("simd-adler32", &random[..100_000], |b, data| {
      b.iter(|| {
        let mut adler = simd_adler32::Adler32::new();

        adler.write(data);
        adler.finish()
      })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
