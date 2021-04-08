<h1 align="center">simd-adler32</h1>
<p align="center">
  <a href="https://docs.rs/simd-adler32">
    <img alt="docs.rs badge" src="https://img.shields.io/docsrs/simd-adler32?style=flat-square">
  </a>
  <a href="https://bundlephobia.com/result?p=geotab-rx@latest">
    <img alt="crates.io badge" src="https://img.shields.io/crates/v/simd-adler32?style=flat-square">
  </a>
  <a href="https://github.com/mcountryman/simd-adler32/blob/main/LICENSE.md">
    <img alt="mit license badge" src="https://img.shields.io/github/license/mcountryman/simd-adler32?style=flat-square">
  </a>
</p>

A SIMD-accelerated Adler-32 rolling hash algorithm implementation.

## Goals

- Support `no_std` (with `default-features = false`)
- Runtime CPU feature detection (when `std` enabled)
- Single `cfg_if` dependency
- Blazing fast performance on as many targets as possible

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
adler.update(b"rust is pretty cool, man");
let hash = adler.finalize();

println!("{}", hash);
// 1921255656
```

## Performance

...

## Safety

...

## Support

...

## Contributing

Feel free to submit a issue or pull request. :smile:
