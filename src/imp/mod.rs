pub mod scalar;
#[cfg(target_feature = "ssse3")]
pub mod ssse3;

pub type Adler32Imp = fn(u16, u16, &[u8]) -> (u16, u16);

pub fn get_imp() -> Adler32Imp {
  ssse3::update
}
