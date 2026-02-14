use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Custom request/response body for sub-protocols, containing a protocol ID and raw data
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct CustomBody {
    /// Protocol ID to identify the sub-protocol this body belongs to
    pub protocol: u16,

    /// Raw data for the sub-protocol, which can be parsed according to the protocol ID
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::{request::PlabbleRequestPacket, response::PlabbleResponsePacket};

    #[test]
    fn can_serialize_and_deserialize_custom_request() {
        let packet: PlabbleRequestPacket = toml::from_str(r#"
            version = 1
            
            [header]
            packet_type = "Custom"
            flag1 = true
            flag2 = false
            flag3 = true
            flag4 = false

            [body]
            protocol = 42
            data = [1, 2, 3, 4, 5]
        "#).unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        assert_eq!("01011101", format!("{:08b}", serialized[1]));
        assert_eq!("015d002a0102030405", hex::encode(&serialized));

        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();
        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_custom_response() {
        let packet: PlabbleResponsePacket = toml::from_str(r#"
            version = 1
            
            [header]
            packet_type = "Custom"
            request_counter = 7
            flag1 = true
            flag2 = false
            flag3 = true
            flag4 = false

            [body]
            protocol = 42
            data = [1, 2, 3, 4, 5]
        "#).unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        assert_eq!("01011101", format!("{:08b}", serialized[1]));
        assert_eq!("015d0007002a0102030405", hex::encode(&serialized));

        let deserialized = PlabbleResponsePacket::from_bytes(&serialized, None).unwrap();
        assert_eq!(packet, deserialized);
    }
}