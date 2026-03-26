use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::formats::Lowercase;
use serde_with::hex::Hex;
use serde_with::serde_as;

use crate::scripting::opcode_script::OpcodeScript;

#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionOutput {
    /// Indicates whether this output is a monetary transaction output (true) or an asset transfer output (false)
    #[toggles("monetary")]
    pub is_monetary: bool,

    /// If set to true, the output is burned and can never be spent or moved anymore
    #[toggles("burn")]
    pub burn: bool,

    /// If set to true, this specific output is not replaceable by fee (RBF) and must be included in the replacement transaction as is
    pub not_replaceable: bool,

    // 5 bits reserved for future use
    /// Value of the transaction output in the smallest unit
    #[variant_by = "monetary"]
    pub value: OutputType,

    /// Locking script to lock this output
    #[toggled_by = "!burn"]
    #[dyn_length]
    pub locking_script: Option<OpcodeScript>,
}

#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[no_discriminator]
pub enum OutputType {
    /// Asset transfer output, which represents a transfer of a specific asset on the blockchain
    /// But also for instance, to pay for a smart contract that will be placed in the same block
    Asset(#[serde_as(as = "Hex<Lowercase>")] [u8; 24]),

    /// Monetary transaction output, which represents a transfer in smallest currency unit
    Monetary(#[dyn_int] u64),
}
