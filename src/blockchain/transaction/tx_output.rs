use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::formats::Lowercase;
use serde_with::hex::Hex;
use serde_with::serde_as;

use crate::scripting::opcode_script::OpcodeScript;

/// Transaction output
#[repr(u8)]
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum TransactionOutput {
    /// Monetary transaction output
    Monetary {
        /// Amount (in smallest currency unit)
        #[dyn_int]
        amount: u64,

        /// Locking script
        #[dyn_length]
        lock: OpcodeScript
    } = 0,

    /// Transfer ownership of an asset on the blockchain
    Asset {
        /// Asset identifier
        #[serde_as(as = "Hex<Lowercase>")]
        id: [u8; 24],

        /// Locking script
        #[dyn_length]
        lock: OpcodeScript
    } = 1,

    /// Deploy a smart contract - cannot be used as an input
    DeployContract(#[serde_as(as = "Hex<Lowercase>")] [u8; 24]) = 2,

    /// Invoke a on-chain smart contract - cannot be used as an input
    InvokeContract {
        /// Smart contract identifier
        #[serde_as(as = "Hex<Lowercase>")]
        id: [u8; 24],

        /// Script to call functions on the contract
        #[dyn_length]
        script: OpcodeScript
    } = 3,

    /// Monetary fee value in smallest currency unit (can only be claimed by miner)
    Fee(#[dyn_int] u64) = 4
}
