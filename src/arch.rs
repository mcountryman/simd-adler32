pub mod scalar;
#[cfg(any(target_arch = "wasm32", target_arch = "wasm64"))]
pub mod wasm;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub mod x86;
