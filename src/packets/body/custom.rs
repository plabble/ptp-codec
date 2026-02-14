use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Custom request/response body for sub-protocols, containing a protocol ID and raw data
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct CustomBody {
    /// Protocol ID to identify the sub-protocol this body belongs to
    pub protocol: u16,

    /// Raw data for the sub-protocol, which can be parsed according to the protocol ID
    pub data: Vec<u8>,
}
