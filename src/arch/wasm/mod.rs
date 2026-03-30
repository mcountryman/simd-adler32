#[cfg(feature = "msrv_1_54_0")]
pub mod simd128;

use crate::Update;

pub fn best() -> Option<Update> {
  let best = None;
  #[cfg(feature = "msrv_1_54_0")]
  let best = best.or_else(simd128::get);

  best
}
