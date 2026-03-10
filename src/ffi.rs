use std::sync::Arc;

use async_channel::{Receiver, Sender};
use binary_codec::{BinarySerializer, SerializerConfig};
use futures::lock::Mutex;
use serde::Serialize;

use crate::{
    packets::request::PlabbleRequestPacket,
    protocol::{
        PlabbleConnection as InnerPlabbleConnection,
        error::PlabbleProtocolError,
    },
};

// ── Callback interfaces ─────────────────────────────────────────────────────

/// Callback interface for looking up bucket keys by bucket ID.
#[uniffi::export(callback_interface)]
pub trait BucketKeyProvider: Send + Sync {
    /// Given a bucket ID serialized as bytes, return the 32-byte bucket key, or None.
    fn get_bucket_key(&self, bucket_id_bytes: Vec<u8>) -> Option<Vec<u8>>;
}

/// Callback interface for looking up pre-shared keys.
#[uniffi::export(callback_interface)]
pub trait PskProvider: Send + Sync {
    /// Given a 12-byte PSK ID, return the 64-byte pre-shared key, or None.
    fn get_psk(&self, psk_id: Vec<u8>) -> Option<Vec<u8>>;
}

// ── Connection object ───────────────────────────────────────────────────────

/// A Plabble protocol connection handle.
#[derive(uniffi::Object)]
pub struct PlabbleConnection {
    inner: Mutex<InnerPlabbleConnection>,
    tx: Receiver<Vec<u8>>,
    rx: Sender<Vec<u8>>,
}

// Safety: The inner PlabbleConnection is protected by a Mutex ensuring exclusive
// access. The callback closures stored in the connection context capture only
// Send+Sync values (Arc<dyn BucketKeyProvider> and Arc<dyn PskProvider>).
unsafe impl Send for PlabbleConnection {}
unsafe impl Sync for PlabbleConnection {}

#[uniffi::export]
impl PlabbleConnection {
    /// Create a new Plabble connection.
    #[uniffi::constructor]
    pub fn new() -> Self {
        let (rx_sender, inner_rx) = async_channel::unbounded();
        let (inner_tx, tx_receiver) = async_channel::unbounded();
        let inner = InnerPlabbleConnection::new(inner_tx, inner_rx);

        Self {
            inner: Mutex::new(inner),
            tx: tx_receiver,
            rx: rx_sender,
        }
    }

    /// Set a callback for looking up bucket keys by bucket ID.
    pub async fn set_bucket_key_provider(&self, provider: Box<dyn BucketKeyProvider>) {
        let provider: Arc<dyn BucketKeyProvider> = Arc::from(provider);
        let mut inner = self.inner.lock().await;
        let data = inner.config.data.as_mut().unwrap();
        data.get_bucket_key = Some(Arc::new(move |bucket_id| {
            let bytes = bucket_id
                .to_bytes(None::<&mut SerializerConfig<()>>)
                .unwrap();
            provider
                .get_bucket_key(bytes)
                .and_then(|v| <[u8; 32]>::try_from(v).ok())
        }));
    }

    /// Set a callback for looking up pre-shared keys by PSK ID.
    pub async fn set_psk_provider(&self, provider: Box<dyn PskProvider>) {
        let provider: Arc<dyn PskProvider> = Arc::from(provider);
        let mut inner = self.inner.lock().await;
        let data = inner.config.data.as_mut().unwrap();
        data.get_psk = Some(Arc::new(move |psk_id| {
            provider
                .get_psk(psk_id.to_vec())
                .and_then(|v| <[u8; 64]>::try_from(v).ok())
        }));
    }

    /// Send a request packet serialized as a JSON (or TOML) string.
    pub async fn send_request(&self, request: String) -> Result<(), PlabbleProtocolError> {
        let packet = deserialize_request(&request)?;
        let mut inner = self.inner.lock().await;
        inner.send(packet).await
    }

    /// Feed raw incoming bytes received from the transport layer into the connection.
    pub fn handle_incoming(&self, bytes: Vec<u8>) -> Result<(), PlabbleProtocolError> {
        self.rx
            .try_send(bytes)
            .map_err(|_| PlabbleProtocolError::SenderError)
    }

    /// Poll for the next outgoing packet (non-blocking).
    /// Returns the raw bytes to send over the transport, or None if nothing is queued.
    pub async fn poll_outgoing(&self) -> Option<Vec<u8>> {
        self.tx.recv().await.ok()
    }
}

// ── Free functions ──────────────────────────────────────────────────────────

/// Returns the library version string.
#[uniffi::export]
pub fn plabble_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Deserialize a request packet from a JSON or TOML string (depending on enabled features).
fn deserialize_request(data: &str) -> Result<PlabbleRequestPacket, PlabbleProtocolError> {
    #[cfg(feature = "with-json")]
    {
        return serde_json::from_str(data).map_err(|_| PlabbleProtocolError::InputParsingFailed);
    }

    #[cfg(all(feature = "with-toml", not(feature = "with-json")))]
    {
        return toml::from_str(data).map_err(|_| PlabbleProtocolError::InputParsingFailed);
    }
}

/// Serialize an object to a JSON or TOML string (depending on enabled features).
#[allow(dead_code)]
fn serialize_output<T: Serialize>(data: &T) -> Result<String, PlabbleProtocolError> {
    #[cfg(feature = "with-json")]
    {
        return serde_json::to_string(data)
            .map_err(|_| PlabbleProtocolError::OutputSerializationFailed);
    }

    #[cfg(all(feature = "with-toml", not(feature = "with-json")))]
    {
        return toml::to_string(data).map_err(|_| PlabbleProtocolError::OutputSerializationFailed);
    }
}