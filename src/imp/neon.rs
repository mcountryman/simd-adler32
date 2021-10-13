use super::Adler32Imp;

/// Resolves update implementation if CPU supports avx512f and avx512bw instructions.
pub fn get_imp() -> Option<Adler32Imp> {
  get_imp_inner()
}

#[inline]
#[cfg(all(
  feature = "std",
  feature = "nightly",
  target_arch = "arm",
  target_feature = "v7"
))]
fn get_imp_inner() -> Option<Adler32Imp> {
  if std::is_arm_feature_detected!("neon") {
    Some(imp::update)
  } else {
    None
  }
}

#[inline]
#[cfg(all(feature = "std", target_arch = "aarch64"))]
fn get_imp_inner() -> Option<Adler32Imp> {
  if std::is_aarch64_feature_detected!("neon") {
    Some(imp::update)
  } else {
    None
  }
}

#[inline]
#[cfg(all(
  target_feature = "neon",
  not(all(
    feature = "std",
    any(
      all(feature = "nightly", target_arch = "arm", target_feature = "v7"),
      target_arch = "aarch64"
    )
  ))
))]
fn get_imp_inner() -> Option<Adler32Imp> {
  Some(imp::update)
}

#[inline]
#[cfg(all(
  not(target_feature = "neon"),
  not(all(
    feature = "std",
    any(
      all(feature = "nightly", target_arch = "arm", target_feature = "v7"),
      target_arch = "aarch64"
    )
  ))
))]
fn get_imp_inner() -> Option<Adler32Imp> {
  None
}

#[cfg(all(
  any(
    all(feature = "nightly", target_arch = "arm", target_feature = "v7"),
    target_arch = "aarch64"
  ),
  any(feature = "std", target_feature = "neon")
))]
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
    unsafe { update_imp(a, b, data) }
  }

  #[inline]
  #[cfg_attr(target_arch = "arm", target_feature(enable = "v7"))]
  #[target_feature(enable = "neon")]
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

    let weight_hi_v = get_weight_hi();
    let weight_lo_v = get_weight_lo();

    let mut p_v: uint32x4_t = core::mem::transmute([*a * blocks.len() as u32, 0, 0, 0]);
    let mut a_v: uint32x4_t = core::mem::transmute([0u32, 0, 0, 0]);
    let mut b_v: uint32x4_t = core::mem::transmute([*b, 0, 0, 0]);

    for block in blocks {
      let block_ptr = block.as_ptr() as *const uint8x16_t;
      let v_lo = core::ptr::read_unaligned(block_ptr);
      let v_hi = core::ptr::read_unaligned(block_ptr.add(1));

      p_v = vaddq_u32(p_v, a_v);

      a_v = vaddq_u32(a_v, vqaddlq_u8(v_lo));
      b_v = vdotq_u32(b_v, v_lo, weight_lo_v);

      a_v = vaddq_u32(a_v, vqaddlq_u8(v_hi));
      b_v = vdotq_u32(b_v, v_hi, weight_hi_v);
    }

    b_v = vaddq_u32(b_v, vshlq_n_u32(p_v, 5));

    *a += vaddvq_u32(a_v);
    *b = vaddvq_u32(b_v);

    blocks_remainder
  }

  #[inline(always)]
  unsafe fn vqaddlq_u8(a: uint8x16_t) -> uint32x4_t {
    vpaddlq_u16(vpaddlq_u8(a))
  }

  #[inline(always)]
  unsafe fn get_weight_lo() -> uint8x16_t {
    core::mem::transmute([
      32u8, 31, 30, 29, 28, 27, 26, 25, 24, 23, 22, 21, 20, 19, 18, 17,
    ])
  }

  #[inline(always)]
  unsafe fn get_weight_hi() -> uint8x16_t {
    core::mem::transmute([16u8, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1])
  }
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
    let mut random = [0; 1024 * 1024];
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
    if let Some(update) = super::get_imp() {
      let (a, b) = update(1, 0, data);
      let left = u32::from(b) << 16 | u32::from(a);
      let right = adler::adler32_slice(data);

      assert_eq!(left, right, "len({})", data.len());
    }
  }
}
