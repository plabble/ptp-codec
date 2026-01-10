use binary_codec::{BinaryDeserializer, BinarySerializer, BitStreamWriter, SerializerConfig};
use serde::{Deserialize, Serialize};

use crate::packets::{
    base::{PlabblePacketBase, settings::CryptoSettings}, body::request_body::PlabbleRequestBody, context::PlabblePacketContext, header::{request_header::PlabbleRequestHeader, type_and_flags::RequestPacketType}
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

impl BinarySerializer<PlabblePacketContext> for PlabbleRequestPacket {
    fn write_bytes(
        &self,
        stream: &mut BitStreamWriter,
        config: Option<&mut binary_codec::SerializerConfig<PlabblePacketContext>>,
    ) -> Result<(), binary_codec::SerializationError> {
        self.header.preprocess();

        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        // TODO: full packet encryption if in session
        self.base.write_bytes(stream, Some(config))?;
        
        // TODO: header encryption
        self.header.write_bytes(stream, Some(config))?;

        // TODO: body encryption
        self.body.write_bytes(stream, Some(config))?;
        
        // If MAC is enabled, add it to the packet
        if !self.base.use_encryption {

        }

        Ok(())
    }
}

impl BinaryDeserializer<PlabblePacketContext> for PlabbleRequestPacket {
    fn read_bytes(
        stream: &mut binary_codec::BitStreamReader,
        config: Option<&mut SerializerConfig<PlabblePacketContext>>,
    ) -> Result<Self, binary_codec::DeserializationError> {
        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        // TODO: full packet encryption if in session
        let base = PlabblePacketBase::read_bytes(stream, Some(config))?;
        if base.crypto_settings.is_none() {
            CryptoSettings::apply_defaults(config);
        }

        // If mac is enabled, keep an offset of 16 on the reader
        if !base.use_encryption {
            // stream.set_offset_end(16);
        }

        // if use encryption, set the crypto in the bitreader. Choose algorithm(s) based on crypto settings
        let header = PlabbleRequestHeader::read_bytes(stream, Some(config))?;
        config.discriminator = Some(header.packet_type.get_discriminator());

        // TODO: body decryption
        let body = PlabbleRequestBody::read_bytes(stream, Some(config))?;

        // IF mac is enabled, check it here
        if !base.use_encryption {
            // let mac: &[u8; 16] = stream.slice_end().try_into().expect("A 16-byte MAC on the end");
        }

        Ok(Self { base, header, body })
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
            RequestPacketType::Certificate { .. } => {
                PlabbleRequestBody::Certificate(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Session { .. } => {
                PlabbleRequestBody::Session(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Get { .. } => {
                PlabbleRequestBody::Get(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Stream { .. } => todo!(),
            RequestPacketType::Post { .. } => {
                PlabbleRequestBody::Post(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Patch => todo!(),
            RequestPacketType::Put { .. } => todo!(),
            RequestPacketType::Delete { .. } => todo!(),
            RequestPacketType::Subscribe { .. } => todo!(),
            RequestPacketType::Unsubscribe { .. } => todo!(),
            RequestPacketType::Register => todo!(),
            RequestPacketType::Identify => todo!(),
            RequestPacketType::Proxy { .. } => todo!(),
            RequestPacketType::Opcode { .. } => todo!(),
        };

        Ok(PlabbleRequestPacket {
            base: raw.base,
            header: raw.header,
            body,
        })
    }
}

// Tests for the request and response packets are in the type-specific files
