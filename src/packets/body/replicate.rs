use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

#[cfg(feature = "server")]
use crate::protocol::server::node::WhisperMessage;

/// Replicate request body, used for both bucket replication and state updates
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Clone)]
#[no_discriminator]
pub enum ReplicateRequestBody {
    /// Replicate a bucket to other nodes
    Bucket(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 16]),

    /// State update message, used for synchronizing state across nodes (e.g., new buckets, slot updates)
    #[cfg(feature = "server")]
    StateUpdate(WhisperMessage),

    /// State update not supported (client builds)
    #[cfg(not(feature = "server"))]
    Unsupported,
}

/// Replicate response body, used for both bucket replication and state updates
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Clone)]
#[no_discriminator]
pub enum ReplicateResponseBody {
    /// Replication started
    Ok,

    /// State update message, used for synchronizing state across nodes (e.g., new buckets, slot updates)
    #[cfg(feature = "server")]
    StateUpdate(WhisperMessage),

    /// State update not supported (client builds)
    #[cfg(not(feature = "server"))]
    Unsupported,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::request::PlabbleRequestPacket;

    #[test]
    fn can_serialize_and_deserialize_replicate_request_with_state_update() {
        let request: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Replicate"
            state_update = true

            [body.StateUpdate]
            from = "AQEBAQEBAQEBAQEBAQEBAQ"
            binary_keys = true
            version = 3
            timestamp = "2161-02-07T06:28:15Z"

            [body.StateUpdate.message]
            WhoIs = "AgICAgICAgICAgICAgICAg"
            
            [[body.StateUpdate.signatures]]
            Ed25519 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"
        "#,
        )
        .unwrap();

        let bytes = request.to_bytes(None).unwrap();
        assert_eq!(bytes[1], 0b0110_1001);
        assert_eq!(
            format!("0169{}{}{}03ffffffff{}", 
            "01".repeat(16),
            hex::encode(&[0b0001_0001]), // type = 0001 (WhoIs), binary_keys = 1
            "02".repeat(16),
            "00".repeat(64)),
            hex::encode(&bytes)
        );

        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_replicate_request_with_bucket() {
        let request: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Replicate"
            state_update = false
            mirror = true

            [body]
            Bucket = "ceR2iLSzPCzaJkVqW2gN3A"
        "#,
        )
        .unwrap();

        let bytes = request.to_bytes(None).unwrap();
        assert_eq!(bytes[1], 0b0010_1001);
        assert_eq!(
            "012971e47688b4b33c2cda26456a5b680ddc",
            hex::encode(&bytes)
        );

        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }
}

