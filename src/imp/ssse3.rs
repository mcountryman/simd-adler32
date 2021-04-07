const MOD: u32 = 65521;
const CHUNK_SIZE: u32 = 5552 * 4;

#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub fn update(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
  unsafe {
    let mut a = a as u32;
    let mut b = b as u32;

    let (data, data_remainder) = data.split_at(data.len() - data.len() % 4);
    let chunks = data.chunks_exact(CHUNK_SIZE as _);
    let remainder = chunks.remainder();

    let mut a_v = _mm_setzero_si128();
    let mut b_v = a_v;

    for chunk in chunks {
      accumulate(&mut a_v, &mut b_v, chunk);

      b += CHUNK_SIZE * a;
      a_v = _mm_mod_epi32(a_v, MOD as _);
      b_v = _mm_mod_epi32(b_v, MOD as _);
      b %= MOD;
    }

    let chunks = remainder.chunks_exact(4);
    for chunk in chunks {
      accumulate(&mut a_v, &mut b_v, chunk);
    }

    b += remainder.len() as u32 * a;
    a_v = _mm_mod_epi32(a_v, MOD as _);
    b_v = _mm_mod_epi32(b_v, MOD as _);
    b %= MOD;

    let mut a_v: [u32; 4] = core::mem::transmute(a_v);
    let mut b_v: [u32; 4] = core::mem::transmute(b_v);

    for b_v in &mut b_v {
      *b_v = *b_v * 4;
    }

    b_v[1] += MOD - a_v[1];
    b_v[2] += (MOD - a_v[2]) * 2;
    b_v[3] += (MOD - a_v[3]) * 3;

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
}

#[inline]
unsafe fn accumulate(a_v: &mut __m128i, b_v: &mut __m128i, chunk: &[u8]) {
  for chunk in chunk.chunks_exact(4) {
    let chunk = _mm_set_epi32(
      i32::from(chunk[0]),
      i32::from(chunk[1]),
      i32::from(chunk[2]),
      i32::from(chunk[3]),
    );

    *a_v = _mm_add_epi32(*a_v, chunk);
    *b_v = _mm_add_epi32(*b_v, *a_v);
  }
}

#[inline]
unsafe fn _mm_mod_epi32(a: __m128i, b: i32) -> __m128i {
  let a: [i32; 4] = core::mem::transmute(a);
  _mm_set_epi32(a[0] % b, a[1] % b, a[2] % b, a[3] % b)
}

#[cfg(test)]
mod tests {
  #[test]
  fn zeroes() {
    assert_eq!(adler32(&[]), 1);
    assert_eq!(adler32(&[0]), 1 | 1 << 16);
    assert_eq!(adler32(&[0, 0]), 1 | 2 << 16);
    assert_eq!(adler32(&[0; 100]), 0x00640001);
    assert_eq!(adler32(&[0; 1024]), 0x04000001);
    assert_eq!(adler32(&[0; 1024 * 1024]), 0x00f00001);
  }

  #[test]
  fn ones() {
    assert_eq!(adler32(&[0xff; 1024]), 0x79a6fc2e);
    assert_eq!(adler32(&[0xff; 1024 * 1024]), 0x8e88ef11);
  }

  #[test]
  fn mixed() {
    assert_eq!(adler32(&[1]), 2 | 2 << 16);
    assert_eq!(adler32(&[40]), 41 | 41 << 16);

    assert_eq!(adler32(&[0xA5; 1024 * 1024]), 0xd5009ab1);
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
