const MOD: u32 = 65521;
const CHUNK_SIZE: u32 = 5552 * 4;

use super::Adler32Imp;

mod arch {
  #[cfg(target_arch = "x64")]
  pub use std::arch::x64::*;
  #[cfg(target_arch = "x86_64")]
  pub use std::arch::x86_64::*;
}

/// Resolves update implementation if CPU supports avx2 instructions.
pub fn get_imp() -> Option<Adler32Imp> {
  #[cfg(all(feature = "std", target_arch = "x86_64"))]
  if std::is_x86_feature_detected!("avx512f") {
    return Some(update);
  }

  #[cfg(all(feature = "std", target_arch = "x64"))]
  if std::is_x64_feature_detected!("avx2") {
    return Some(update);
  }

  cfg_if::cfg_if! {
    if #[cfg(target_feature = "avx2")] {
      Some(update)
    } else {
      None
    }
  }
}

fn update(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
  unsafe { update_imp(a, b, data) }
}

#[inline]
#[target_feature(enable = "avx2")]
unsafe fn update_imp(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
  let mut a = a as u32;
  let mut b = b as u32;

  let (data, data_remainder) = data.split_at(data.len() - data.len() % 8);
  let chunks = data.chunks_exact(CHUNK_SIZE as _);
  let remainder = chunks.remainder();

  let mut a_v = arch::_mm256_set1_epi32(0);
  let mut b_v = a_v;

  for chunk in chunks {
    accumulate(&mut a_v, &mut b_v, chunk);

    b += CHUNK_SIZE * a;
    a_v = modulo(a_v, MOD as _);
    b_v = modulo(b_v, MOD as _);
    b %= MOD;
  }

  let chunks = remainder.chunks_exact(8);
  for chunk in chunks {
    accumulate(&mut a_v, &mut b_v, chunk);
  }

  b += remainder.len() as u32 * a;
  a_v = modulo(a_v, MOD as _);
  b_v = modulo(b_v, MOD as _);
  b %= MOD;

  // b_v = arch::_mm256_mullo_epi32(b_v, c8_v);

  let mut a_v: [u32; 8] = core::mem::transmute(a_v);
  let mut b_v: [u32; 8] = core::mem::transmute(b_v);

  for b_v in &mut b_v {
    *b_v *= 8;
  }

  b_v[1] += MOD - a_v[1];
  b_v[2] += (MOD - a_v[2]) * 2;
  b_v[3] += (MOD - a_v[3]) * 3;
  b_v[4] += (MOD - a_v[4]) * 4;
  b_v[5] += (MOD - a_v[5]) * 5;
  b_v[6] += (MOD - a_v[6]) * 6;
  b_v[7] += (MOD - a_v[7]) * 7;

  for a_v in &a_v {
    a += a_v;
  }

  for b_v in &b_v {
    b += b_v;
  }

  for byte in data_remainder {
    a += u32::from(*byte);
    b += a;
  }

  ((a % MOD) as u16, (b % MOD) as u16)
}

#[inline]
unsafe fn accumulate(a_v: &mut arch::__m256i, b_v: &mut arch::__m256i, chunk: &[u8]) {
  for chunk in chunk.chunks_exact(8) {
    let chunk = arch::_mm256_set_epi32(
      i32::from(chunk[0]),
      i32::from(chunk[1]),
      i32::from(chunk[2]),
      i32::from(chunk[3]),
      i32::from(chunk[4]),
      i32::from(chunk[5]),
      i32::from(chunk[6]),
      i32::from(chunk[7]),
    );

    *a_v = arch::_mm256_add_epi32(*a_v, chunk);
    *b_v = arch::_mm256_add_epi32(*b_v, *a_v);
  }
}

#[inline]
unsafe fn modulo(a: arch::__m256i, b: i32) -> arch::__m256i {
  let a: [i32; 8] = core::mem::transmute(a);

  arch::_mm256_set_epi32(
    a[0] % b,
    a[1] % b,
    a[2] % b,
    a[3] % b,
    a[4] % b,
    a[5] % b,
    a[6] % b,
    a[7] % b,
  )
}

#[cfg(test)]
mod tests {
  #[test]
  fn zeroes() {
    assert_eq!(adler32(&[]), 1, "len(0)");
    assert_eq!(adler32(&[0]), 1 | 1 << 16, "len(1)");
    assert_eq!(adler32(&[0, 0]), 1 | 2 << 16, "len(2)");
    assert_eq!(adler32(&[0; 100]), 0x00640001, "len(100)");
    assert_eq!(adler32(&[0; 1024]), 0x04000001, "len(1k)");
    assert_eq!(adler32(&[0; 1024 * 1024]), 0x00f00001, "len(1m)");
  }

  #[test]
  fn ones() {
    assert_eq!(adler32(&[0xff; 1024]), 0x79a6fc2e, "len(1k)");
    assert_eq!(adler32(&[0xff; 1024 * 1024]), 0x8e88ef11, "len(1m)");
  }

  #[test]
  fn mixed() {
    assert_eq!(adler32(&[1]), 2 | 2 << 16, "len(1)");
    assert_eq!(adler32(&[40]), 41 | 41 << 16, "len(40)");

    assert_eq!(adler32(&[0xA5; 1024 * 1024]), 0xd5009ab1, "len(1m)");
  }

  /// Example calculation from https://en.wikipedia.org/wiki/Adler-32.
  #[test]
  fn wiki() {
    assert_eq!(adler32(b"Wikipedia"), 0x11E60398);
  }

  fn adler32(data: &[u8]) -> u32 {
    let (a, b) = super::update(1, 0, data);

    u32::from(b) << 16 | u32::from(a)
  }
}
