pub mod avx2;
#[cfg(feature = "msrv_1_89_0")]
pub mod avx512;
pub mod sse2;
pub mod ssse3;

use crate::Update;

pub fn best() -> Option<Update> {
  let best = None;

  #[cfg(feature = "msrv_1_89_0")]
  let best = best.or_else(avx512::get);

  best
    .or_else(avx2::get)
    .or_else(ssse3::get)
    .or_else(sse2::get)
}

#[inline]
#[allow(non_snake_case)]
const fn _MM_SHUFFLE(z: u32, y: u32, x: u32, w: u32) -> i32 {
  ((z << 6) | (y << 4) | (x << 2) | w) as i32
}
