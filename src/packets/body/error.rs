use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Plabble error code body
/// The length is prefixed by a u8 in the packet body.
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub enum PlabbleError {
    /// The requested protocol version is not supported by this implementation.
    /// Contains the min and max version the server supports.
    UnsupportedVersion { min_version: u8, max_version: u8 },
    UnsupportedAlgorithm {
        #[dyn_length]
        name: String,
    },
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::response::PlabbleResponsePacket;

    #[test]
    fn can_serialize_and_deserialize_unsupported_version_error_response() {
        let response: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Error"
            request_counter = 1

            [body.UnsupportedVersion]
            min_version = 1
            max_version = 3
        "#,
        )
        .unwrap();

        let serialized = response.to_bytes(None).unwrap();
        let deserialized = PlabbleResponsePacket::from_bytes(&serialized, None).unwrap();

        // Version = 0001, flags = 0100 Packet type: 15 = 1111, flags = 0000. Counter = 01, Error type = 0, min version = 1, max version = 3
        assert_eq!(vec![0b0100_0001, 0b0000_1111, 0, 1, 0, 1, 3], serialized);
        assert_eq!(response, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_unsupported_algorithm_error_response() {
        let response: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Error"
            request_counter = 1

            [body.UnsupportedAlgorithm]
            name = "Ed25519"
        "#,
        )
        .unwrap();

        let serialized = response.to_bytes(None).unwrap();
        let deserialized = PlabbleResponsePacket::from_bytes(&serialized, None).unwrap();

        // Version = 0001, flags = 0100 Packet type: 15 = 1111, flags = 0000. Counter = 01, Error type = 1, name length = 7, name = "Ed25519"
        assert_eq!(
            vec![
                0b0100_0001,
                0b0000_1111,
                0,
                1,
                1,
                7,
                b'E',
                b'd',
                b'2',
                b'5',
                b'5',
                b'1',
                b'9'
            ],
            serialized
        );

        assert_eq!(response, deserialized);
    }
}
