pub use tcp::*;
#[cfg(not(target_arch = "wasm32"))]
pub mod serial;
pub mod tcp;
