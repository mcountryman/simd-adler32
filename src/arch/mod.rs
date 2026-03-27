pub mod aarch64;
pub mod scalar;
pub mod wasm;
pub mod x86_64;

use crate::Update;

#[inline]
#[allow(non_snake_case)]
pub const fn _MM_SHUFFLE(z: u32, y: u32, x: u32, w: u32) -> i32 {
  ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

pub fn get_imp() -> Update {
  x86_64::avx512::get_imp()
    .or_else(aarch64::neon::get_imp)
    .or_else(x86_64::avx2::get_imp)
    .or_else(x86_64::ssse3::get_imp)
    .or_else(x86_64::sse2::get_imp)
    .or_else(wasm::simd128::get_imp)
    .unwrap_or(scalar::update)
}
