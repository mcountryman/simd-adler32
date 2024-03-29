use crate::update::Adler32Update;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

const MOD: u32 = 65521;
const NMAX: usize = 5552;
const BLOCK_SIZE: usize = 32;
const CHUNK_SIZE: usize = NMAX / BLOCK_SIZE * BLOCK_SIZE;

pub fn get_update_if_supported() -> Option<Adler32Update> {
  if super::is_x86_feature_detected!("sse2") {
    fn stub(a: u16, b: u16, bytes: &[u8]) -> (u16, u16) {
      unsafe { update(a, b, bytes) }
    }

    Some(stub)
  } else {
    None
  }
}

#[inline]
#[target_feature(enable = "sse2")]
pub unsafe fn update(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
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

  *a %= MOD;
  *b %= MOD;
}

#[inline(always)]
unsafe fn reduce_add_blocks<'a>(a: &mut u32, b: &mut u32, chunk: &'a [u8]) -> &'a [u8] {
  if chunk.len() < BLOCK_SIZE {
    return chunk;
  }

  let blocks = chunk.chunks_exact(BLOCK_SIZE);
  let blocks_remainder = blocks.remainder();

  let zero_v = _mm_setzero_si128();
  let weight_hi_v = get_weight_hi();
  let weight_lo_v = get_weight_lo();

  let mut p_v = _mm_set_epi32(0, 0, 0, (*a * blocks.len() as u32) as _);
  let mut a_v = _mm_setzero_si128();
  let mut b_v = _mm_set_epi32(0, 0, 0, *b as _);

  for block in blocks {
    let block_ptr = block.as_ptr() as *const _;
    let left_v = _mm_loadu_si128(block_ptr);
    let right_v = _mm_loadu_si128(block_ptr.add(1));

    p_v = _mm_add_epi32(p_v, a_v);

    a_v = _mm_add_epi32(a_v, _mm_sad_epu8(left_v, zero_v));
    let mad = maddubs(left_v, weight_hi_v);
    b_v = _mm_add_epi32(b_v, mad);

    a_v = _mm_add_epi32(a_v, _mm_sad_epu8(right_v, zero_v));
    let mad = maddubs(right_v, weight_lo_v);
    b_v = _mm_add_epi32(b_v, mad);
  }

  b_v = _mm_add_epi32(b_v, _mm_slli_epi32(p_v, 5));

  *a += reduce_add(a_v);
  *b = reduce_add(b_v);

  blocks_remainder
}

#[inline(always)]
unsafe fn maddubs(a: __m128i, b: __m128i) -> __m128i {
  let a_lo = _mm_unpacklo_epi8(a, _mm_setzero_si128());
  let a_hi = _mm_unpackhi_epi8(a, _mm_setzero_si128());

  let b_lo = _mm_unpacklo_epi8(b, _mm_setzero_si128());
  let b_hi = _mm_unpackhi_epi8(b, _mm_setzero_si128());

  let lo = _mm_madd_epi16(a_lo, b_lo);
  let hi = _mm_madd_epi16(a_hi, b_hi);

  _mm_add_epi32(lo, hi)
}

#[inline(always)]
unsafe fn reduce_add(v: __m128i) -> u32 {
  let hi = _mm_unpackhi_epi64(v, v);
  let sum = _mm_add_epi32(hi, v);
  let hi = _mm_shuffle_epi32(sum, super::_mm_shuffle(2, 3, 0, 1));

  let sum = _mm_add_epi32(sum, hi);

  _mm_cvtsi128_si32(sum) as _
}

#[inline(always)]
unsafe fn get_weight_lo() -> __m128i {
  _mm_set_epi8(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16)
}

#[inline(always)]
unsafe fn get_weight_hi() -> __m128i {
  _mm_set_epi8(
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
  )
}
