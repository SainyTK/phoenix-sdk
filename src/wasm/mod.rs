// This module only compiles when targeting wasm32
#![cfg(target_arch = "wasm32")]

pub mod bindings;

// Re-export the WASM-specific components
pub use bindings::PhoenixSDK; 