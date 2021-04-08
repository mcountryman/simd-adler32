#![cfg_attr(not(feature = "std"), no_std)]

use imp::{get_imp, Adler32Imp};
pub mod imp;

#[derive(Clone)]
pub struct Adler32 {
  low: u16,
  high: u16,
  update: Adler32Imp,
}

impl Adler32 {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn update(&mut self, data: &[u8]) {
    let (high, low) = (self.update)(self.low, self.high, data);

    self.low = low;
    self.high = high;
  }

  pub fn finalize(&mut self) -> u32 {
    let checksum = u32::from(self.high) << 16 | u32::from(self.low);

    self.low = 1;
    self.high = 0;

    checksum
  }
}

impl Default for Adler32 {
  fn default() -> Self {
    Self {
      low: 1,
      high: 0,
      update: get_imp(),
    }
  }
}
