use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Plabble error code body
/// The length is prefixed by a u8 in the packet body.
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub enum PlabbleError {
    /// The requested protocol version is not supported by this implementation.
    /// Contains the min and max version the server supports.
    UnsupportedProtocolVersion(u8, u8),


}

#[cfg(test)]
mod tests {
    use crate::packets::response::PlabbleResponsePacket;

    #[test]
    fn can_serialize_and_deserialize_error_response() {
        let response: PlabbleResponsePacket = 
    }
}