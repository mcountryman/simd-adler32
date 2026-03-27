use simd_adler32::imp::*;

#[test]
#[cfg_attr(
  not(all(target_feature = "avx512f", target_feature = "avx512bw")),
  ignore
)]
fn avx512() {
  assert_adler_sums(avx512::get_imp());
}

#[test]
#[cfg_attr(not(target_feature = "avx2"), ignore)]
fn avx2() {
  assert_adler_sums(avx2::get_imp());
}

#[test]
#[cfg_attr(not(target_feature = "sse2"), ignore)]
fn sse2() {
  assert_adler_sums(sse2::get_imp());
}

#[test]
#[cfg_attr(not(target_feature = "ssse3"), ignore)]
fn ssse3() {
  assert_adler_sums(ssse3::get_imp());
}

#[test]
#[cfg_attr(not(target_feature = "neon"), ignore)]
fn neon() {
  assert_adler_sums(neon::get_imp());
}

#[test]
#[cfg_attr(not(target_feature = "simd128"), ignore)]
fn wasm() {
  assert_adler_sums(wasm::get_imp());
}

#[test]
fn scalar() {
  assert_adler_sums(Some(scalar::update));
}

fn assert_adler_sums(update: Option<Adler32Imp>) {
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
