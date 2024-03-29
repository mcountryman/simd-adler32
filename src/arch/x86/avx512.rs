use crate::update::Adler32Update;

pub fn get_update_if_supported() -> Option<Adler32Update> {
  let has_avx512f = super::is_x86_feature_detected!("avx512f");
  let has_avx512bw = super::is_x86_feature_detected!("avx512bw");

  if has_avx512f && has_avx512bw {
    // FIXME: What if runtime support but dev forgot to compile with feature flags?
    fn stub(a: u16, b: u16, bytes: &[u8]) -> (u16, u16) {
      unsafe { update(a, b, bytes) }
    }

    Some(stub)
  } else {
    None
  }
}

#[inline]
pub unsafe fn update(a: u16, b: u16, bytes: &[u8]) -> (u16, u16) {
  imp::update(a, b, bytes)
}

#[cfg(not(all(feature = "nightly", any(target_arch = "x86", target_arch = "x86_64"))))]
mod imp {
  pub unsafe fn update(_: u16, _: u16, _: &[u8]) -> (u16, u16) {
    panic!("Target platform does not support `avx512f` and or `avx512bw`")
  }
}

#[cfg(all(feature = "nightly", any(target_arch = "x86", target_arch = "x86_64")))]
mod imp {
  #[cfg(target_arch = "x86")]
  use core::arch::x86::*;
  #[cfg(target_arch = "x86_64")]
  use core::arch::x86_64::*;

  const MOD: u32 = 65521;
  const NMAX: usize = 5552;
  const BLOCK_SIZE: usize = 64;
  const CHUNK_SIZE: usize = NMAX / BLOCK_SIZE * BLOCK_SIZE;

  #[inline]
  #[target_feature(enable = "avx512f")]
  #[target_feature(enable = "avx512bw")]
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

  #[inline]
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

  #[inline]
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

    let one_v = _mm512_set1_epi16(1);
    let zero_v = _mm512_setzero_si512();
    let weights = get_weights();

    let p_v = (*a * blocks.len() as u32) as _;
    let mut p_v = _mm512_set_epi32(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, p_v);
    let mut a_v = _mm512_setzero_si512();
    let mut b_v = _mm512_set_epi32(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, *b as _);

    for block in blocks {
      let block_ptr = block.as_ptr() as *const _;
      let block = _mm512_loadu_si512(block_ptr);

      p_v = _mm512_add_epi32(p_v, a_v);

      a_v = _mm512_add_epi32(a_v, _mm512_sad_epu8(block, zero_v));
      let mad = _mm512_maddubs_epi16(block, weights);
      b_v = _mm512_add_epi32(b_v, _mm512_madd_epi16(mad, one_v));
    }

    b_v = _mm512_add_epi32(b_v, _mm512_slli_epi32(p_v, 6));

    *a += reduce_add(a_v);
    *b = reduce_add(b_v);

    blocks_remainder
  }

  #[inline(always)]
  unsafe fn reduce_add(v: __m512i) -> u32 {
    let v: [__m256i; 2] = core::mem::transmute(v);

    reduce_add_256(v[0]) + reduce_add_256(v[1])
  }

  #[inline(always)]
  unsafe fn reduce_add_256(v: __m256i) -> u32 {
    let v: [__m128i; 2] = core::mem::transmute(v);
    let sum = _mm_add_epi32(v[0], v[1]);
    let hi = _mm_unpackhi_epi64(sum, sum);

    let sum = _mm_add_epi32(hi, sum);
    let hi = _mm_shuffle_epi32(sum, crate::x86::_mm_shuffle(2, 3, 0, 1));

    let sum = _mm_add_epi32(sum, hi);
    let sum = _mm_cvtsi128_si32(sum) as _;

    sum
  }

  #[inline(always)]
  unsafe fn get_weights() -> __m512i {
    _mm512_set_epi8(
      1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
      24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
      45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
    )
  }
}
