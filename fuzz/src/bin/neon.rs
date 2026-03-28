#![no_main]

use libfuzzer_sys::fuzz_target;

#[cfg(target_feature = "neon")]
fuzz_target!(|data: &[u8]| {
  unsafe { simd_adler32::arch::aarch64::neon::update(1, 0, data) };
});

#[cfg(not(target_feature = "neon"))]
compile_error!("missing target_feature `neon`");
