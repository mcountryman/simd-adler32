#![no_main]
#[macro_use]
use libfuzzer_sys::fuzz_target;

fuzz_target! {
  |data: &[u8] {
    if let Some(imp) = simd_adler32::imp::avx512::get_imp() {
      imp(1, 0, data);
    }
  }
}
