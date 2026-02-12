use binary_codec::{BinaryDeserializer, BinarySerializer, BitStreamReader, BitStreamWriter, DeserializationError, SerializationError, SerializerConfig};
use serde::{Deserialize, Serialize};

use crate::packets::{
    base::{PlabblePacketBase, settings::CryptoSettings},
    body::request_body::PlabbleRequestBody,
    context::PlabbleConnectionContext,
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

impl BinarySerializer<PlabbleConnectionContext> for PlabbleRequestPacket {
    fn write_bytes(
        &self,
        stream: &mut BitStreamWriter,
        config: Option<&mut SerializerConfig<PlabbleConnectionContext>>,
    ) -> Result<(), SerializationError> {
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
        if !self.base.use_encryption {}

        Ok(())
    }
}

impl BinaryDeserializer<PlabbleConnectionContext> for PlabbleRequestPacket {
    fn read_bytes(
        stream: &mut BitStreamReader,
        config: Option<&mut SerializerConfig<PlabbleConnectionContext>>,
    ) -> Result<Self, DeserializationError> {
        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        // If full encryption is enabled, try set it
        if let Some(ctx) = &config.data && ctx.full_encryption {
            stream.set_crypto(ctx.create_crypto_stream(None, true));
        }
        
        let base = PlabblePacketBase::read_bytes(stream, Some(config))?;
        if base.crypto_settings.is_none() {
            // TODO: apply context settings
            CryptoSettings::apply_defaults(config);
        } else {
            // TODO: overwrite context settings
        }

        // If encryption enabled, try set it (might overwrite the full packet encryption key, if that was the case)
        if base.use_encryption && let Some(ctx) = &config.data {
            stream.set_crypto(ctx.create_crypto_stream(Some(&base), true));
        }

        // TODO If MAC is enabled, keep an offset of 16 on the reader
        if !base.use_encryption {
            stream.set_offset_end(16);
        }

        let header = PlabbleRequestHeader::read_bytes(stream, Some(config))?;
        config.discriminator = Some(header.packet_type.get_discriminator());

        // Read body bytes from stream
        let mut body = stream.read_bytes(stream.bytes_left())?.to_owned();
        
        // Decrypt the body if that is needed
        if base.use_encryption && let Some(ctx) = &config.data {
            // TODO: AAD
            let res = ctx.decrypt(&base, true, &body, &body);
        }

        let body = PlabbleRequestBody::from_bytes(&body, Some(config))?;

        // TODO IF mac is enabled, check it here
        if !base.use_encryption {
            let mac: &[u8; 16] = stream.slice_end().try_into().expect("A 16-byte MAC on the end");
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
            RequestPacketType::Custom { .. } => todo!(),
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

#[cfg(test)]
mod tests {
    // use binary_codec::{BinaryDeserializer, SerializerConfig};

    // use crate::packets::{context::PlabbleConnectionContext, request::PlabbleRequestPacket};

    // #[test]
    // fn test_serialization_with_context() {
    //     let mut config = SerializerConfig::new(Some(PlabbleConnectionContext::new()));

    //     let reference = Some(&mut config);
    //     // config.data.as_mut().unwrap().server_counter += 1;

    //     let packet = PlabbleRequestPacket::from_bytes(&[0u8], reference);
    // }
}
