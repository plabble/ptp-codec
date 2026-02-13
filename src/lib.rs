pub mod core;
pub mod crypto;
pub mod errors;
pub mod packets;
pub mod scripting;

/// Default to true for serde boolean fields
fn default_true() -> bool {
    true
}
