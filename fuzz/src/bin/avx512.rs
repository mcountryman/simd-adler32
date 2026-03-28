#![no_main]

use libfuzzer_sys::fuzz_target;

#[cfg(all(target_feature = "avx512f", target_feature = "avx512bw"))]
fuzz_target!(|data: &[u8]| {
  unsafe { simd_adler32::arch::x86_64::avx512::update(1, 0, data) };
});

#[cfg(not(all(target_feature = "avx512f", target_feature = "avx512bw")))]
compile_error!("missing target_feature `avx512`");
