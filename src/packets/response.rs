use binary_codec::{BinaryDeserializer, BinarySerializer, BitStreamWriter, SerializerConfig};
use serde::{Deserialize, Serialize};

use crate::{
    crypto::calculate_mac,
    errors::{DeserializationError, SerializationError},
    packets::{
        base::PlabblePacketBase,
        body::response_body::PlabbleResponseBody,
        context::PlabbleConnectionContext,
        header::{response_header::PlabbleResponseHeader, type_and_flags::ResponsePacketType},
        read_base_packet, write_base_packet,
    },
};

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

impl BinarySerializer<PlabbleConnectionContext, SerializationError> for PlabbleResponsePacket {
    fn write_bytes(
        &self,
        stream: &mut BitStreamWriter,
        config: Option<&mut binary_codec::SerializerConfig<PlabbleConnectionContext>>,
    ) -> Result<(), SerializationError> {
        self.header.preprocess();

        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        // Write the base packet and apply crypto settings to the stream as needed
        write_base_packet(stream, &self.base, config)?;

        // Write the header to the stream
        self.header.write_bytes(stream, Some(config))?;

        // Copy plain base/header bytes to integrity buffer for later checks
        let raw_base_and_header = stream.slice_marker(None).to_vec();

        let mut body_bytes = self.body.to_bytes(Some(config))?;

        // Encrypt the body if that is needed (and context is provided)
        if self.base.use_encryption
            && let Some(ctx) = &config.data
        {
            body_bytes = ctx
                .encrypt(
                    &self.base,
                    false,
                    &body_bytes,
                    &ctx.create_authenticated_data(&raw_base_and_header, None),
                )
                .ok_or(SerializationError::EncryptionFailed)?;
        }

        // Write the body bytes to the stream
        stream.write_bytes(&body_bytes);

        // If MAC is enabled and inside session, calculate and add it to the packet
        // Note that SESSION packets without PSK do never have a MAC, because there is no shared key yet
        if !self.base.use_encryption
            && let Some(ctx) = &config.data
            && (!self.header.is_session_packet() || self.base.pre_shared_key)
        {
            let mac_key = ctx
                .create_key(Some(&self.base), 0xFF, false)
                .expect("Failed to create MAC key from context");

            let mac = calculate_mac(
                ctx.use_blake3(),
                &mac_key,
                &body_bytes,
                Some(&ctx.create_authenticated_data(&raw_base_and_header, None)),
            );
            stream.write_bytes(&mac);
        }

        Ok(())
    }
}

