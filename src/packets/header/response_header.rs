use std::cell::RefCell;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::packets::header::type_and_flags::ResponsePacketType;

/// Plabble Packet response header
#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlabbleResponseHeader {
    /// Internal packet type field for binary serialization/deserialization
    #[serde(skip_serializing, skip_deserializing)]
    #[bits = 4]
    #[variant_for("packet_type")]
    _type: RefCell<u8>,

    /// Packet type (derived from `_type`)
    #[variant_by = "packet_type"]
    #[serde(flatten)]
    pub packet_type: ResponsePacketType,

    /// Counter of request to reply to, if in session
    #[toggled_by = "!fire_and_forget"]
    pub request_counter: Option<u16>,
}

impl PlabbleResponseHeader {
    /// Create new packet header for specific response packet type
    pub fn new(packet_type: ResponsePacketType, counter: Option<u16>) -> Self {
        Self {
            _type: RefCell::new(packet_type.get_discriminator()),
            packet_type,
            request_counter: counter,
        }
    }

    /// Indicates if SESSION packet
    pub fn is_session_packet(&self) -> bool {
        matches!(self.packet_type, ResponsePacketType::Session {..})
    }

    pub fn preprocess(&self) {
        self._type.replace(self.packet_type.get_discriminator());
    }
}
