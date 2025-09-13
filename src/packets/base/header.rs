use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Plabble Protocol packet header
#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlabblePacketHeader {
    /// Packet type
    #[bits = 4]
    packet_type: u8,

    /// Packet flags, specific to the packet type
    packet_flags: [bool; 4],

    /// If in a session, the counter of the request to respond to
    response_to: Option<u16>
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn can_serialize_packet_header() {
        let toml = r#"
        packet_type = 1
        packet_flags = [false, true, true, false]
        response_to = 123
        "#;

        let header: PlabblePacketHeader = toml::from_str(toml).unwrap();
        let bytes = header.to_bytes().unwrap();

        let deserialized_header = PlabblePacketHeader::from_bytes(&bytes).unwrap();
        assert_eq!(header, deserialized_header);
        assert_eq!(vec![0b0110_0001, 0, 123], bytes);
    }
}