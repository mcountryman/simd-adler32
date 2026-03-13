#[cfg(not(feature = "force-scalar"))]
pub mod avx2;
#[cfg(not(feature = "force-scalar"))]
pub mod avx512;
pub mod scalar;
#[cfg(not(feature = "force-scalar"))]
pub mod sse2;
#[cfg(not(feature = "force-scalar"))]
pub mod ssse3;
#[cfg(not(feature = "force-scalar"))]
pub mod wasm;

pub type Adler32Imp = fn(u16, u16, &[u8]) -> (u16, u16);

#[inline]
#[allow(non_snake_case)]
pub const fn _MM_SHUFFLE(z: u32, y: u32, x: u32, w: u32) -> i32 {
  ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

pub fn get_imp() -> Adler32Imp {
  #[cfg(feature = "force-scalar")]
  return scalar::update;
  #[cfg(not(feature = "force-scalar"))]
  return avx512::get_imp()
    .or_else(avx2::get_imp)
    .or_else(ssse3::get_imp)
    .or_else(sse2::get_imp)
    .or_else(wasm::get_imp)
    .unwrap_or(scalar::update);
}
