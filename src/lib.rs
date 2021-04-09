//! # simd-adler32
//!
//! A SIMD-accelerated Adler-32 rolling hash algorithm implementation.
//!
//! ## Features
//!
//! - No dependencies
//! - Support `no_std` (with `default-features = false`)
//! - Runtime CPU feature detection (when `std` enabled)
//! - Blazing fast performance on as many targets as possible (currently only x86 and x86_64)
//! - Default to scalar implementation when simd not available
//!
//! ## Quick start
//!
//! > Cargo.toml
//!
//! ```toml
//! [dependencies]
//! simd-adler32 = "*"
//! ```
//!
//! > example.rs
//!
//! ```rust
//! use simd_adler32::Adler32;
//!
//! let mut adler = Adler32::new();
//! adler.update(b"rust is pretty cool, man");
//! let hash = adler.finalize();
//!
//! println!("{}", hash);
//! // 1921255656
//! ```
//!
//! ## Support
//!
//! | supported          | arch             | feature |
//! | ------------------ | ---------------- | ------- |
//! | :construction:     | `x86`, `x86_64`  | avx512  |
//! | :white_check_mark: | `x86`, `x86_64`  | avx2    |
//! | :white_check_mark: | `x86`, `x86_64`  | ssse3   |
//! | :construction:     | `arm`, `aarch64` | neon    |
//! |                    | `wasm32`         | simd128 |
//!
//!
//! ## CPU Feature Detection
//! simd-adler32 supports both runtime and compile time CPU feature detection using the
//! `std::is_x86_feature_detected` macro when the `Adler32` struct is instantiated with
//! the `new` fn.  
//!
//! Without `std` feature enabled simd-adler32 falls back to compile time feature detection
//! using `target-feature` or `target-cpu` flags supplied to rustc. See https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html
//! for more information.
//!
//! Feature detection tries to use the fastest supported feature first.
#![cfg_attr(not(feature = "std"), no_std)]

#[doc(hidden)]
use imp::{get_imp, Adler32Imp};
pub mod imp;

/// A rolling hash generator type.
#[derive(Clone)]
pub struct Adler32 {
  low: u16,
  high: u16,
  update: Adler32Imp,
}

impl Adler32 {
  /// Constructs a new `Adler32`.
  ///
  /// # Examples
  /// ```rust
  /// use simd_adler32::Adler32;
  ///
  /// let mut adler = Adler32::new();
  /// ```
  ///
  /// # Remarks
  /// Potential overhead here due to runtime feature detection although in testing on 100k
  /// and 10k random byte arrays it was not really noticeable.
  pub fn new() -> Self {
    Default::default()
  }

  /// Compute checksum on supplied slice.
  pub fn update(&mut self, data: &[u8]) {
    let (high, low) = (self.update)(self.low, self.high, data);

    self.low = low;
    self.high = high;
  }

  /// Get final checksum and reset for further use.
  ///
  /// # Examples
  /// ```rust
  /// use simd_adler32::Adler32;
  ///
  /// let mut adler = Adler32::new();
  ///
  /// // Computes hash for `Hello there`
  /// adler.update(b"Hello there");
  /// println("{}", adler.finalize()); // 3735928495
  ///
  /// // After finalize is called inner checksum is reset so we are free to re-use `Adler32`
  /// // and hash again with `Another value`.
  /// adler.update(b"Another value");
  /// println("{}", adler.finalize()) // 800813569
  /// ```
  pub fn finalize(&mut self) -> u32 {
    let checksum = u32::from(self.high) << 16 | u32::from(self.low);

    self.low = 1;
    self.high = 0;

    checksum
  }
}

impl Default for Adler32 {
  fn default() -> Self {
    Self {
      low: 1,
      high: 0,
      update: get_imp(),
    }
  }
}
