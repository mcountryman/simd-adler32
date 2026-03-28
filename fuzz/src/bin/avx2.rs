#![no_main]

use libfuzzer_sys::fuzz_target;

#[cfg(target_feature = "avx2")]
fuzz_target!(|data: &[u8]| {
  unsafe { simd_adler32::arch::x86_64::avx2::update(1, 0, data) };
});

#[cfg(not(target_feature = "avx2"))]
compile_error!("missing target_feature `avx2`");
