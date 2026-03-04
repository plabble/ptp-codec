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
pub struct PlabbleConnection {
    hooks: HashMap<u16, Sender<PlabbleResponsePacket>>,
    pub config: SerializerConfig<PlabbleConnectionContext>,
    pub tx: Sender<Vec<u8>>,
    pub rx: Receiver<Vec<u8>>,
}
