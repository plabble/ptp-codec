pub mod node_address;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::{
    core::{BucketId, PlabbleDateTime},
    crypto::algorithm::{CryptoSignature, VerificationKey},
    packets::{
        base::settings::CryptoSettings,
        body::{
            bucket::{BucketQuery, PutRequestBody},
            post::PostRequestBody,
        },
    },
    protocol::server::node::node_address::NodeAddress,
};

#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NodeInfo {
    /// Node identifier (same as certificate ID)
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub id: [u8; 16],

    /// Node address (IP and port)
    pub address: NodeAddress,

    /// Last seen timestamp
    pub last_seen: PlabbleDateTime,

    /// Crypto settings used by the node (e.g., supported algorithms)
    pub crypto_settings: CryptoSettings,

    /// Public keys
    #[multi_enum]
    pub verification_keys: Vec<VerificationKey>,
}

#[serde_as]
#[repr(u8)]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[discriminator_bits = 4]
pub enum Whisper {
    /// Broadcasted when a new node appears in the network
    Hello(NodeInfo) = 0,

    /// Asking the network who is a specific node id
    WhoIs(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 16]) = 1,

    /// Telling other nodes about a new bucket
    NewBucket(PostRequestBody) = 2,

    /// Change the value of a slot
    PutSlot(PutRequestBody) = 3,

    /// Append a slot to the bucket (slot number doesn't matter)
    AppendSlot { id: BucketId, data: Vec<u8> } = 4,

    /// Delete. Remember deletion to avoid resurrecting when receiving older messages
    DeleteSlot(BucketQuery) = 5,
}

/// Whisper message with conflict resolving
///
/// If the version is higher: accept
/// If the version is equal: accept if timestamp is lower (first come first serve)
/// If the version is lower: reject
/// If the version is equal and timestamp is equal: accept if node ID is higher (to break ties)
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WhisperMessage {
    /// Node ID of the sender (same as certificate ID)
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub from: [u8; 16],
    
    /// If applicable, indicates the keys in the message are in binary format (String)
    #[toggles("binary_keys")]
    pub binary_keys: bool,

    /// Whisper message
    #[skip_bits(3)] // 3 reserved bits for future flags
    pub message: Whisper,

    /// Version number for conflict resolution
    #[dyn_int]
    pub version: u32,

    /// Message timestamp (when it was sent)
    pub timestamp: PlabbleDateTime,

    /// Signatures by the sender to ensure authenticity and integrity of the message
    #[multi_enum]
    pub signatures: Vec<CryptoSignature>,
}