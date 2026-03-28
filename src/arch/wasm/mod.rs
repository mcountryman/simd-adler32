pub mod simd128;

use crate::Update;

pub fn best() -> Option<Update> {
  simd128::get()
}
