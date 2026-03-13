use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransactionOutput {
}