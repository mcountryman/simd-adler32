use rand::prelude::*;

fn main() {
  if let Some(imp) = simd_adler32::imp::avx2::get_imp() {
    afl::fuzz!(|data: &[u8]| {
      imp(1, 0, data);

      // let mut rnd = rand::thread_rng();
      // let rnd: u8 = rnd.gen();

      // if rnd == 1 {
      //   // panic!("test");
      // }
    });
  }
}
