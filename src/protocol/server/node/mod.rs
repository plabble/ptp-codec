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
pub enum Whisper {
    /// Way of saying "Here I am" or "I am alive"
    /// Broadcasted when a new node appears in the network
    Hineni(NodeInfo) = 0,

    /// Way of asking "who are you?"
    WhoIs(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 16]) = 1,

    /// Telling other nodes about a new bucket
    NewBucket(PostRequestBody) = 2,

    /// Change the value of a slot
    PutSlot {
        #[toggles("binary_keys")]
        binary_keys: bool,
        body: PutRequestBody,
    } = 3,

    /// Append a slot to the bucket (slot number doesn't matter)
    AppendSlot { id: BucketId, data: Vec<u8> } = 4,

    /// Delete. Remember deletion to avoid resurrecting when receiving older messages
    DeleteSlot {
        #[toggles("binary_keys")]
        binary_keys: bool,
        query: BucketQuery,
    } = 5,
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

    /// Whisper message
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

pub struct SlotHistory {
    pub version: u32,
    pub last_update: PlabbleDateTime,
    pub deleted: bool,
}

impl SlotHistory {
    /// Check if the incoming message can be accepted based on version/timestamp/node ID
    pub fn can_accept(&self, incoming: &WhisperMessage, self_id: &[u8; 16]) -> bool {
        // Do not accept clock skewed (> 1 sec future) message
        if incoming.timestamp.timestamp() > PlabbleDateTime::from_now(1).timestamp() {
            return false;
        }

        // Do not allow skewed versions (+10 versions in the future)
        if incoming.version > self.version + 10 {
            return false;
        }

        // Do not accept older versions
        if incoming.version < self.version {
            return false;
        }

        // If version is equal (conflict), try to resolve
        if incoming.version == self.version {
            // If the incoming message is newer, reject (first come first serve)
            if incoming.timestamp.timestamp() > self.last_update.timestamp() {
                return false;
            }

            if incoming.timestamp.timestamp() == self.last_update.timestamp() {
                // If the timestamps are equal, break ties by node ID (higher wins)
                if &incoming.from < self_id {
                    return false;
                }
            }
        }

        true
    }
}
