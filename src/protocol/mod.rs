use std::collections::HashMap;

use async_channel::{Receiver, Sender};
use binary_codec::SerializerConfig;
use serde::{Deserialize, Serialize};

pub mod error;

use crate::packets::{context::PlabbleConnectionContext, response::PlabbleResponsePacket};

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "client")]
pub mod client;

/// Plabble Connection
pub struct PlabbleConnection {
    pub hooks: HashMap<u16, Sender<PlabbleResponsePacket>>,
    pub config: SerializerConfig<PlabbleConnectionContext>,
    pub tx: Sender<Vec<u8>>,
    pub rx: Receiver<Vec<u8>>,
}

/// Implementation of common functionality for [`PlabbleConnection`].
impl PlabbleConnection {
    /// Creates a new [`PlabbleConnection`] with the given binary sender and receiver.
    pub fn new(tx: Sender<Vec<u8>>, rx: Receiver<Vec<u8>>) -> Self {
        Self {
            config: SerializerConfig::new(Some(PlabbleConnectionContext::new())),
            tx,
            rx,
            hooks: HashMap::new(),
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────
use crate::protocol::error::PlabbleProtocolError;

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
