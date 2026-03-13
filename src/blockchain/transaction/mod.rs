pub mod tx_input;
pub mod tx_output;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::{blockchain::transaction::{tx_input::TransactionInput, tx_output::TransactionOutput}, core::PlabbleDateTime};

#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transaction {
    /// Transaction version number, used for compatibility checks and future upgrades
    #[bits = 4]
    version: u8,

    /// Indicates whether the transaction has a time lock or height lock
    #[toggles("time_lock")]
    has_time_lock: bool,

    // 3 bits reserved for future use

    /// Transaction inputs
    #[dyn_length]
    inputs: Vec<TransactionInput>,

    /// Transaction outputs
    #[dyn_length]
    outputs: Vec<TransactionOutput>,

    /// Optional time lock for the transaction, which specifies the earliest time the transaction can be included in a block. 
    /// This field is only present if `has_time_lock` is true.
    #[toggled_by = "has_time_lock"]
    time_lock: Option<PlabbleDateTime>,

    /// Optional relative height lock for the transaction, which specifies the minimum number of blocks that must be mined after the transaction is included before it can be spent.
    #[toggled_by = "!has_time_lock"]
    height_lock: Option<u16>
}