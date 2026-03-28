use simd_adler32::arch::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86_64 {
  use super::*;
  use simd_adler32::arch::x86_64::*;

  #[cfg(any(feature = "msrv_1_89_0", feature = "nightly"))]
  #[test]
  #[cfg_attr(
    not(all(target_feature = "avx512f", target_feature = "avx512bw")),
    ignore
  )]
  fn avx512() {
    assert_adler_sums(avx512::get());
  }

  #[test]
  #[cfg_attr(not(target_feature = "avx2"), ignore)]
  fn avx2() {
    assert_adler_sums(avx2::get());
  }

  #[test]
  #[cfg_attr(not(target_feature = "sse2"), ignore)]
  fn sse2() {
    assert_adler_sums(sse2::get());
  }

  #[test]
  #[cfg_attr(not(target_feature = "ssse3"), ignore)]
  fn ssse3() {
    assert_adler_sums(ssse3::get());
  }
}

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
// neon is the only specialization and requires nightly or 1.61.0
#[cfg(any(feature = "msrv_1_61_0", feature = "nightly"))]
mod aarch64 {
  use super::*;
  use simd_adler32::arch::aarch64::*;

  #[test]
  #[cfg_attr(not(target_feature = "neon"), ignore)]
  fn neon() {
    assert_adler_sums(neon::get());
  }
}

#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
mod wasm {
  use super::*;
  use simd_adler32::arch::wasm::*;

  #[test]
  #[cfg_attr(not(target_feature = "simd128"), ignore)]
  fn simd128() {
    assert_adler_sums(simd128::get());
  }
}

#[test]
fn scalar() {
  assert_adler_sums(Some(scalar::update));
}

fn assert_adler_sums(update: Option<fn(u16, u16, &[u8]) -> (u16, u16)>) {
  let update = update.expect("platform not supported");

  macro_rules! assert_adler_sum {
    ($input:expr, $expected:expr) => {
      let (a, b) = update(1, 0, $input);
      let actual = u32::from(b) << 16 | u32::from(a);

      assert_eq!($expected, actual);
    };
  }

  assert_adler_sum!(&[], 0x1);

  assert_adler_sum!(&[0], 0x10001);
  assert_adler_sum!(&[0, 0], 0x20001);
  assert_adler_sum!(&[0; 100], 0x640001);
  assert_adler_sum!(&[0; 1024], 0x4000001);
  assert_adler_sum!(&[0; 1024 * 1024], 0xf00001);

  assert_adler_sum!(&[1], 0x20002);
  assert_adler_sum!(&[1, 1], 0x50003);
  assert_adler_sum!(&[1; 100], 0x141e0065);
  assert_adler_sum!(&[1; 1024], 0x6780401);
  assert_adler_sum!(&[1; 1024 * 1024], 0x71e800f1);

  assert_adler_sum!(&[0x81], 0x820082);
  assert_adler_sum!(&[0x81, 1], 0x1050083);
  assert_adler_sum!(&[0x81; 100], 0xf1a53265);
  assert_adler_sum!(&[0x81; 1024], 0x4287041f);
  assert_adler_sum!(&[0x81; 1024 * 1024], 0xf13078f1);

  assert_adler_sum!(b"Wikipedia", 0x11e60398);
}
