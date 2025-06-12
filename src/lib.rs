// Export types for both Rust and WASM
pub mod types;

// Export the core module for Rust users
pub mod core;
// Re-export core for Rust users
pub use core::*;

// Only compile WASM module when targeting wasm32
#[cfg(target_arch = "wasm32")]
pub mod wasm;
// Re-export wasm bindings when compiling for wasm32
#[cfg(target_arch = "wasm32")]
pub use wasm::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    Ok(())
}
