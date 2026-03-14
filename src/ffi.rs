use std::sync::Arc;

use async_channel::{Receiver, Sender};
use futures::lock::Mutex;

use crate::{providers::KeyProvider, protocol::{
    PlabbleConnection as InnerPlabbleConnection, deserialize_input, error::PlabbleProtocolError,
    serialize_output,
}};

// ── Callback interfaces ─────────────────────────────────────────────────────

/// Callback interface for looking up bucket keys by bucket ID and pre-shared keys.
#[uniffi::export(callback_interface)]
pub trait SessionKeyProvider: Send + Sync {
    /// Given a bucket ID serialized as bytes, return the 32-byte bucket key, or None.
    fn get_bucket_key(&self, bucket_id_bytes: Vec<u8>) -> Option<Vec<u8>>;

    /// Given a 12-byte PSK ID, return the 64-byte pre-shared key, or None.
    fn get_psk(&self, psk_id: Vec<u8>) -> Option<Vec<u8>>;

    /// Store a pre-shared key with the given PSK ID and optional expiration time (as a UNIX timestamp).
    fn store_psk(&self, psk_id: Vec<u8>, psk: Vec<u8>, expiration: Option<u32>);
}

struct KeyProviderBridge {
    inner: Arc<dyn SessionKeyProvider>,
}

impl KeyProviderBridge {
    fn new(inner: Arc<dyn SessionKeyProvider>) -> Self {
        Self { inner }
    }
}

impl KeyProvider for KeyProviderBridge {
    fn get_bucket_key(&self, bucket_id: &[u8; 16]) -> Option<[u8; 32]> {
        let result = self.inner.get_bucket_key(bucket_id.to_vec())?;
        result.try_into().ok()
    }

    fn get_psk(&self, psk_id: &[u8; 12]) -> Option<[u8; 64]> {
        let result = self.inner.get_psk(psk_id.to_vec())?;
        result.try_into().ok()
    }

    fn store_psk(&self, psk_id: [u8; 12], psk: [u8; 64], expiration: Option<u32>) {
        self.inner.store_psk(psk_id.to_vec(), psk.to_vec(), expiration)
    }
}

// ── Connection object ───────────────────────────────────────────────────────

/// A Plabble protocol connection handle.
#[derive(uniffi::Object)]
pub struct PlabbleConnection {
    inner: Mutex<InnerPlabbleConnection>,
    tx: Receiver<Vec<u8>>,
    rx: Sender<Vec<u8>>,
}

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
    pub async fn set_key_provider(&self, provider: Box<dyn SessionKeyProvider>) {
        let mut inner = self.inner.lock().await;
        let data = inner.config.data.as_mut().unwrap();

        let provider: Arc<dyn SessionKeyProvider> = Arc::from(provider);
        data.key_provider = Some(Arc::new(KeyProviderBridge::new(provider)));
    }

    /// Send a request packet serialized as a JSON (or TOML) string.
    pub async fn send_request(&self, request: String) -> Result<(), PlabbleProtocolError> {
        let packet = deserialize_input(&request)?;
        let mut inner = self.inner.lock().await;
        inner.send_request(packet).await
    }

    /// Send a request packet and wait for the associated response, returning it as a JSON (or TOML) string.
    pub async fn send_and_recv(&self, request: String) -> Result<String, PlabbleProtocolError> {
        let packet = deserialize_input(&request)?;
        let mut inner = self.inner.lock().await;
        let response = inner.send_and_recv(packet).await?;
        serialize_output(&response)
    }

    /// Wait for the next incoming response packet and return it as a JSON (or TOML) string.
    pub async fn recv_response(&self) -> Result<String, PlabbleProtocolError> {
        let mut inner = self.inner.lock().await;
        let response = inner.recv_response().await?;
        serialize_output(&response)
    }

    /// Feed raw incoming bytes received from the transport layer into the connection.
    pub fn handle_incoming(&self, bytes: Vec<u8>) -> Result<(), PlabbleProtocolError> {
        self.rx
            .try_send(bytes)
            .map_err(|_| PlabbleProtocolError::SenderError)
    }

    /// Start a new session with the given options serialized as a JSON (or TOML) string. Returns the PSK ID as 12-byte array if a pre-shared key is created.
    pub async fn start_session(
        &self,
        options: Option<String>,
    ) -> Result<Option<Vec<u8>>, PlabbleProtocolError> {
        let options = options.map(|opts| deserialize_input(&opts)).transpose()?;
        let mut inner = self.inner.lock().await;
        let psk_id = inner.start_session(options).await?;
        Ok(psk_id.map(|id| id.to_vec()))
    }

    /// Poll for the next outgoing packet (non-blocking).
    /// Returns the raw bytes to send over the transport, or None if nothing is queued.
    pub async fn poll_outgoing(&self) -> Option<Vec<u8>> {
        self.tx.recv().await.ok()
    }
}

#[cfg(feature = "server")]
#[uniffi::export]
impl PlabbleConnection {
    /// Send a response packet serialized as a JSON (or TOML) string.
    pub async fn send_response(&self, response: String) -> Result<(), PlabbleProtocolError> {
        let packet = deserialize_input(&response)?;
        let mut inner = self.inner.lock().await;
        inner.send_response(packet).await
    }

    /// Wait for the next incoming request packet and return it as a JSON (or TOML) string.
    pub async fn recv_request(&self) -> Result<String, PlabbleProtocolError> {
        let mut inner = self.inner.lock().await;
        let request = inner.recv_request().await?;
        serialize_output(&request)
    }
}

// ── Free functions ──────────────────────────────────────────────────────────

/// Returns the library version string.
#[uniffi::export]
pub fn plabble_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
