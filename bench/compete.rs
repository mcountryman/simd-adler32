use criterion::{
  criterion_group, criterion_main, measurement::Measurement, Bencher, Criterion,
  Throughput,
};
use rand::{thread_rng, RngCore};

pub fn bench(c: &mut Criterion) {
  let mut random = vec![0; 100_000];
  thread_rng().fill_bytes(&mut random[..]);

  c.benchmark_group("compete/10k")
    .throughput(Throughput::Bytes(10_000))
    .bench_with_input("wuffs", &random[..10_000], bench_wuffs)
    .bench_with_input("adler", &random[..10_000], bench_adler)
    .bench_with_input("adler32", &random[..10_000], bench_adler32)
    .bench_with_input("simd-adler32", &random[..10_000], bench_simd_adler32);

  c.benchmark_group("compete/100k")
    .throughput(Throughput::Bytes(100_000))
    .bench_with_input("wuffs", &random[..100_000], bench_wuffs)
    .bench_with_input("adler", &random[..100_000], bench_adler)
    .bench_with_input("adler32", &random[..100_000], bench_adler32)
    .bench_with_input("simd-adler32", &random[..100_000], bench_simd_adler32);
}

fn bench_wuffs<'a, M>(b: &mut Bencher<'a, M>, data: &[u8])
where
  M: Measurement,
{
  b.iter(|| {
    let mut adler = wuffs::std::hash::adler32::WuffsAdler32::new().unwrap();

    adler.update(data);
  })
}

fn bench_adler<'a, M>(b: &mut Bencher<'a, M>, data: &[u8])
where
  M: Measurement,
{
  b.iter(|| {
    let mut adler = adler::Adler32::new();

    adler.write_slice(data);
    adler.checksum()
  })
}

fn bench_adler32<'a, M>(b: &mut Bencher<'a, M>, data: &[u8])
where
  M: Measurement,
{
  b.iter(|| {
    let mut adler = adler32::RollingAdler32::new();

    adler.update_buffer(data);
    adler.hash()
  })
}

fn bench_simd_adler32<'a, M>(b: &mut Bencher<'a, M>, data: &[u8])
where
  M: Measurement,
{
  b.iter(|| {
    let mut adler = simd_adler32::Adler32::new();

    adler.write(data);
    adler.finish()
  })
}

criterion_group!(benches, bench);
criterion_main!(benches);
