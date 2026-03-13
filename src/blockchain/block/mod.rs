pub mod block_header;
pub mod block_data;
pub mod block_proof;

use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::hex::Hex;
use serde_with::formats::Lowercase;
use serde_with::serde_as;

use crate::blockchain::block::{block_data::BlockData, block_header::BlockHeader};

/// A block structure for the Plabble Blockchain
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Block {
    /// Block header/metadata
    header: BlockHeader,

    /// Block entries/data, mapping from entry hash to the actual data
    #[serde_as(as = "HashMap<Hex<Lowercase>, _>")]
    data: HashMap<[u8; 32], BlockData>,
}

