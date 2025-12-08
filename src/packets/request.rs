use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};
use serde::{Deserialize, Serialize};

use crate::packets::{
    base::{PlabblePacketBase, settings::CryptoSettings},
    body::request_body::PlabbleRequestBody,
    header::{request_header::PlabbleRequestHeader, type_and_flags::RequestPacketType},
};

/// A Plabble request packet, consisting of a base, header, and body.
/// The body type is determined by the packet type in the header.
///
/// # Members
/// - `base`: The base packet information common to all Plabble packets (both requests and responses).
/// - `header`: The request-specific header containing metadata.
/// - `body`: The request-specific body, whose structure depends on the packet type.
#[derive(Serialize, Debug, PartialEq)]
pub struct PlabbleRequestPacket {
    /// The base packet information common to all Plabble packets.
    #[serde(flatten)]
    pub base: PlabblePacketBase,

    pub header: PlabbleRequestHeader,

    pub body: PlabbleRequestBody,
}

impl BinarySerializer for PlabbleRequestPacket {
    fn serialize_bytes(&self, config: Option<&mut binary_codec::SerializerConfig<()>>) -> Result<Vec<u8>, binary_codec::SerializationError> {
        let mut buffer = Vec::new();
        Self::write_bytes(&self, &mut buffer, config)?;
        Ok(buffer)
    }

    fn write_bytes(&self, buffer: &mut Vec<u8>, config: Option<&mut binary_codec::SerializerConfig<()>>) -> Result<(), binary_codec::SerializationError> {
        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        self.base.write_bytes(buffer, Some(config))?;

        // TODO: header encryption
        self.header.preprocess();
        self.header.write_bytes(buffer, Some(config))?;

        // TODO: body decryption
        self.body.write_bytes(buffer, Some(config))?;

        Ok(())
    }
}

impl BinaryDeserializer for PlabbleRequestPacket {
    fn deserialize_bytes(bytes: &[u8], config: Option<&mut SerializerConfig<()>>) -> Result<Self, binary_codec::DeserializationError> {
        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        let base = PlabblePacketBase::deserialize_bytes(bytes, Some(config))?;
        if base.crypto_settings.is_none() {
            CryptoSettings::apply_defaults(config);
        }

        // TODO: header encryption
        let header = PlabbleRequestHeader::deserialize_bytes(bytes, Some(config))?;
        config.discriminator = Some(header.packet_type.get_discriminator());

        // TODO: body encryption
        let body = PlabbleRequestBody::deserialize_bytes(bytes, Some(config))?;

        Ok(Self {
            base,
            header,
            body
        })
    }
}

impl<'de> Deserialize<'de> for PlabbleRequestPacket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        struct RawPacket {
            #[serde(flatten)]
            base: PlabblePacketBase,
            header: PlabbleRequestHeader,
            // We'll temporarily store the body as untyped data
            body: serde_value::Value,
        }

        let raw = RawPacket::deserialize(deserializer)?;
        raw.header.preprocess();

        let body = match raw.header.packet_type {
            RequestPacketType::Certificate { .. } => todo!(),
            RequestPacketType::Session { .. } => {
                PlabbleRequestBody::Session(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Get { .. } => todo!(),
            RequestPacketType::Stream { .. } => todo!(),
            RequestPacketType::Post { .. } => todo!(),
            RequestPacketType::Patch => todo!(),
            RequestPacketType::Put { .. } => todo!(),
            RequestPacketType::Delete { .. } => todo!(),
            RequestPacketType::Subscribe { .. } => todo!(),
            RequestPacketType::Unsubscribe { .. } => todo!(),
            RequestPacketType::Register => todo!(),
            RequestPacketType::Identify => todo!(),
            RequestPacketType::Proxy { .. } => todo!(),
            RequestPacketType::_Reserved13 => todo!(),
            RequestPacketType::Opcode { .. } => todo!(),
            RequestPacketType::_Reserved15 => todo!(),
        };

        Ok(PlabbleRequestPacket {
            base: raw.base,
            header: raw.header,
            body,
        })
    }
}

// Tests for the request and response packets are in the type-specific files
