use super::Adler32Imp;

#[cfg(target_feature = "neon")]
pub fn get_imp() -> Option<Adler32Imp> {
  Some(imp::update)
}

#[cfg(not(target_feature = "neon"))]
pub fn get_imp() -> Option<Adler32Imp> {
  None
}

#[cfg(target_feature = "neon")]
mod imp {
  const MOD: u32 = 65521;
  const NMAX: usize = 5552;
  const BLOCK_SIZE: usize = 32;
  const CHUNK_SIZE: usize = NMAX / BLOCK_SIZE * BLOCK_SIZE;

  #[cfg(target_arch = "aarch64")]
  use core::arch::aarch64::*;
  #[cfg(target_arch = "arm")]
  use core::arch::arm::*;

  pub fn update(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
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

  fn update_block(a: &mut u32, b: &mut u32, chunk: &[u8]) {
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

  fn update_chunk_block(a: &mut u32, b: &mut u32, chunk: &[u8]) {
    debug_assert_eq!(
      chunk.len(),
      CHUNK_SIZE,
      "Unexpected chunk size (expected {}, got {})",
      CHUNK_SIZE,
      chunk.len()
    );

    reduce_add_blocks(a, b, chunk);
  }

  fn reduce_add_blocks<'a>(a: &mut u32, b: &mut u32, chunk: &'a [u8]) -> &'a [u8] {
    if chunk.len() < BLOCK_SIZE {
      return chunk;
    }
    let blocks = chunk.chunks_exact(BLOCK_SIZE);
    let num_blocks = blocks.len() as u32;
    let blocks_remainder = blocks.remainder();

    #[inline(always)]
    fn reduce_add(v: uint32x4_t) -> u32 {
      #[cfg(target_arch = "aarch64")]
      unsafe {
        vaddvq_u32(v)
      }
      #[cfg(not(target_arch = "aarch64"))]
      unsafe {
        let sum = vadd_u32(vget_low_u32(v), vget_high_u32(v));
        vget_lane_u32(sum, 0).wrapping_add(vget_lane_u32(sum, 1))
      }
    }

    // Conversion of the code from Chromium zlib with extra optimizations.
    // https://chromium.googlesource.com/chromium/src/third_party/+/main/zlib/adler32_simd.c
    unsafe {
      // a_v accumulates the raw sum of input bytes.
      let mut a_v: uint32x4_t = vdupq_n_u32(0);
      // b_v accumulates the block-level prefix sum of a_v.
      let mut b_v: uint32x4_t = vdupq_n_u32(0);

      // Allocate 32 lanes to accumulate positional sums across all blocks.
      // Each lane corresponds an index in the block from 0-31.
      let mut v_column_sum_1: uint16x8_t = vdupq_n_u16(0);
      let mut v_column_sum_2: uint16x8_t = vdupq_n_u16(0);
      let mut v_column_sum_3: uint16x8_t = vdupq_n_u16(0);
      let mut v_column_sum_4: uint16x8_t = vdupq_n_u16(0);

      for block in blocks {
        let block_ptr = block.as_ptr();
        // Slurp in 32 bytes
        let bytes1: uint8x16_t = vld1q_u8(block_ptr);
        let bytes2: uint8x16_t = vld1q_u8(block_ptr.add(16));

        // Wrapping-add the sums from the previous block together.
        // b_v[i] += a_v[i]
        b_v = vaddq_u32(b_v, a_v);

        // Unsigned add, accumulate long pairwise.
        // Adjacent elements in bytes1 are zipped, added, lengthened.
        a_v = vpadalq_u16(a_v, vpadalq_u8(vpaddlq_u8(bytes1), bytes2));

        // Update positional sums for B with widening addition. Have to oscillate
        // between low and high elements, since vaddw's first argument is already
        // q-length.
        v_column_sum_1 = vaddw_u8(v_column_sum_1, vget_low_u8(bytes1));
        v_column_sum_2 = vaddw_u8(v_column_sum_2, vget_high_u8(bytes1));
        v_column_sum_3 = vaddw_u8(v_column_sum_3, vget_low_u8(bytes2));
        v_column_sum_4 = vaddw_u8(v_column_sum_4, vget_high_u8(bytes2));
      }

      // Then product-sum of each D column.
      let w12 = vld1q_u16([32, 31, 30, 29, 28, 27, 26, 25].as_ptr());
      let w34 = vld1q_u16([24, 23, 22, 21, 20, 19, 18, 17].as_ptr());
      let w56 = vld1q_u16([16, 15, 14, 13, 12, 11, 10, 9].as_ptr());
      let w78 = vld1q_u16([8, 7, 6, 5, 4, 3, 2, 1].as_ptr());
      // Stage 1: multiply and shift together the low halves.
      let b_final_1: uint32x4_t = vmlal_u16(
        vmlal_u16(
          vmlal_u16(
            vmlal_u16(
              vdupq_n_u32(0),
              vget_low_u16(v_column_sum_1),
              vget_low_u16(w12),
            ),
            vget_low_u16(v_column_sum_2),
            vget_low_u16(w34),
          ),
          vget_low_u16(v_column_sum_3),
          vget_low_u16(w56),
        ),
        vget_low_u16(v_column_sum_4),
        vget_low_u16(w78),
      );
      // Stage 2: multiply and accumulate the high halves.
      let b_final_2: uint32x4_t = vmlal_u16(
        vmlal_u16(
          vmlal_u16(
            vmlal_u16(
              vdupq_n_u32(0),
              vget_high_u16(v_column_sum_1),
              vget_high_u16(w12),
            ),
            vget_high_u16(v_column_sum_2),
            vget_high_u16(w34),
          ),
          vget_high_u16(v_column_sum_3),
          vget_high_u16(w56),
        ),
        vget_high_u16(v_column_sum_4),
        vget_high_u16(w78),
      );
      // Add low and high halves together.
      let b_final_v = vaddq_u32(b_final_1, b_final_2);

      let a_sum = reduce_add(a_v);
      let b_sum = reduce_add(vaddq_u32(
        // Multiply previous block values by 32x
        vshlq_n_u32(b_v, 5),
        b_final_v,
      ));

      let initial_a = *a;
      let initial_b = *b;

      *b = (initial_b
        .wrapping_add(initial_a.wrapping_mul(num_blocks).wrapping_mul(32))
        .wrapping_add(b_sum))
        % MOD;
      *a = (initial_a.wrapping_add(a_sum)) % MOD;

      blocks_remainder
    }
  }
}

#[cfg(test)]
mod tests {
  use rand::{rngs::SmallRng, Rng, SeedableRng};

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
    let mut random = [0; 1024 * 1024];
    SmallRng::from_entropy().fill(&mut random[..]);

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
    if let Some(update) = super::get_imp() {
      let (a, b) = update(1, 0, data);
      let left = u32::from(b) << 16 | u32::from(a);
      let right = adler::adler32_slice(data);

      assert_eq!(left, right, "len({})", data.len());
    }
  }
}
