use core::sync::atomic::{AtomicPtr, Ordering};

pub mod avx2;
pub mod avx512;
pub mod scalar;
pub mod sse2;
pub mod ssse3;
pub mod wasm;

type Adler32Imp = fn(u16, u16, &[u8]) -> (u16, u16);

#[inline]
#[allow(non_snake_case)]
pub const fn _MM_SHUFFLE(z: u32, y: u32, x: u32, w: u32) -> i32 {
  ((z << 6) | (y << 4) | (x << 2) | w) as i32
}

fn get_imp() -> Adler32Imp {
  avx512::get_imp()
    .or_else(avx2::get_imp)
    .or_else(ssse3::get_imp)
    .or_else(sse2::get_imp)
    .or_else(wasm::get_imp)
    .unwrap_or(scalar::update)
}

#[inline]
const fn adler_imp_to_raw_pointer(imp: Adler32Imp) -> *mut () {
  // Safety: Equivalent to `imp as usize as *mut ()`, but avoids pointer-to-int
  // casts which are lossy in terms of provenance.
  unsafe { core::mem::transmute(imp) }
}

// This either contains the resolver function (initially), or the
// already-resolved `Adler32Imp` (after the first call).
static IMP: AtomicPtr<()> = AtomicPtr::new(adler_imp_to_raw_pointer(resolve_and_call));
// Initial value of `IMP`. This resolves the implementation to use, stores it in
// IMP (so that all calls after the first skip resolving), and then forwards the
// arguments it gets to the implementation it resolved.
fn resolve_and_call(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
  let resolved_imp = get_imp();
  let imp_as_raw_ptr = adler_imp_to_raw_pointer(resolved_imp);
  // Ensure the next call goes directly to the resolved implementation.
  IMP.store(imp_as_raw_ptr, Ordering::Relaxed);
  // Forward the arguments on.
  resolved_imp(a, b, data)
}

/// Loads and invokes the implementation, resolving it if needed (only needed
/// the first time through).
#[inline]
pub fn call(a: u16, b: u16, data: &[u8]) -> (u16, u16) {
  let imp = IMP.load(Ordering::Relaxed);
  // Safety: `IMP` only ever contains valid `Adler32Imp`s.
  let imp: Adler32Imp = unsafe { core::mem::transmute(imp) };
  imp(a, b, data)
}
