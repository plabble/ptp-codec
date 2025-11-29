use binary_codec::SerializerConfig;

use crate::packets::{
    base::PlabblePacketBase,
    header::{request_header::PlabbleRequestHeader, response_header::PlabbleResponseHeader},
};

pub mod bucket;
pub mod request_body;
pub mod response_body;
pub mod session;

#[derive(Clone)]
pub struct RequestSerializationContext<'a> {
    pub header: &'a PlabbleRequestHeader,
    pub packet: &'a PlabblePacketBase,
    pub config: SerializerConfig,
}

#[derive(Clone)]
pub struct ResponseSerializationContext<'a> {
    pub header: &'a PlabbleResponseHeader,
    pub packet: &'a PlabblePacketBase,
    pub config: SerializerConfig,
}
