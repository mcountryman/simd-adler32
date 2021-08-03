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

A SIMD-accelerated Adler-32 rolling hash algorithm implementation.

## Features

- No dependencies
- Support `no_std` (with `default-features = false`)
- Runtime CPU feature detection (when `std` enabled)
- Blazing fast performance on as many targets as possible (currently only x86 and x86_64)
- Default to scalar implementation when simd not available

## Feature flags

- `std` - Enabled by default

Enables std support, see [CPU Feature Detection](#cpu-feature-detection) for runtime
detection support.

- `nightly`

Enables nightly features required for avx512 support.

- `const-generics` - Enabled by default

Enables const-generics support allowing for user-defined array hashing by value. See
[`Adler32Hash`] for details.

## Quick start

> Cargo.toml

```toml
[dependencies]
simd-adler32 = "*"
```

> example.rs

```rust
use simd_adler32::Adler32;

let mut adler = Adler32::new();
adler.write(b"rust is pretty cool, man");
let hash = adler.finish();

println!("{}", hash);
// 1921255656
```

## Support

**CPU Features**

| impl | arch             | feature |
| ---- | ---------------- | ------- |
| ✅   | `x86`, `x86_64`  | avx512  |
| ✅   | `x86`, `x86_64`  | avx2    |
| ✅   | `x86`, `x86_64`  | ssse3   |
| ✅   | `x86`, `x86_64`  | sse2    |
| 🚧   | `arm`, `aarch64` | neon    |
|      | `wasm32`         | simd128 |

**MSRV** `1.36.0`\*\*

Minimum supported rust version is tested before a new version is published. [**] Feature
`const-generics` needs to disabled to build on rustc versions `<1.51` which can be done
by updating your dependency definition to the following.

> Cargo.toml

```toml
[dependencies]
simd-adler32 = { version "*", default-features = false, features = ["std"] }
```

## Performance

Benchmarks listed display number of randomly generated bytes (10k / 100k) and library
name. Benchmarks sources can be found under the [bench](/bench) directory. Crates used for
comparison are [adler](https://crates.io/crates/adler) and
[adler32](https://crates.io/crates/adler32).

> Windows 10 Pro - Intel i5-8300H @ 2.30GHz

| name                      | avg. time       | avg. thrpt           |
| ------------------------- | --------------- | -------------------- |
| **10k/simd-adler32**      | **217.936 ns**  | **42.734 GiB/s**     |
| 10k/wuffs                 | 884.369 ns      | 10.531 GiB/s         |
| 10k/adler32               | 4.576 µs        | 2.035 GiB/s          |
| 10k/adler                 | 18.515 µs       | 515.075 MiB/s        |
| ------------------------- | --------------- | -------------------- |
| **100k/simd-adler32**     | **2.542 µs**    | **36.641 GiB/s**     |
| 100k/wuffs                | 4.873 µs        | 19.111 GiB/s         |
| 100k/adler32              | 45.917 µs       | 2.028 GiB/s          |
| 100k/adler                | 178.365 µs      | 534.677 MiB/s        |

\* wuffs ran using mingw64/gcc, ran with `wuffs bench -ccompilers=gcc -reps=1 -iterscale=300 std/adler32`.

> MacBookPro16,1 - Intel i9-9880H CPU @ 2.30GHz

| name                      | avg. time       | avg. thrpt           |
| ------------------------- | --------------- | -------------------- |
| **10k/simd-adler32**      | **194.690 ns**  | **47.836 GiB/s**     |
| 10k/wuffs                 | 517.003 ns      | 18.014 GiB/s         |
| 10k/adler32               | 4.044 µs        | 2.303 GiB/s          |
| 10k/adler                 | 17.490 µs       | 545.284 MiB/s        |
| ------------------------- | --------------- | -------------------- |
| **100k/simd-adler32**     | **2.237 µs**    | **41.633 GiB/s**     |
| 100k/wuffs                | 3.908 µs        | 23.828 GiB/s         |
| 100k/adler32              | 40.745 µs       | 2.286 GiB/s          |
| 100k/adler                | 174.528 µs      | 546.431 MiB/s        |

> c5.xlarge - Intel(R) Xeon(R) Platinum 8124M CPU @ 3.00GHz

| name                      | avg. time       | avg. thrpt           |
| ------------------------- | --------------- | -------------------- |
| **10k/simd-adler32**      | **202.186 ns**  | **46.063 GiB/s**     |
| 10k/wuffs                 | 2.247 µs        | 4.144 GiB/s          |
| 10k/adler                 | 2.968 µs        | 3.138 GiB/s          |
| 10k/adler32               | 4.898 µs        | 1.901 GiB/s          |
| ------------------------- | --------------- | -------------------- |
| **100k/simd-adler32**     | **2.397 µs**    | **38.855 GiB/s**     |
| 100k/wuffs                | 7.106 µs        | 13.107 GiB/s         |
| 100k/adler                | 25.209 µs       | 3.694 GiB/s          |
| 100k/adler32              | 49.044 µs       | 1.899 GiB/s          |

## CPU Feature Detection

simd-adler32 supports both runtime and compile time CPU feature detection using the
`std::is_x86_feature_detected` macro when the `Adler32` struct is instantiated with
the `new` fn.  
Without `std` feature enabled simd-adler32 falls back to compile time feature detection
using `target-feature` or `target-cpu` flags supplied to rustc. See [https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html](https://rust-lang.github.io/packed_simd/perf-guide/target-feature/rustflags.html)
for more information.
Feature detection tries to use the fastest supported feature first.

## Safety

This crate contains a significant amount of `unsafe` code due to the requirement of `unsafe`
for simd intrinsics. Fuzzing is done on release and debug builds prior to publishing via
`afl`. Fuzzy tests can be found under [fuzz](/fuzz) the directory.

## Resources

- [LICENSE](./LICENSE.md) - MIT
- [CHANGELOG](./CHANGELOG.md)

## Credits

Thank you to the contributors of the following projects.

- [adler](https://github.com/jonas-schievink/adler)
- [adler32](https://github.com/remram44/adler32-rs)
- [crc32fast](https://github.com/srijs/rust-crc32fast)
- [wuffs](https://github.com/google/wuffs)
- [chromium](https://bugs.chromium.org/p/chromium/issues/detail?id=762564)
- [zlib](https://zlib.net/)

## Contributing

Feel free to submit a issue or pull request. :smile:
