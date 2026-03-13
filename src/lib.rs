pub mod core;
pub mod crypto;
pub mod errors;
pub mod packets;
pub mod scripting;

#[cfg(feature = "protocol")]
pub mod protocol;

// Initialize uniffi
#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

#[cfg(feature = "blockchain")]
pub mod blockchain;

#[cfg(feature = "ffi")]
pub mod ffi;

#[cfg(feature = "wasm")]
pub mod wasm;
