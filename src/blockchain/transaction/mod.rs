pub mod tx_input;
pub mod tx_output;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::blockchain::transaction::{tx_input::TransactionInput, tx_output::TransactionOutput};

#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transaction {
    /// Transaction version number, used for compatibility checks and future upgrades
    #[bits(4)]
    version: u8,

    /// Transaction inputs
    #[dyn_length]
    inputs: Vec<TransactionInput>,

    /// Transaction outputs
    #[dyn_length]
    outputs: Vec<TransactionOutput>
}