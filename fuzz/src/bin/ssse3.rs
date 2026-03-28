#![no_main]

use libfuzzer_sys::fuzz_target;

#[cfg(target_feature = "ssse3")]
fuzz_target!(|data: &[u8]| {
  unsafe { simd_adler32::arch::x86_64::ssse3::update(1, 0, data) };
});

#[cfg(not(target_feature = "ssse3"))]
compile_error!("missing target_feature `ssse3`");
