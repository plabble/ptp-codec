use binary_codec::{BinaryDeserializer, BinarySerializer, BitStreamWriter, SerializerConfig};
use serde::{Deserialize, Serialize};

use crate::{errors::{SerializationError, DeserializationError}, packets::{
    base::{PlabblePacketBase, settings::CryptoSettings},
    body::response_body::PlabbleResponseBody,
    header::{response_header::PlabbleResponseHeader, type_and_flags::ResponsePacketType},
}};

/// A Plabble response packet, consisting of a base, header, and body.
/// The body type is determined by the packet type in the header.
///
/// # Members
/// - `base`: The base packet information common to all Plabble packets (both requests and responses).
/// - `header`: The request-specific header containing metadata.
/// - `body`: The response-specific body, whose structure depends on the packet type.
#[derive(Serialize, Debug, PartialEq)]
pub struct PlabbleResponsePacket {
    /// The base packet information common to all Plabble packets.
    #[serde(flatten)]
    pub base: PlabblePacketBase,

    pub header: PlabbleResponseHeader,

    pub body: PlabbleResponseBody,
}

/// Response context for cryptography, session etc.
#[derive(Clone)]
pub struct PlabbleResponseContext {}

impl BinarySerializer<PlabbleResponseContext, SerializationError> for PlabbleResponsePacket {
    fn write_bytes(
        &self,
        stream: &mut BitStreamWriter,
        config: Option<&mut binary_codec::SerializerConfig<PlabbleResponseContext>>,
    ) -> Result<(), SerializationError> {
        self.header.preprocess();

        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        self.base.write_bytes(stream, Some(config))?;

        // println!("{:?} {:?}", stream, config);

        // TODO: header encryption
        self.header.write_bytes(stream, Some(config))?;

        // TODO: body decryption
        self.body.write_bytes(stream, Some(config))?;

        Ok(())
    }
}

impl BinaryDeserializer<PlabbleResponseContext, DeserializationError> for PlabbleResponsePacket {
    fn read_bytes(
        stream: &mut binary_codec::BitStreamReader,
        config: Option<&mut SerializerConfig<PlabbleResponseContext>>,
    ) -> Result<Self, DeserializationError> {
        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        let base = PlabblePacketBase::read_bytes(stream, Some(config))?;
        if base.crypto_settings.is_none() {
            CryptoSettings::apply_defaults(config);
        }

        // TODO: header encryption
        let header = PlabbleResponseHeader::read_bytes(stream, Some(config))?;
        config.discriminator = Some(header.packet_type.get_discriminator());

        // TODO: body encryption
        let body = PlabbleResponseBody::read_bytes(stream, Some(config))?;

        Ok(Self { base, header, body })
    }
}

impl<'de> Deserialize<'de> for PlabbleResponsePacket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        struct RawPacket {
            #[serde(flatten)]
            base: PlabblePacketBase,
            header: PlabbleResponseHeader,
            // We'll temporarily store the body as untyped data
            body: serde_value::Value,
        }

        let raw = RawPacket::deserialize(deserializer)?;
        raw.header.preprocess();

        let body = match raw.header.packet_type {
            ResponsePacketType::Certificate => {
                PlabbleResponseBody::Certificate(raw.body.deserialize_into().unwrap())
            }
            ResponsePacketType::Session { .. } => {
                PlabbleResponseBody::Session(raw.body.deserialize_into().unwrap())
            }
            ResponsePacketType::Get { .. } => {
                PlabbleResponseBody::Get(raw.body.deserialize_into().unwrap())
            }
            ResponsePacketType::Stream => todo!(),
            ResponsePacketType::Post => PlabbleResponseBody::Post,
            ResponsePacketType::Patch => todo!(),
            ResponsePacketType::Put => todo!(),
            ResponsePacketType::Delete => todo!(),
            ResponsePacketType::Subscribe => todo!(),
            ResponsePacketType::Unsubscribe => todo!(),
            ResponsePacketType::Register => todo!(),
            ResponsePacketType::Identify => todo!(),
            ResponsePacketType::Proxy { .. } => todo!(),
            ResponsePacketType::Custom { .. } => todo!(),
            ResponsePacketType::Opcode => todo!(),
            ResponsePacketType::Error => {
                PlabbleResponseBody::Error(raw.body.deserialize_into().unwrap())
            }
        };

        Ok(PlabbleResponsePacket {
            base: raw.base,
            header: raw.header,
            body,
        })
    }
}

// Tests for the request and response packets are in the type-specific files
