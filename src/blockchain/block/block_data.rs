use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::formats::Lowercase;
use serde_with::hex::Hex;
use serde_with::serde_as;

use crate::blockchain::contract::SmartContract;
use crate::blockchain::transaction::Transaction;
use crate::crypto::certificate::Certificate;

/// Data that can be stored on the Plabble Blockchain
///
/// In general, blockchain storage is expensive.
/// Therefore, most nodes will NOT accept any data that is not somehow (at the same time or in advance) referenced by a transaction
/// However, we want to allow the possibility to store data on the blockchain without a transaction, for instance Plabble Certificates which might be free
#[repr(u8)]
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum BlockData {
    /// Coinbase transaction
    Coinbase(Transaction) = 0,

    /// Transaction / payment
    Transaction(Transaction) = 1,

    /// Existence proof hash (e.g. file, image, etc.) stored outside the blockchain, to prove existence
    /// This is in general something nobody can verify
    Hash(#[serde_as(as = "Hex<Lowercase>")] [u8; 32]) = 2,

    /// Resource hash (e.g. file, image, etc.) stored outside the blockchain, to prove existence
    /// Contains URL
    Resource(String) = 3,

    /// Smart contract that automatically executes when certain conditions are met
    SmartContract(SmartContract) = 4,

    /// Plabble Certificate, which is a proof of identity and reputation for users on the Plabble platform
    Certificate(Certificate) = 5,

    /// Arbitrary binary data
    Blob(#[serde_as(as = "Hex<Lowercase>")] Vec<u8>) = 255,
}
