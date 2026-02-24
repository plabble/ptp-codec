use async_channel::{Receiver, Sender};
use binary_codec::SerializerConfig;
pub mod error;

use crate::{packets::{context::PlabbleConnectionContext, request::PlabbleRequestPacket, response::PlabbleResponsePacket}};

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "client")]
pub mod client;

pub struct PlabbleConnection {
    config: SerializerConfig<PlabbleConnectionContext>,
    req_sender: Sender<PlabbleRequestPacket>,
    req_receiver: Receiver<PlabbleRequestPacket>,
    res_sender: Sender<PlabbleResponsePacket>,
    res_receiver: Receiver<PlabbleResponsePacket>,
}

