use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::hex::Hex;
use serde_with::formats::Lowercase;
use serde_with::serde_as;

use crate::blockchain::block::block_proof::BlockProof;
use crate::core::PlabbleDateTime;

#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct BlockHeader {
    /// Block version number, used for compatibility checks and future upgrades
    #[bits(4)]
    version: u8,

    /// Indicates if this block contains a proof-of-work
    #[toggles("proof_of_work")]
    has_proof_of_work: bool,

    /// Merkle root of the previous block, linking this block to the blockchain
    #[serde_as(as = "Hex<Lowercase>")]
    prev_block_hash: [u8; 32],
    
    /// Merkle root of the items/entries included in the block, ensuring data integrity
    #[serde_as(as = "Hex<Lowercase>")]
    merkle_root: [u8; 32], 

    /// Timestamp of when the block was created
    timestamp: PlabbleDateTime,

    /// Proofs that validate the block
    #[multi_enum]
    proofs: Vec<BlockProof>,
}