use binary_codec::{BinaryDeserializer, SerializerConfig};
use serde::{Deserialize, Serialize};

use crate::packets::{base::PlabblePacketBase, header::request_header::PlabbleRequestHeader};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PlabbleRequestPacket {
    #[serde(flatten)]
    base: PlabblePacketBase,

    header: PlabbleRequestHeader,
}

impl BinaryDeserializer for PlabbleRequestPacket {
    fn from_bytes(bytes: &[u8], config: Option<&mut binary_codec::SerializerConfig>) -> Result<Self, binary_codec::DeserializationError> {
        let mut default_config = SerializerConfig::new();
        let config = config.unwrap_or(&mut default_config);
        let base = PlabblePacketBase::from_bytes(bytes, Some(config))?;
        let header = PlabbleRequestHeader::from_bytes(bytes, Some(config))?;
        config.reset_bits(true); // TODO check if this is correct. But I think it is at this point.
        
        Ok(Self {
            base,
            header
        })
    }
}

#[cfg(test)]
mod tests {
    use binary_codec::BinaryDeserializer;

    use crate::packets::request::PlabbleRequestPacket;

    #[test]
    fn try_deserialize_request_packet() {
        let bytes = vec![0b0100_0000, 0b1111_0101];
        let req = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        println!("{:?}", req);
    }
}