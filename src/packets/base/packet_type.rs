use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Plabble Protocol packet header
#[derive(FromBytes, ToBytes, Deserialize, Serialize, PartialEq, Debug)]
pub struct PlabblePacketHeader {
    /// Packet type
    #[serde(skip_serializing, skip_deserializing)]
    #[bits = 4]
    #[variant_for("packet_type")]
    _type: u8,

    /// Packet type (derived from `_type`)
    #[variant_by = "packet_type"]
    packet_type: PacketType,

    /// Packet flags, specific to the packet type
    flags: [bool; 4],

    /// If in a session, the counter of the request to respond to
    response_to: Option<u16>
}

impl PlabblePacketHeader {
    /// Pre-process the header before binary serialization
    pub fn pre_process(mut self) -> Self {
        self._type = self.packet_type as u8;
        self
    }
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinarySerializer, BinaryDeserializer};
    use super::*;
    
    #[test]
    fn can_serialize_packet_header() {
        let toml = r#"
        packet_type = "Session"
        flags = [false, true, true, false] # flags are specific to the type
        response_to = 123
        "#;

        let header: PlabblePacketHeader = toml::from_str(toml).unwrap();
        let header = header.pre_process();
        println!("{:?}", header);
        let bytes = header.to_bytes(None).unwrap();
        assert_eq!(vec![0b0110_0001, 0, 123], bytes);

        let deserialized_header = PlabblePacketHeader::from_bytes(&bytes, None).unwrap();
        assert_eq!(deserialized_header.packet_type, PacketType::Session);
        assert_eq!(header, deserialized_header);
    }
}