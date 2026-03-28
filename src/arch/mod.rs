#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod aarch64;
pub mod scalar;
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub mod wasm;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86_64;

use crate::Update;

pub fn best() -> Update {
  let best = None;
  #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
  let best = best.or_else(aarch64::best);
  #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
  let best = best.or_else(wasm::best);
  #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
  let best = best.or_else(x86_64::best);

  best.unwrap_or(scalar::update)
}
