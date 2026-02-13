use binary_codec::{BinaryDeserializer, BinarySerializer, BitStreamReader, BitStreamWriter, SerializerConfig};
use serde::{Deserialize, Serialize};

use crate::{crypto::calculate_mac, errors::{DeserializationError, SerializationError}, packets::{
    base::{PlabblePacketBase, settings::CryptoSettings},
    body::request_body::PlabbleRequestBody,
    context::PlabbleConnectionContext,
    header::{request_header::PlabbleRequestHeader, type_and_flags::RequestPacketType},
}};

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

impl BinarySerializer<PlabbleConnectionContext, SerializationError> for PlabbleRequestPacket {
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

impl BinaryDeserializer<PlabbleConnectionContext, DeserializationError> for PlabbleRequestPacket {
    fn read_bytes(
        stream: &mut BitStreamReader,
        config: Option<&mut SerializerConfig<PlabbleConnectionContext>>,
    ) -> Result<Self, DeserializationError> {
        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        // If full encryption is enabled (in provided context), try set it
        if let Some(ctx) = &config.data && ctx.full_encryption {
            stream.set_crypto(ctx.create_crypto_stream(None, true));
        }

        let base = PlabblePacketBase::read_bytes(stream, Some(config))?;
        if base.crypto_settings.is_none() {
            // TODO: apply context settings
            CryptoSettings::apply_defaults(config);
        } else if let Some(ctx) = config.data.as_mut() {
            // Overwrite context settings
            ctx.crypto_settings = base.crypto_settings.clone();
        }

        // If encryption enabled (and context provided), try set it (might overwrite the full packet encryption key, if that was the case)
        if base.use_encryption && let Some(ctx) = &config.data {
            stream.set_crypto(ctx.create_crypto_stream(Some(&base), true));
        }

        // If MAC is enabled (and context provided), keep an offset of 16 on the reader
        if !base.use_encryption && config.data.is_some() {
            stream.set_offset_end(16);
        }

        let header = PlabbleRequestHeader::read_bytes(stream, Some(config))?;
        config.discriminator = Some(header.packet_type.get_discriminator());

        // Copy plain base/header bytes to integrity buffer for later checks
        let raw_base_and_header = stream.slice_marker(None).to_vec();

        // Read body bytes from stream
        let mut body_bytes = stream.read_bytes(stream.bytes_left())?.to_owned();
        
        // Decrypt the body if that is needed (and context is provided), first without bucket key then with bucket key as AAD
        if base.use_encryption && let Some(ctx) = &config.data {
            body_bytes = ctx.decrypt(&base, true, &body_bytes, &ctx.create_authenticated_data(&raw_base_and_header, None))
                .or(ctx.decrypt(&base, true, &body_bytes, &ctx.create_authenticated_data(&raw_base_and_header, header.id.as_ref())))
                .ok_or(DeserializationError::DecryptionFailed)?;
        }

        let body = PlabbleRequestBody::from_bytes(&body_bytes, Some(config))?;

        // Verify the MAC if that is enabled (and context provided), first without bucket key and then with bucket key as AAD
        if !base.use_encryption && let Some(ctx) = &config.data {
            let expected: [u8; 16] = stream.slice_end().try_into().expect("A 16-byte MAC on the end");
            let mac_key = ctx.create_key(Some(&base), 0xFF, true).expect("Failed to create MAC key from context");
            
            let mac1 = calculate_mac(ctx.use_blake3(), &mac_key, &body_bytes, Some(&ctx.create_authenticated_data(&raw_base_and_header, None)));
            if mac1 != expected {
                let mac2 = calculate_mac(ctx.use_blake3(), &mac_key, &body_bytes, Some(&ctx.create_authenticated_data(&raw_base_and_header, header.id.as_ref())));
                if mac2 != expected {
                    return Err(DeserializationError::IntegrityFailed);
                }
            }
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
