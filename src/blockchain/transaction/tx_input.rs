use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::formats::Lowercase;
use serde_with::hex::Hex;
use serde_with::serde_as;

use crate::scripting::opcode_script::OpcodeScript;

/// A transaction input structure for a Plabble Transaction
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionInput {
    /// Reference to the previous transaction ID
    #[serde_as(as = "Hex<Lowercase>")]
    pub transaction_id: [u8; 24],

    /// Index of the output in the previous transaction that is being spent
    #[dyn_int]
    pub output_index: u64,

    /// Unlocking script to unlock the referenced output, allowing it to be transferred in this transaction
    #[dyn_length]
    pub unlocking_script: OpcodeScript,
}
