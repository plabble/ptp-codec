mod bucket_id;
mod datetime;
pub mod node_address;

pub use bucket_id::BucketId;
pub use datetime::PlabbleDateTime;

/// Default to true for serde boolean fields
pub fn default_true() -> bool {
    true
}
