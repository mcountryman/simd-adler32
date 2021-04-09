use simd_adler32::imp;
use std::time::Instant;

const DURATION_MS: u128 = 10_000;

#[cfg(not(tarpaulin_include))]
fn main() {
  let imp = std::env::args()
    .nth(1)
    .unwrap_or_else(|| "avx2".to_owned())
    .trim()
    .to_lowercase();

  let imp = match imp.as_str() {
    "avx2" => imp::avx2::get_imp().expect("avx2 not supported on this system!"),
    "ssse3" => imp::ssse3::get_imp().expect("ssse3 not supported on this system!"),
    _ => {
      println!("Defaulting to scalar impl");
      imp::scalar::update
    }
  };

  let buf = vec![0xaf; 1024 * 1024 * 1024];

  // Estimate time for each computation
  let time = Instant::now();
  let warmups = 5;
  for _ in 0..warmups {
    imp(1, 0, &buf[..]);
  }

  let time = Instant::now().duration_since(time).as_millis();
  let time = time / warmups as u128;
  let est_iters = DURATION_MS / time;

  println!("est time: {}ms", time);
  println!("est iters: {}", est_iters);

  for _ in 0..est_iters {
    imp(1, 0, &buf[..]);
  }
}
