#[cfg(feature = "msrv_1_61_0")]
pub mod neon;

use crate::Update;

pub fn best() -> Option<Update> {
  let best = None;
  #[cfg(feature = "msrv_1_61_0")]
  let best = best.or_else(neon::get);

  best
}
