use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::packets::header::type_and_flags::RequestPacketType;

#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlabbleRequestHeader {
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
    pub fn new(packet_type: RequestPacketType) -> Self {
        Self {
            _type: packet_type.get_discriminator(),
            packet_type,
        }
    }
}
