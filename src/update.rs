/// Return the 16bit Adler-32 checksum pair from the given seed pair and bytes.
pub type Adler32Update = fn(u16, u16, &[u8]) -> (u16, u16);

/// Returns the [Adler32Update] function that runs best on the target system.
pub fn best() -> Adler32Update {
  cfg_if::cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
      fn best_for_target_arch() -> Adler32Update {
        use crate::arch::x86::*;
        use crate::arch::scalar;

        avx512::get_update_if_supported()
          .or_else(avx2::get_update_if_supported)
          .or_else(ssse3::get_update_if_supported)
          .or_else(sse2::get_update_if_supported)
          .unwrap_or(scalar::update)
      }
    } else if #[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))] {
      fn best_for_target_arch() -> Adler32Update {
        use crate::arch::wasm;
        use crate::arch::scalar;

        wasm::get_update_if_supported()
          .unwrap_or(scalar::update)
      }
    } else {
      fn best_for_target_arch() -> Adler32Update {
        use crate::arch::scalar;

        scalar::update
      }
    }
  }

  best_for_target_arch()
}
