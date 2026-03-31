# Changelog

## 1.0.0 - tbd

- Remove `Adler32Hash` trait
- Remove `nightly` feature
  - Rust 1.89.0 now supports everything we're using here
  - Nightly feature may have helped for users on really old versions of nightly
  but, testing and verifying that something broke for a nightly version from 5
  years ago is a pain.
- Add `msrv_*` features
- Rename `imp` -> `arch`
- Remove `#[doc(hidden)]` attributes

## 0.3.3 - 2021-04-14

### Features

- **from_checksum**: add `Adler32::from_checksum`

### Performance Improvements

- **scalar**: improve scalar performance by 90-600%
  - Defer modulo until right before u16 overflow
