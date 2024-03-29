pub mod avx2;
pub mod avx512;
pub mod sse2;
pub mod ssse3;

/// A macro to test whether a CPU feature is available on x86/x86-x64 platforms.
///
/// This macro will attempt to test at runtime if `std` feature is enabled.  Otherwise will
/// fallback to target_feature conditional compilation flags.
#[allow(unused_macros)]
macro_rules! is_x86_feature_detected {
  ($name:tt) => {{
    #[cfg(feature = "std")]
    #[inline(always)]
    fn __is_x86_feature_detected() -> bool {
      std::is_x86_feature_detected!($name)
    }

    #[cfg(all(not(feature = "std"), target_feature = $name))]
    #[inline(always)]
    fn __is_x86_feature_detected() -> bool {
      true
    }

    #[cfg(all(not(feature = "std"), not(target_feature = $name)))]
    #[inline(always)]
    fn __is_x86_feature_detected() -> bool {
      false
    }

    __is_x86_feature_detected()
  }};
}

pub(crate) use is_x86_feature_detected;

#[inline]
#[allow(non_snake_case)]
pub const fn _mm_shuffle(z: u32, y: u32, x: u32, w: u32) -> i32 {
  ((z << 6) | (y << 4) | (x << 2) | w) as i32
}