impl BinaryDeserializer<PlabbleConnectionContext, DeserializationError> for PlabbleResponsePacket {
    fn read_bytes(
        stream: &mut binary_codec::BitStreamReader,
        config: Option<&mut SerializerConfig<PlabbleConnectionContext>>,
    ) -> Result<Self, DeserializationError> {
        let mut new_config = SerializerConfig::new(None);
        let config = config.unwrap_or(&mut new_config);

        // Read the base packet and apply crypto settings to the stream as needed
        let base = read_base_packet(stream, config)?;

        let header = PlabbleResponseHeader::read_bytes(stream, Some(config))?;
        config.discriminator = Some(header.packet_type.get_discriminator());

        // Copy plain base/header bytes to integrity buffer for later checks
        let raw_base_and_header = stream.slice_marker(None).to_vec();

        // Read body bytes from stream
        let mut body_bytes = stream.read_bytes(stream.bytes_left())?.to_owned();

        // Decrypt the body if that is needed (and context is provided)
        if base.use_encryption
            && let Some(ctx) = &config.data
        {
            body_bytes = ctx
                .decrypt(
                    &base,
                    false,
                    &body_bytes,
                    &ctx.create_authenticated_data(&raw_base_and_header, None),
                )
                .ok_or(DeserializationError::DecryptionFailed)?;
        }

        let body = PlabbleResponseBody::from_bytes(&body_bytes, Some(config))?;

        // Verify the MAC if that is enabled (and context provided)
        // Note that SESSION packets without PSK do never have a MAC, because there is no shared key yet
        if !base.use_encryption
            && let Some(ctx) = &config.data
            && (!header.is_session_packet() || base.pre_shared_key)
        {
            let expected: [u8; 16] = stream
                .slice_end()
                .try_into()
                .expect("A 16-byte MAC on the end");
            let mac_key = ctx
                .create_key(Some(&base), 0xFF, false)
                .expect("Failed to create MAC key from context");

            let mac = calculate_mac(
                ctx.use_blake3(),
                &mac_key,
                &body_bytes,
                Some(&ctx.create_authenticated_data(&raw_base_and_header, None)),
            );
            if mac != expected {
                return Err(DeserializationError::IntegrityFailed);
            }
        }

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
#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    use crate::{errors::DeserializationError, packets::{context::PlabbleConnectionContext, response::PlabbleResponsePacket}};

    #[test]
    fn can_encrypt_and_decrypt_response_packet() {
        let response: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Error"
            request_counter = 1

            [body]
            type = "UnsupportedVersion"
            min_version = 1
            max_version = 3
        "#,
        )
        .unwrap();

        let mut context = PlabbleConnectionContext::new();
        context.session_key = Some([0u8; 64]);
        let mut config = SerializerConfig::new(Some(context));

        let plain = "410f0001000103";
        let cipher = response.to_bytes(Some(&mut config)).unwrap();
        assert_ne!(plain, hex::encode(&cipher));
        assert_eq!(7 + 16, cipher.len()); // 7 bytes of base+header, 16 bytes of ciphertext

        let decrypted = PlabbleResponsePacket::from_bytes(&cipher, Some(&mut config)).unwrap();
        assert_eq!(response, decrypted);

        // Invalid byte in ciphertext MAC should fail decryption
        let mut wrong = cipher.clone();
        wrong[cipher.len() - 1] ^= 0xFF; // Flip a byte in the ciphertext
        assert_eq!(Err(DeserializationError::DecryptionFailed), PlabbleResponsePacket::from_bytes(&wrong, Some(&mut config)));
    }

    #[test]
    fn can_serialize_and_deserialize_response_packet_with_mac() {
        let response: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 1
            use_encryption = false

            [header]
            packet_type = "Error"
            request_counter = 1

            [body]
            type = "UnsupportedVersion"
            min_version = 1
            max_version = 3
        "#,
        )
        .unwrap();

        let plain = "010f0001000103";
        let mac = "b7fbd584e891fc4af0499fbfdbfda11c";

        let mut context = PlabbleConnectionContext::new();
        context.session_key = Some([0u8; 64]);
        let mut config = SerializerConfig::new(Some(context));

        let serialized = response.to_bytes(Some(&mut config)).unwrap();
        assert_eq!(format!("{}{}", plain, mac), hex::encode(&serialized));

        let deserialized = PlabbleResponsePacket::from_bytes(&serialized, Some(&mut config)).unwrap();
        assert_eq!(response, deserialized);

        // Invalid MAC should fail integrity check
        let mut wrong = serialized.clone();
        wrong[serialized.len() - 1] ^= 0xFF; // Flip a byte in the MAC
        assert_eq!(Err(DeserializationError::IntegrityFailed), PlabbleResponsePacket::from_bytes(&wrong, Some(&mut config)));

        // Full packet encryption should encrypt the MAC as well
        let context = config.data.as_mut().unwrap();
        context.full_encryption = true;

        let encrypted = response.to_bytes(Some(&mut config)).unwrap();
        assert_ne!(format!("{}{}", plain, mac), hex::encode(&encrypted));
        assert_eq!("971df7105b2bc21db879f2f1d469a99882647b66676f2b", hex::encode(&encrypted));
        let decrypted = PlabbleResponsePacket::from_bytes(&encrypted, Some(&mut config)).unwrap();
        assert_eq!(response, decrypted);
    }
}
