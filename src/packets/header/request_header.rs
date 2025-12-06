use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::packets::header::type_and_flags::RequestPacketType;

/// Plabble Packet request header
#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlabbleRequestHeader {
    /// Internal packet type field for binary serialization/deserialization
    #[serde(skip_serializing, skip_deserializing)]
    #[bits = 4]
    #[variant_for("packet_type")]
    _type: u8,

    /// Packet type (derived from `_type`)
    #[variant_by = "packet_type"]
    #[serde(flatten)]
    pub packet_type: RequestPacketType,
}

impl PlabbleRequestHeader {
    /// Create new packet header for specific request packet type
    pub fn new(packet_type: RequestPacketType) -> Self {
        Self {
            _type: packet_type.get_discriminator(),
            packet_type,
        }
    }

    pub fn preprocess(&mut self) {
        self._type = self.packet_type.get_discriminator();
    }
}
