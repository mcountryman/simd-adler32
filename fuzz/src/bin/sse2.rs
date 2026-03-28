#![no_main]

use libfuzzer_sys::fuzz_target;

#[cfg(target_feature = "sse2")]
fuzz_target!(|data: &[u8]| {
  unsafe { simd_adler32::arch::x86_64::sse2::update(1, 0, data) };
});

#[cfg(not(target_feature = "sse2"))]
compile_error!("missing target_feature `sse2`");
