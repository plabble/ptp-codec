use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::scripting::{opcode_script::OpcodeScript, stack::StackData};

/// Smart contract that automatically executes when certain conditions are met
/// - When called from another smart contract
/// - When referenced in a transaction (TXIN)
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct SmartContract {
    /// Contract state
    #[dyn_length]
    state: HashMap<u8, StackData>,

    /// Contract functions
    #[dyn_length]
    #[val_dyn_length]
    functions: HashMap<u8, OpcodeScript>,

    /// Constructor code (if any) that is executed when the contract is deployed
    constructor: Option<OpcodeScript>,
}