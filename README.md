<h1 align="center">simd-adler32</h1>
<p align="center">
  <a href="https://docs.rs/simd-adler32">
    <img alt="docs.rs badge" src="https://img.shields.io/docsrs/simd-adler32?style=flat-square">
  </a>
  <a href="https://crates.io/crates/simd-adler32">
    <img alt="crates.io badge" src="https://img.shields.io/crates/v/simd-adler32?style=flat-square">
  </a>
  <a href="https://github.com/mcountryman/simd-adler32/blob/main/LICENSE.md">
    <img alt="mit license badge" src="https://img.shields.io/github/license/mcountryman/simd-adler32?style=flat-square">
  </a>
</p>

A SIMD-accelerated Adler-32 hash algorithm implementation.

## Usage

> Cargo.toml
```toml
[dependencies]
simd-adler32 = "*"
```

> example.rs
```rust
simd_adler32::adler32(b"some data here");
```

## Features

- `std` - Enables runtime cpu-feature detection.  If disabled the fastest
  implementation will be determined by the rustc [target-feature](https://doc.rust-lang.org/rustc/codegen-options/index.html?highlight=target-feature#target-feature)
  flag defined at build-time.
- `msrv_1_89_0` - Raises the MSRV to 1.89.0.  Enables `avx512` implementation
- `msrv_1_61_0` - Raises the MSRV to 1.61.0.  Enables `neon` implementation
- `msrv_1_54_0` - Raises the MSRV to 1.54.0.  Enables `simd128` implementation

## MSRV

This crate's minimum supported rust version is `1.50.0`.  The intent here is to
retain parity with miniz_oxide.

## Credits

Thank you to the contributors of the following projects.

- [adler](https://github.com/jonas-schievink/adler)
- [adler32](https://github.com/remram44/adler32-rs)
- [crc32fast](https://github.com/srijs/rust-crc32fast)
- [wuffs](https://github.com/google/wuffs)
- [chromium](https://bugs.chromium.org/p/chromium/issues/detail?id=762564)
- [zlib](https://zlib.net/)
