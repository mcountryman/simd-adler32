pub mod scalar;
pub mod ssse3;

pub type Adler32Imp = dyn Fn(u16, u16, &[u8]) -> (u16, u16);

pub fn get_imp() -> &'static Adler32Imp {
  &scalar::update
}
