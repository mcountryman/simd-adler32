mod avx2;
mod avx512;
mod scalar;
mod sse2;
mod ssse3;

use criterion::{criterion_main, criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
  avx2::bench(c);
  avx512::bench(c);
  scalar::bench(c);
  sse2::bench(c);
  ssse3::bench(c);
}

criterion_group!(benches, bench);
criterion_main!(benches);
