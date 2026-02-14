use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Plabble error code body
/// The length is prefixed by a u8 in the packet body.
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[repr(u8)]
pub enum PlabbleError {
    /* generic errors: 0-10 */
    /// The requested protocol version is not supported by this implementation.
    /// Contains the min and max version the server supports.
    UnsupportedVersion { min_version: u8, max_version: u8 } = 0,
    /// The requested algorithm in crypto settings is not supported by the server
    UnsupportedAlgorithm {
        #[dyn_length]
        name: String,
    } = 1,

    /// The requested CUSTOM packet type is not supported by the server
    UnsupportedSubProtocol = 2,

    /* bucket errors: 10-100 */
    /// Bucket by ID not found (or existence denied)
    BucketNotFound = 10,

    /// Bucket with that ID already exists
    BucketAlreadyExists = 11,

    /* certificate errors: 110-115 */
    /// Certificate by ID not found
    CertificateNotFound = 110,
    /// Requested certificate is not valid (according to server)
    CertificateInvalid = 111,
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

            [body]
            type = "UnsupportedVersion"
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

            [body]
            type = "UnsupportedAlgorithm"
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
