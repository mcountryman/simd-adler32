const MOD: u32 = 65521;
const NMAX: usize = 5552;

pub fn update(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
  let mut a = a as u32;
  let mut b = b as u32;

  let chunks = data.chunks_exact(NMAX);
  let remainder = chunks.remainder();

  for chunk in chunks {
    for byte in chunk {
      a = a.wrapping_add(*byte as _);
      b = b.wrapping_add(a);
    }

    a %= MOD;
    b %= MOD;
  }

  for byte in remainder {
    a = a.wrapping_add(*byte as _);
    b = b.wrapping_add(a);
  }

  a %= MOD;
  b %= MOD;

  (a as u16, b as u16)
}
