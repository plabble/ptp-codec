use std::collections::HashMap;

use async_channel::{Receiver, Sender};
use binary_codec::SerializerConfig;

pub mod error;

use crate::packets::{context::PlabbleConnectionContext, response::PlabbleResponsePacket};

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "client")]
pub mod client;

/// Plabble Connection
// #[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
// #[cfg_attr(feature = "ffi", derive(uniffi::Object))]
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