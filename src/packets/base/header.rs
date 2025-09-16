use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Plabble Protocol packet types
#[derive(ToBytes, FromBytes, Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
#[no_disc_prefix]
pub enum PacketType {
    Certificate = 0,
    Session = 1,
    Get = 2,
    Stream = 3,
    Post = 4,
    Patch = 5,
    Put = 6,
    Delete = 7,
    Subscribe = 8,
    Unsubscribe = 9,
    Register = 10,
    Identify = 11,
    Proxy = 12,
    _Reserved13 = 13,
    _Reserved14 = 14,
    Error = 15
}

/// Plabble Protocol packet header
#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlabblePacketHeader {
    /// Packet type
    #[serde(skip_serializing, skip_deserializing)]
    #[bits = 4]
    _type: u8,

    /// Packet type (derived from `_type`)
    #[variant_by = "_type"]
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
        let bytes = header.to_bytes().unwrap();

        let deserialized_header = PlabblePacketHeader::from_bytes(&bytes).unwrap();
        assert_eq!(deserialized_header.packet_type, PacketType::Session);
        assert_eq!(header, deserialized_header);
        assert_eq!(vec![0b0110_0001, 0, 123], bytes);
    }
}