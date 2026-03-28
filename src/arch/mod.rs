//! Specialized implementations of Adler-32.
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub mod aarch64;
pub mod scalar;
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub mod wasm;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86_64;

use crate::Update;

/// Returns the Adler-32 sums using the best specialized implementation.
pub fn update(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
  use core::mem;
  use core::sync::atomic::{AtomicPtr, Ordering};

  static UPDATE: AtomicPtr<()> = AtomicPtr::new(resolve_store_update as *mut _);

  fn resolve_store_update(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
    let update = best();

    UPDATE.store(update as *mut _, Ordering::Relaxed);

    update(a, b, data)
  }

  let update = UPDATE.load(Ordering::Relaxed);
  let update = unsafe { mem::transmute::<*mut (), Update>(update) };

  update(a, b, data)
}

/// Returns the best specialized implementation of [Update].
#[inline]
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
