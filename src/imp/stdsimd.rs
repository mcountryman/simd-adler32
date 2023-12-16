use super::Adler32Imp;

pub fn get_imp() -> Option<Adler32Imp> {
  get_imp_inner()
}

#[inline]
#[cfg(feature = "nightly")]
fn get_imp_inner() -> Option<Adler32Imp> {
  Some(imp::update)
}

#[inline]
#[cfg(not(feature = "nightly"))]
fn get_imp_inner() -> Option<Adler32Imp> {
  None
}

#[cfg(feature = "nightly")]
mod imp {
  use core::{
    simd::{num::SimdUint, Simd},
    slice::ChunksExact,
  };

  const LANES: usize = 64;

  const MOD: u32 = 65521;
  const NMAX: usize = 5552 & (!(LANES - 1));

  pub fn update(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
    let mut a = a as u32;
    let mut b = b as u32;

    let chunks = data.chunks_exact(NMAX);
    let remainder = chunks.remainder();

    for chunk in chunks {
      update_simd(&mut a, &mut b, chunk.chunks_exact(LANES));
      a %= MOD;
      b %= MOD;
    }

    let vs = remainder.chunks_exact(LANES);
    let vremainder = vs.remainder();
    update_simd(&mut a, &mut b, vs);

    for byte in vremainder {
      a = a.wrapping_add(*byte as _);
      b = b.wrapping_add(a);
    }

    a %= MOD;
    b %= MOD;

    (a as u16, b as u16)
  }

  const WEIGHTS: Simd<u16, LANES> = {
    let mut weights = [0; LANES];
    let mut i = 0;
    while i < LANES {
      weights[LANES - 1 - i] = i as u16 + 1;
      i += 1;
    }

    Simd::from_array(weights)
  };

  fn update_simd(a_out: &mut u32, b_out: &mut u32, values: ChunksExact<u8>) {
    let (mut a, mut b) = (*a_out, *b_out);

    for v in values {
      let v = Simd::from_slice(v).cast::<u16>();
      b = b
        .wrapping_add(a.wrapping_mul(LANES as u32))
        .wrapping_add((v * WEIGHTS).cast::<u32>().reduce_sum() as u32);
      a = a.wrapping_add(v.reduce_sum() as u32);
    }

    *a_out = a;
    *b_out = b;
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
    assert_sum_eq(&random[..8]);
    assert_sum_eq(&random[..64]);
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
