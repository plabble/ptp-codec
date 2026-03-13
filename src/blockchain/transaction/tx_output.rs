use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::scripting::opcode_script::OpcodeScript;

#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionOutput {
    /// Indicates whether this output is a monetary transaction output (true) or an asset transfer output (false)
    #[toggles("monetary")]
    is_monetary: bool,

    /// If set to true, the output is burned and can never be spent
    #[toggles("burn")]
    burn: bool,

    /// If set to true, this specific output is not replaceable by fee (RBF) and must be included in the replacement transaction as is
    not_replaceable: bool,

    // 7 bits reserved for future use

    /// Value of the transaction output in the smallest unit
    value: OutputType,

    /// Locking script to lock this output
    #[toggled_by = "!burn"]
    #[dyn_length]
    locking_script: Option<OpcodeScript>,
}

#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[no_discriminator]
pub enum OutputType {
    /// Monetary transaction output, which represents a transfer in smallest currency unit
    #[toggled_by = "monetary"]
    Monetary(#[dyn_int] u128),

    /// Asset transfer output, which represents a transfer of a specific asset on the blockchain
    #[toggled_by = "!monetary"]
    Asset([u8; 24])
}