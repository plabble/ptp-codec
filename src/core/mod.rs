mod bucket_id;
mod datetime;

pub use bucket_id::BucketId;
pub use datetime::PlabbleDateTime;
use serde::{Deserialize, Serialize};

use crate::protocol::error::PlabbleProtocolError;

/// Default to true for serde boolean fields
pub fn default_true() -> bool {
    true
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Deserialize a packet from a JSON or TOML string (depending on enabled features).
pub fn deserialize_input<T: for<'a> Deserialize<'a>>(
    data: &str,
) -> Result<T, PlabbleProtocolError> {
    #[cfg(all(feature = "use-json", not(feature = "use-toml")))]
    {
        serde_json::from_str(data).map_err(|_| PlabbleProtocolError::InputParsingFailed)
    }

    #[cfg(all(feature = "use-toml", not(feature = "use-json")))]
    {
        toml::from_str(data).map_err(|_| PlabbleProtocolError::InputParsingFailed)
    }
}

/// Serialize an object to a JSON or TOML string (depending on enabled features).
pub fn serialize_output<T: Serialize>(data: &T) -> Result<String, PlabbleProtocolError> {
    #[cfg(all(feature = "use-json", not(feature = "use-toml")))]
    {
        serde_json::to_string(data).map_err(|_| PlabbleProtocolError::OutputSerializationFailed)
    }

    #[cfg(all(feature = "use-toml", not(feature = "use-json")))]
    {
        toml::to_string(data).map_err(|_| PlabbleProtocolError::OutputSerializationFailed)
    }
}
