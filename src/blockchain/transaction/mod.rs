pub mod tx_input;
pub mod tx_output;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::{
    blockchain::transaction::{tx_input::TransactionInput, tx_output::TransactionOutput},
    core::PlabbleDateTime,
};

#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Transaction {
    /// Transaction version number, used for compatibility checks and future upgrades
    #[bits = 4]
    pub version: u8,

    /// Indicates whether the transaction has a time lock or height lock
    #[toggles("time_lock")]
    pub has_time_lock: bool,

    /// Indicates whether the transaction is replaceable by fee (RBF),
    /// allowing it to be replaced with a higher fee transaction before being included in a block
    pub replaceable_by_fee: bool,

    // 2 bits reserved for future use
    /// Transaction inputs
    #[dyn_length]
    pub inputs: Vec<TransactionInput>,

    /// Transaction outputs
    #[dyn_length]
    pub outputs: Vec<TransactionOutput>,

    /// Transaction lock
    #[variant_by = "time_lock"]
    pub lock: TransactionLock,
}

#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
#[no_discriminator]
pub enum TransactionLock {
    /// Relative height lock, specifying the minimum number of blocks that must be mined after the transaction is included before it can be spent.
    Height(#[dyn_int] u64),

    /// Time lock, specifying the earliest time (as a Plabble timestamp) that the transaction can be included in a block.
    Time(PlabbleDateTime),
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    use crate::{
        blockchain::transaction::{
            Transaction, TransactionLock,
            tx_input::TransactionInput,
            tx_output::{OutputType, TransactionOutput},
        },
        scripting::opcode_script::{Opcode, OpcodeScript},
    };

    #[test]
    fn can_serialize_and_deserialize_transaction() {
        use crate::core::PlabbleDateTime;

        let original_tx = Transaction {
            version: 1,
            has_time_lock: true,
            replaceable_by_fee: false,
            inputs: vec![
                TransactionInput {
                    transaction_id: [1u8; 24],
                    output_index: 7,
                    unlocking_script: OpcodeScript::new(vec![Opcode::NOP, Opcode::RETURN]),
                },
                TransactionInput {
                    transaction_id: [2u8; 24],
                    output_index: 3,
                    unlocking_script: OpcodeScript::new(vec![Opcode::TRUE]),
                },
            ],
            outputs: vec![
                TransactionOutput {
                    is_monetary: true,
                    burn: false,
                    not_replaceable: false,
                    value: OutputType::Monetary(123),
                    locking_script: Some(OpcodeScript::new(vec![Opcode::ASSERT])),
                },
                TransactionOutput {
                    is_monetary: true,
                    burn: true,
                    not_replaceable: true,
                    value: OutputType::Monetary(456),
                    locking_script: None,
                },
                TransactionOutput {
                    is_monetary: false,
                    burn: false,
                    not_replaceable: false,
                    value: OutputType::Asset([0xFFu8; 24]),
                    locking_script: Some(OpcodeScript::new(vec![Opcode::FALSE])),
                },
            ],
            lock: TransactionLock::Time(PlabbleDateTime::new(10)),
        };

        let config: Option<&mut SerializerConfig> = None;
        let bytes = original_tx.to_bytes(config).unwrap();

        assert_eq!(bytes[0], 0b0001_0001); // version 1, first flag set
        // 02 input count
        // 24x 01 first input transaction ID
        // 07 first transaction vout
        // 02 script length
        // 0x46 NOP, 0x4F RETURN
        // 24x 02 second input transaction ID
        // 03 second transaction vout
        // 01 script length
        // 0x01 TRUE
        // 03 output count
        // 0000_0001 first output flags (monetary, not burnable, replaceable)
        // 0x7B value (123)
        // 01 script length
        // 0x4E ASSERT
        // 0000_0111 second output flags (monetary, burnable, not replaceable)
        // 0xc803 value (456)
        // 0000_0000 third output flags (non-monetary, not burnable, replaceable)
        // 24x 0xFF asset ID
        // 01 script length
        // 0x00 FALSE
        // 0000000a time lock

        assert_eq!(
            "11020101010101010101010101010101010101010101010101010702464f02020202020202020202020202020202020202020202020203010103017b014e07c80300ffffffffffffffffffffffffffffffffffffffffffffffff01000000000a",
            hex::encode(&bytes)
        );

        println!("---------------------------");

        let config: Option<&mut SerializerConfig> = None;
        let deserialized_tx = Transaction::from_bytes(&bytes, config).unwrap();

        assert_eq!(original_tx, deserialized_tx);
    }
}
