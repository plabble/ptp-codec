use binary_codec::SerializerConfig;

use crate::packets::{
    base::PlabblePacketBase,
    header::{request_header::PlabbleRequestHeader, response_header::PlabbleResponseHeader},
};

pub mod bucket;
pub mod request_body;
pub mod response_body;
pub mod session;

/// Context information required for serializing request packets.
/// Includes references to the packet header, base packet, and serializer configuration.
/// 
/// # Fields
/// - `header`: Reference to the request packet header.
/// - `packet`: Reference to the base packet structure.
/// - `config`: Serializer configuration settings.
#[derive(Clone)]
pub struct RequestSerializationContext<'a> {
    pub header: &'a PlabbleRequestHeader,
    pub packet: &'a PlabblePacketBase,
    pub config: SerializerConfig,
}

/// Context information required for serializing response packets.
/// Includes references to the packet header, base packet, and serializer configuration.
/// 
/// # Fields
/// - `header`: Reference to the response packet header.
/// - `packet`: Reference to the base packet structure.
/// - `config`: Serializer configuration settings.
#[derive(Clone)]
pub struct ResponseSerializationContext<'a> {
    pub header: &'a PlabbleResponseHeader,
    pub packet: &'a PlabblePacketBase,
    pub config: SerializerConfig,
}
