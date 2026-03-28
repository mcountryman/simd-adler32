#[cfg(any(feature = "msrv_1_61_0", feature = "nightly"))]
pub mod neon;

use crate::Update;

pub fn best() -> Option<Update> {
  let best = None;
  #[cfg(any(feature = "msrv_1_61_0", feature = "nightly"))]
  let best = best.or_else(neon::get);

  best
}
