use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::scripting::opcode_script::ScriptError;

/// Plabble error code body
/// The length is prefixed by a u8 in the packet body.
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Error))]
#[serde(tag = "type")]
#[repr(u8)]
pub enum PlabbleError {
    /* 0 = no error, but it is NOT used */

    /* generic errors: 1-10 */
    /// The requested protocol version is not supported by this implementation.
    /// Contains the min and max version the server supports.
    UnsupportedVersion { min_version: u8, max_version: u8 } = 1,
    /// The requested algorithm in crypto settings is not supported by the server
    UnsupportedAlgorithm {
        #[dyn_length]
        name: String,
    } = 2,

    /// The requested CUSTOM packet type is not supported by the server
    UnsupportedSubProtocol = 3,

    /// The request is not valid
    InvalidRequest = 4,

    /// The operation requires an established, authenticated session.
    AuthenticationRequired = 5,

    /// The authenticated caller is not allowed to perform the operation.
    PermissionDenied = 6,

    /* bucket errors: 10-100 */
    /// Bucket by ID not found (or existence denied)
    BucketNotFound = 10,

    /// Bucket with that ID already exists
    BucketAlreadyExists = 11,

    /// An asserted key already exists and therefore cannot be appended.
    KeyAlreadyExists = 12,

    /// The requested bucket setting or ACL is locked.
    BucketLocked = 13,

    /// No matching active subscription exists.
    SubscriptionNotFound = 14,

    /* certificate errors: 110-115 */
    /// Certificate by ID not found
    CertificateNotFound = 110,
    /// Requested certificate is not valid (according to server)
    CertificateInvalid = 111,

    // ...
    /// OPCODE script execution error
    OpcodeScriptError(ScriptError) = 210,

    /// Internal server error / unknown error
    InternalServerError = 255,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::{
        base::PlabblePacketBase,
        body::{error::PlabbleError, response_body::PlabbleResponseBody},
        header::{response_header::PlabbleResponseHeader, type_and_flags::ResponsePacketType},
        response::PlabbleResponsePacket,
    };

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

        // Version = 0001, flags = 0100 Packet type: 15 = 1111, flags = 0000. Counter = 01, Error type = 1, min version = 1, max version = 3
        assert_eq!(vec![0b0100_0001, 0b0000_1111, 0, 1, 1, 1, 3], serialized);
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

        // Version = 0001, flags = 0100 Packet type: 15 = 1111, flags = 0000. Counter = 01, Error type = 2, name length = 7, name = "Ed25519"
        assert_eq!(
            vec![
                0b0100_0001,
                0b0000_1111,
                0,
                1,
                2,
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

    #[test]
    fn bucket_and_authentication_errors_keep_their_wire_codes() {
        for (error, code) in [
            (PlabbleError::AuthenticationRequired, 5),
            (PlabbleError::PermissionDenied, 6),
            (PlabbleError::KeyAlreadyExists, 12),
            (PlabbleError::BucketLocked, 13),
            (PlabbleError::SubscriptionNotFound, 14),
        ] {
            let response = PlabbleResponsePacket {
                base: PlabblePacketBase::default(),
                header: PlabbleResponseHeader::new(ResponsePacketType::Error, Some(0)),
                body: PlabbleResponseBody::Error(error),
            };
            let bytes = response.to_bytes(None).unwrap();
            assert_eq!(bytes[4], code);
            assert_eq!(
                PlabbleResponsePacket::from_bytes(&bytes, None).unwrap(),
                response
            );
        }
    }
}
