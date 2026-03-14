pub mod block_data;
pub mod block_header;
pub mod block_proof;

use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::formats::Lowercase;
use serde_with::hex::Hex;
use serde_with::serde_as;

use crate::blockchain::block::{block_data::BlockData, block_header::BlockHeader};

/// A block structure for the Plabble Blockchain
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Block {
    /// Block header/metadata
    pub header: BlockHeader,

    /// Block entries/data, mapping from entry hash to the actual data
    #[serde_as(as = "HashMap<Hex<Lowercase>, _>")]
    #[val_dyn_length]
    pub data: HashMap<[u8; 24], BlockData>,
}

impl Block {
    /// Get hashes of all entries in the block, sorted in ascending order.
    pub fn get_hashes(&self) -> Vec<[u8; 24]> {
        let mut hashes: Vec<[u8; 24]> = self.data.keys().cloned().collect();
        hashes.sort();
        hashes
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    use crate::{
        blockchain::block::{
            Block,
            block_data::BlockData,
            block_header::BlockHeader,
            block_proof::{BlockProof, Difficulty},
        },
        core::PlabbleDateTime,
    };

    #[test]
    fn can_serialize_and_deserialize_block() {
        let mut data = HashMap::new();
        let mut proofs = Vec::new();

        data.insert([2u8; 24], BlockData::Blob(vec![1, 2, 3, 4, 5]));
        data.insert([4u8; 24], BlockData::Resource(String::from("test")));

        proofs.push(BlockProof::ProofOfWork {
            nonce: 0,
            target: Difficulty {
                exponent: 3,
                coefficient: [1, 2, 3],
            },
        });

        let block = Block {
            header: BlockHeader {
                version: 7,
                has_proof_of_work: true,
                prev_block_hash: [0; 24],
                merkle_root: [1; 24],
                timestamp: PlabbleDateTime::new(10),
                proofs,
            },
            data,
        };

        let config: Option<&mut SerializerConfig> = None;
        let serialized = block.to_bytes(config).unwrap();

        assert_eq!(serialized[0], 0b0001_0111); // version and flags
        // 24x 00
        // 24x 01
        // timestamp (10) = 0000000a
        // 00 = nonce
        // 03 = exponent, 01 02 03 = coefficient
        // 24x 02 (entry hash)
        // 06 (length of entry data)
        // ff (entry type = blob)
        // 01 02 03 04 05 (blob data)
        // 32x 04 (entry hash)
        // 05 (length of entry data)
        // 03 (entry type = resource)
        // 74 65 73 74 (resource data = "test")
        let option1 = "170000000000000000000000000000000000000000000000000101010101010101010101010101010101010101010101010000000a000301020302020202020202020202020202020202020202020202020206ff0102030405040404040404040404040404040404040404040404040404050374657374";
        let option2 = "170000000000000000000000000000000000000000000000000101010101010101010101010101010101010101010101010000000a000301020304040404040404040404040404040404040404040404040405037465737402020202020202020202020202020202020202020202020206ff0102030405";
        let hex = hex::encode(&serialized);
        assert!(hex == option1 || hex == option2);

        let config: Option<&mut SerializerConfig> = None;
        let deserialized = Block::from_bytes(&serialized, config).unwrap();
        assert_eq!(block, deserialized);

    }
}
