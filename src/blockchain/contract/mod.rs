use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::scripting::{
    interpreter::ScriptInterpreter,
    opcode_script::{OpcodeScript, ScriptError, ScriptSettings},
    stack::StackData,
};

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

impl SmartContract {
    /// Create new contract
    pub fn new(constructor: OpcodeScript) -> Self {
        Self {
            state: HashMap::new(),
            functions: HashMap::new(),
            constructor: Some(constructor),
        }
    }

    /// Try to install smart contract by executing the constructor code (if any)
    pub fn install(&mut self) -> Result<(), ScriptError> {
        let constructor = self
            .constructor
            .take()
            .ok_or(ScriptError::PreconditionFailed)?;

        let mut settings: ScriptSettings = Default::default();
        settings.alllow_function_declaration = true;

        let mut interpreter = ScriptInterpreter::new(constructor, Some(settings));
        interpreter.exec()?;

        self.state = interpreter.variables.clone();
        self.functions = interpreter
            .functions
            .iter()
            .map(|(k, v)| (*k, v.1.clone()))
            .collect();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        blockchain::contract::SmartContract,
        scripting::{
            opcode_script::{Opcode, OpcodeScript},
            stack::StackData,
        },
    };

    #[test]
    fn can_create_simple_smart_contract() {
        // Example contract: vending machine (without actual security/checks, just to show how it could look like)
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(10), // initial balance
            Opcode::STOREVAR(0), // store in variable 0 (balance)
            // declare function purchase(amount)
            Opcode::FUN(0, 1),   // function id 0, 1 parameter      [amount]
            Opcode::LOADVAR(0),  // load balance                    [amount, balance]
            Opcode::SWAP,        // swap to get balance on top      [balance, amount]
            Opcode::DUP2, // duplicate balance and amount on stack  [balance, amount, balance, amount]
            Opcode::GT,   // check if balance > amount              [balance, amount, is_enough]
            Opcode::ASSERT, // if not, cancel                       [balance, amount]
            Opcode::SUB,  // balance - amount                       [new_balance]
            Opcode::STOREVAR(0), // update balance in state         []
            Opcode::NUF,  // end function declaration
        ]);

        let mut contract = SmartContract::new(script);
        contract.install().expect("installation should succeed");

        assert_eq!(contract.state.get(&0), Some(&StackData::Number(10)));
        assert!(contract.functions.contains_key(&0));
        assert_eq!(contract.functions.get(&0).unwrap().instructions.len(), 7);
    }
}
