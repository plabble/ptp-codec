use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::core::BucketId;
use crate::core::PlabbleDateTime;
use crate::crypto::algorithm::CryptoSignature;
use crate::network::node_info::NodeInfo;
use crate::packets::body::bucket::{BucketQuery, PutRequestBody};
use crate::packets::body::post::PostRequestBody;

/// Whisper metadata for conflict resolving
///
/// If the version is higher: accept
/// If the version is equal: accept if timestamp is lower (first come first serve)
/// If the version is lower: reject
/// If the version is equal and timestamp is equal: accept if node ID is higher (to break ties)
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WhisperMetadata {
    /// If applicable, indicates the keys in the message are in binary format (String)
    #[toggles("binary_keys")]
    pub binary_keys: bool,

    /// Whether the message has a "from" field (some messages may be anonymous)
    #[toggles("has_from")]
    pub has_from: bool,

    /// Node ID of the sender (same as certificate ID)
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    #[toggled_by = "has_from"]
    pub from: Option<[u8; 16]>,

    /// Version number for conflict resolution
    #[dyn_int]
    pub version: u32,

    /// Message timestamp (when it was sent)
    pub timestamp: PlabbleDateTime,

    /// Signatures by the sender to ensure authenticity and integrity of the message
    #[multi_enum]
    pub signatures: Vec<CryptoSignature>,
}

/// Whisper request body, used for server<->server messaging
#[repr(u8)]
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Clone)]
#[no_discriminator]
pub enum WhisperRequestBody {
    /// Ping other nodes to check if they are alive (with random number)
    Ping(u8) = 0,

    /// Broadcasted when a new node appears in the network
    Hello(NodeInfo) = 1,

    /// Asking the network who is a specific node id
    WhoIs(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 16]) = 2,

    /// Telling other nodes about a new bucket
    NewBucket(PostRequestBody, WhisperMetadata) = 3,

    /// Telling other nodes to change the content of a slot in the bucket (with conflict resolution)
    PutSlot(BucketId, PutRequestBody, WhisperMetadata) = 4,

    /// Telling other nodes to delete a slot in the bucket (with conflict resolution)
    DeleteSlot(BucketId, BucketQuery, WhisperMetadata) = 5,
}

/// Whisper response body, used for server<->server messaging
#[repr(u8)]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Clone)]
#[no_discriminator]
pub enum WhisperResponseBody {
    /// Response to a ping request, with the same random number (should match)
    Pong(u8) = 0,

    /// Response to a hello message
    Hello = 1,

    /// Response to a WhoIs request
    WhoIs(Option<NodeInfo>) = 2,

    /// Acknowledgment for a new bucket message (true if accepted, false if rejected)
    NewBucketAck(bool) = 3,

    /// Acknowledgment for a put slot message (true if accepted, false if rejected)
    PutSlotAck(bool) = 4,

    /// Acknowledgment for a delete slot message (true if accepted, false if rejected)
    DeleteSlotAck(bool) = 5,
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddrV4};

    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::{core::{PlabbleDateTime, node_address::NodeAddress}, crypto::algorithm::VerificationKey, network::node_info::NodeInfo, packets::{body::request_body::PlabbleRequestBody, header::{request_header::PlabbleRequestHeader, type_and_flags::RequestPacketType}, request::PlabbleRequestPacket}};

    #[test]
    fn can_serialize_and_deserialize_ping() {
        let request: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Whisper"

            [body]
            Ping = 42
        "#).unwrap();

        assert_eq!(request.header.packet_type, RequestPacketType::Whisper { whisper_type: 0 });
        let bytes = request.to_bytes(None).unwrap();
        assert_eq!("01092a", hex::encode(&bytes));

        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_hello() {
        let req: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Whisper"

            [body.Hello]
            id = "AQEBAQEBAQEBAQEBAQEBAQ"
            address.V4 = "127.0.0.1:1234"
            last_seen = "2161-02-07T06:28:15Z"
            
            [[body.Hello.verification_keys]]
            Ed25519 = "yX8-B6lkBke5guSLzDWbasSLRQ524mUq7YezQz4YeVU"
        "#,
        )
        .unwrap();

        println!("{}", toml::to_string(&req).unwrap());

        let vkey = "c97f3e07a9640647b982e48bcc359b6ac48b450e76e2652aed87b3433e187955";
    }

    /*
    #[test]
    fn can_serialize_and_deserialize_whois_request() {
        let req = PlabbleRequestPacket {
            base: Default::default(),
            header: PlabbleRequestHeader::new(RequestPacketType::Whisper { whisper_type: 2 }, None),
            body: PlabbleRequestBody::Whisper(crate::packets::body::whisper::WhisperRequestBody::WhoIs([0u8; 16])),
        };

        println!("{}", toml::to_string(&req).unwrap());
        
        let request: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Whisper"

            [body.WhoIs]
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
            format!(
                "0169{}{}{}03ffffffff{}",
                "01".repeat(16),
                hex::encode(&[0b0001_0001]), // type = 0001 (WhoIs), binary_keys = 1
                "02".repeat(16),
                "00".repeat(64)
            ),
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

            [body.Bucket]
            id = "ceR2iLSzPCzaJkVqW2gN3A"
        "#,
        )
        .unwrap();

        let bytes = request.to_bytes(None).unwrap();
        assert_eq!(bytes[1], 0b0010_1001);
        assert_eq!("012971e47688b4b33c2cda26456a5b680ddc", hex::encode(&bytes));

        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }
    */
}
