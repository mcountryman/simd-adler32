const MOD: u32 = 65521;
const CHUNK_SIZE: usize = 5504; // 5552;

use super::Adler32Imp;

#[cfg(target_arch = "x84")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

/// Resolves update implementation if CPU supports avx2 instructions.
pub fn get_imp() -> Option<Adler32Imp> {
  #[cfg(all(feature = "std", target_arch = "x86"))]
  if std::is_x86_feature_detected!("avx2") {
    return Some(update);
  }

  #[cfg(all(feature = "std", target_arch = "x86_64"))]
  if std::is_x86_feature_detected!("avx2") {
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

  let chunks = data.chunks_exact(CHUNK_SIZE);
  let remainder = chunks.remainder();
  for chunk in chunks {
    update_chunk_block(&mut a, &mut b, chunk);
  }

  update_block(&mut a, &mut b, remainder);

  (a as u16, b as u16)
}

unsafe fn update_chunk_block(a: &mut u32, b: &mut u32, chunk: &[u8]) {
  debug_assert_eq!(
    chunk.len(),
    CHUNK_SIZE,
    "Unexpected chunk size (expected {}, got {})",
    CHUNK_SIZE,
    chunk.len()
  );

  reduce_add_blocks(a, b, chunk);

  *a %= MOD;
  *b %= MOD;
}

unsafe fn update_block(a: &mut u32, b: &mut u32, chunk: &[u8]) {
  debug_assert!(
    chunk.len() <= CHUNK_SIZE,
    "Unexpected chunk size (expected <= {}, got {})",
    CHUNK_SIZE,
    chunk.len()
  );

  for byte in reduce_add_blocks(a, b, chunk) {
    *a += *byte as u32;
    *b += *a;
  }

  if *a >= MOD {
    *a -= MOD;
  }

  *b %= MOD;
}

#[inline(always)]
unsafe fn reduce_add_blocks<'a>(a: &mut u32, b: &mut u32, chunk: &'a [u8]) -> &'a [u8] {
  if chunk.len() < 64 {
    return chunk;
  }

  let blocks = chunk.chunks_exact(64);
  let blocks_remainder = blocks.remainder();

  let n = CHUNK_SIZE / 64;
  let n = if n > blocks.len() {
    blocks.len() as u32
  } else {
    n as u32
  };

  let one_v = _mm256_set1_epi16(1);
  let zero_v = _mm256_set1_epi16(0);
  let weight_hi_v = get_weight_hi();
  let weight_lo_v = get_weight_lo();

  let mut p_v = _mm256_set_epi32(0, 0, 0, 0, 0, 0, 0, (*a * n) as _);
  let mut a_v = _mm256_set_epi32(0, 0, 0, 0, 0, 0, 0, 0);
  let mut b_v = _mm256_set_epi32(0, 0, 0, 0, 0, 0, 0, *b as _);

  for block in blocks {
    let block_ptr = block.as_ptr() as *const _;
    let left_v = _mm256_loadu_si256(block_ptr);
    let right_v = _mm256_loadu_si256(block_ptr.add(1));

    p_v = _mm256_add_epi32(p_v, a_v);

    a_v = _mm256_add_epi32(a_v, _mm256_sad_epu8(left_v, zero_v));
    let mad = _mm256_maddubs_epi16(left_v, weight_hi_v);
    b_v = _mm256_add_epi32(b_v, _mm256_madd_epi16(mad, one_v));

    a_v = _mm256_add_epi32(a_v, _mm256_sad_epu8(right_v, zero_v));
    let mad = _mm256_maddubs_epi16(right_v, weight_lo_v);
    b_v = _mm256_add_epi32(b_v, _mm256_madd_epi16(mad, one_v));
  }

  b_v = _mm256_add_epi32(b_v, _mm256_slli_epi32(p_v, 6));

  *a += reduce_add(a_v);
  *b = reduce_add(b_v);

  blocks_remainder
}

#[inline(always)]
unsafe fn reduce_add(v: __m256i) -> u32 {
  // print_m256i_u32("v", &v);
  let sum = _mm_add_epi32(_mm256_castsi256_si128(v), _mm256_extracti128_si256(v, 1));
  // print_m128i_u32("sum", &sum);
  let hi = _mm_unpackhi_epi64(sum, sum);
  // print_m128i_u32("hi", &hi);

  let sum = _mm_add_epi32(hi, sum);
  // print_m128i_u32("sum", &sum);
  let hi = _mm_shuffle_epi32(sum, _MM_SHUFFLE(2, 3, 0, 1));
  // print_m128i_u32("hi", &hi);

  let sum = _mm_add_epi32(sum, hi);
  // print_m128i_u32("sum", &sum);
  let sum = _mm_cvtsi128_si32(sum) as _;
  // println!("sum: {}", sum);

  sum
}

#[inline(always)]
unsafe fn get_weight_lo() -> __m256i {
  _mm256_set_epi8(
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31,
  )
}

#[inline(always)]
unsafe fn get_weight_hi() -> __m256i {
  _mm256_set_epi8(
    32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52,
    53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63,
  )
}

unsafe fn print_m256i_u32(s: &str, v: &__m256i) {
  let v: [u32; 8] = core::mem::transmute(*v);
  println!("{}: {:?}", s, v);
}

unsafe fn print_m128i_u16(s: &str, v: &__m128i) {
  let v: [u16; 8] = core::mem::transmute(*v);
  println!("{}: {:?}", s, v);
}

unsafe fn print_m128i_u32(s: &str, v: &__m128i) {
  let v: [u32; 4] = core::mem::transmute(*v);
  println!("{}: {:?}", s, v);
}

unsafe fn print_m128i_u64(s: &str, v: &__m128i) {
  let v: [u64; 2] = core::mem::transmute(*v);
  println!("{}: {:?}", s, v);
}

#[cfg(test)]
mod tests {
  use rand::Rng;

  #[test]
  fn zeroes() {
    assert_sum_eq(&[]);
    assert_sum_eq(&[0]);
    assert_sum_eq(&[0, 0]);
    assert_sum_eq(&[0; 100]);
    assert_sum_eq(&[0; 1024]);
    assert_sum_eq(&[0; 1024 * 1024]);
  }

  #[test]
  fn ones() {
    assert_sum_eq(&[]);
    assert_sum_eq(&[1]);
    assert_sum_eq(&[1, 1]);
    assert_sum_eq(&[1; 100]);
    assert_sum_eq(&[1; 1024]);
    assert_sum_eq(&[1; 1024 * 1024]);
  }

  #[test]
  fn random() {
    let mut random = vec![0; 1024 * 1024];
    rand::thread_rng().fill(&mut random[..]);

    assert_sum_eq(&random[..1]);
    assert_sum_eq(&random[..100]);
    assert_sum_eq(&random[..1024]);
    assert_sum_eq(&random[..1024 * 1024]);
  }

  /// Example calculation from https://en.wikipedia.org/wiki/Adler-32.
  #[test]
  fn wiki() {
    assert_sum_eq(b"Wikipedia");
  }

  fn assert_sum_eq(data: &[u8]) {
    let (a, b) = super::update(1, 0, data);
    let left = u32::from(b) << 16 | u32::from(a);
    let right = adler::adler32_slice(data);

    assert_eq!(left, right, "len({})", data.len());
  }
}
