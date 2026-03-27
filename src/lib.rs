//! # simd-adler32
//!
//! A SIMD-accelerated Adler-32 hash algorithm implementation.
//!
//! ## Usage
//!
//! > Cargo.toml
//! ```toml
//! [dependencies]
//! simd-adler32 = "*"
//! ```
//!
//! > example.rs
//! ```rust
//! simd_adler32::adler32(b"some data here");
//! ```
//!
//! ## Features
//!
//! - `std` - Enables runtime cpu-feature detection.  If disabled the fastest
//! implementation will be determined by the rustc [target-feature](https://doc.rust-lang.org/rustc/codegen-options/index.html?highlight=target-feature#target-feature)
//! flag defined at build-time.
//! - `nightly` - Enables nightly rust features that otherwise wouldn't be available.
//!
//! ## MSRV
//!
//! This crate's minimum supported rust version is `1.50.0`.  The intent here is to
//! retain parity with miniz_oxide.
//!
//! ## Credits
//!
//! Thank you to the contributors of the following projects.
//!
//! - [adler](https://github.com/jonas-schievink/adler)
//! - [adler32](https://github.com/remram44/adler32-rs)
//! - [crc32fast](https://github.com/srijs/rust-crc32fast)
//! - [wuffs](https://github.com/google/wuffs)
//! - [chromium](https://bugs.chromium.org/p/chromium/issues/detail?id=762564)
//! - [zlib](https://zlib.net/)
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(
  all(feature = "nightly", any(target_arch = "x86", target_arch = "x86_64")),
  feature(stdarch_x86_avx512, avx512_target_feature)
)]
#![cfg_attr(
  all(
    feature = "nightly",
    target_arch = "wasm64",
    target_feature = "simd128"
  ),
  feature(simd_wasm64)
)]

#[doc(hidden)]
pub mod imp;

use imp::{get_imp, Update};

/// An adler32 hash generator type.
#[derive(Clone)]
pub struct Adler32 {
  a: u16,
  b: u16,
  update: Update,
}

impl Adler32 {
  /// Constructs a new `Adler32`.
  ///
  /// Potential overhead here due to runtime feature detection although in testing on 100k
  /// and 10k random byte arrays it was not really noticeable.
  ///
  /// # Examples
  /// ```rust
  /// use simd_adler32::Adler32;
  ///
  /// let mut adler = Adler32::new();
  /// ```
  pub fn new() -> Self {
    Default::default()
  }

  /// Constructs a new `Adler32` using existing checksum.
  ///
  /// Potential overhead here due to runtime feature detection although in testing on 100k
  /// and 10k random byte arrays it was not really noticeable.
  ///
  /// # Examples
  /// ```rust
  /// use simd_adler32::Adler32;
  ///
  /// let mut adler = Adler32::from_checksum(0xdeadbeaf);
  /// ```
  pub fn from_checksum(checksum: u32) -> Self {
    Self {
      a: checksum as u16,
      b: (checksum >> 16) as u16,
      update: get_imp(),
    }
  }

  /// Computes hash for supplied data and stores results in internal state.
  pub fn update(&mut self, data: &[u8]) {
    let (a, b) = (self.update)(self.a, self.b, data);

    self.a = a;
    self.b = b;
  }

  /// Returns the hash value for the values written so far.
  ///
  /// Despite its name, the method does not reset the hasher’s internal state. Additional
  /// writes will continue from the current value. If you need to start a fresh hash
  /// value, you will have to use `reset`.
  pub fn finish(&self) -> u32 {
    (u32::from(self.b) << 16) | u32::from(self.a)
  }

  /// Resets the internal state.
  pub fn reset(&mut self) {
    self.a = 1;
    self.b = 0;
  }
}

/// Compute Adler-32 hash on `Adler32Hash` type.
///
/// # Arguments
/// * `hash` - A Adler-32 hash-able type.
///
/// # Examples
/// ```rust
/// use simd_adler32::adler32;
///
/// let hash = adler32(b"Adler-32");
/// println!("{}", hash); // 800813569
/// ```
pub fn adler32<H: AsRef<[u8]>>(hash: &H) -> u32 {
  let mut hasher = Adler32::new();

  hasher.update(hash.as_ref());
  hasher.finish()
}

impl Default for Adler32 {
  fn default() -> Self {
    Self {
      a: 1,
      b: 0,
      update: get_imp(),
    }
  }
}
