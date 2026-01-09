use std::cell::RefCell;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::{core::BucketId, packets::header::type_and_flags::RequestPacketType};

/// Plabble Packet request header
#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlabbleRequestHeader {
    /// Internal packet type field for binary serialization/deserialization
    #[serde(skip_serializing, skip_deserializing)]
    #[bits = 4]
    #[variant_for("packet_type")]
    _type: RefCell<u8>,

    /// Packet type (derived from `_type`)
    #[variant_by = "packet_type"]
    #[serde(flatten)]
    pub packet_type: RequestPacketType,

    /// ID of the bucket, if needed by the type
    #[toggled_by_variant = "packet_type=2"]
    pub id: Option<BucketId>,
}

impl PlabbleRequestHeader {
    /// Create new packet header for specific request packet type
    pub fn new(packet_type: RequestPacketType, id: Option<BucketId>) -> Self {
        Self {
            _type: RefCell::new(packet_type.get_discriminator()),
            packet_type,
            id,
        }
    }

    pub fn preprocess(&self) {
        self._type.replace(self.packet_type.get_discriminator());
    }
}
