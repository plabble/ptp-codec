use binary_codec::{
    BinaryDeserializer, BinarySerializer, BitStreamReader, BitStreamWriter, SerializerConfig,
};
use serde::{Deserialize, Serialize};

use crate::{
    crypto::calculate_mac,
    errors::{DeserializationError, SerializationError},
    packets::{
        base::PlabblePacketBase,
        body::request_body::PlabbleRequestBody,
        context::PlabbleConnectionContext,
        header::{request_header::PlabbleRequestHeader, type_and_flags::RequestPacketType},
        read_base_packet, write_base_packet,
    },
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

impl BinarySerializer<PlabbleConnectionContext, SerializationError> for PlabbleRequestPacket {
    fn write_bytes(
        &self,
        stream: &mut BitStreamWriter,
        config: Option<&mut SerializerConfig<PlabbleConnectionContext>>,
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

        // Encrypt the body if that is needed (and context is provided), with or without key in AAD depending on the context setting
        if self.base.use_encryption
            && let Some(ctx) = &config.data
        {
            body_bytes = ctx
                .encrypt(
                    &self.base,
                    true,
                    &body_bytes,
                    &ctx.create_authenticated_data(
                        &raw_base_and_header,
                        if ctx.include_bucket_key_in_auth_data {
                            self.header.id.as_ref()
                        } else {
                            None
                        },
                    ),
                )
                .ok_or(SerializationError::EncryptionFailed)?;
        }

        // Write the body bytes to the stream
        stream.write_bytes(&body_bytes);

        // If MAC is enabled and not a SESSION request (or PSK is present), calculate and add it to the packet
        if !self.base.use_encryption
            && let Some(ctx) = &config.data
            && (!self.header.is_session_packet() || self.base.pre_shared_key)
        {
            let mac_key = ctx
                .create_key(Some(&self.base), 0xFF, true)
                .ok_or(SerializationError::NoKeyAvailable)?;

            let mac = calculate_mac(
                ctx.use_blake3(),
                &mac_key,
                &body_bytes,
                Some(&ctx.create_authenticated_data(
                    &raw_base_and_header,
                    if ctx.include_bucket_key_in_auth_data {
                        self.header.id.as_ref()
                    } else {
                        None
                    },
                )),
            );
            stream.write_bytes(&mac);
        }

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

        // Read the base packet and apply crypto settings to the stream as needed
        let base = read_base_packet(stream, config)?;

        let header = PlabbleRequestHeader::read_bytes(stream, Some(config))?;
        config.discriminator = Some(header.packet_type.get_discriminator());

        // Copy plain base/header bytes to integrity buffer for later checks
        let raw_base_and_header = stream.slice_marker(None).to_vec();

        // Read body bytes from stream
        let mut body_bytes = stream.read_bytes(stream.bytes_left())?.to_owned();

        // Decrypt the body if that is needed (and context is provided), first without bucket key then with bucket key as AAD
        if base.use_encryption
            && let Some(ctx) = &config.data
        {
            body_bytes = ctx
                .decrypt(
                    &base,
                    true,
                    &body_bytes,
                    &ctx.create_authenticated_data(&raw_base_and_header, None),
                )
                .or(ctx.decrypt(
                    &base,
                    true,
                    &body_bytes,
                    &ctx.create_authenticated_data(&raw_base_and_header, header.id.as_ref()),
                ))
                .ok_or(DeserializationError::DecryptionFailed)?;
        }

        let body = PlabbleRequestBody::from_bytes(&body_bytes, Some(config))?;

        // Verify the MAC if that is enabled (and context provided), first without bucket key and then with bucket key as AAD
        // Note that SESSION packets without PSK do never have a MAC, because there is no shared key yet
        if !base.use_encryption
            && let Some(ctx) = &config.data
            && (!header.is_session_packet() || base.pre_shared_key)
        {
            let expected: [u8; 16] = stream.slice_end().try_into().map_err(|_| {
                DeserializationError::UnexpectedLength(16, stream.slice_end().len())
            })?;

            let mac_key = ctx
                .create_key(Some(&base), 0xFF, true)
                .ok_or(DeserializationError::NoKeyAvailable)?;

            let mac1 = calculate_mac(
                ctx.use_blake3(),
                &mac_key,
                &body_bytes,
                Some(&ctx.create_authenticated_data(&raw_base_and_header, None)),
            );

            if mac1 != expected {
                let mac2 = calculate_mac(
                    ctx.use_blake3(),
                    &mac_key,
                    &body_bytes,
                    Some(&ctx.create_authenticated_data(&raw_base_and_header, header.id.as_ref())),
                );
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
            RequestPacketType::Stream { .. } => {
                PlabbleRequestBody::Stream(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Post { .. } => {
                PlabbleRequestBody::Post(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Patch { .. } => {
                PlabbleRequestBody::Patch(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Put { .. } => {
                PlabbleRequestBody::Put(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Delete { .. } => {
                PlabbleRequestBody::Delete(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Subscribe { .. } => {
                PlabbleRequestBody::Subscribe(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Unsubscribe { .. } => {
                PlabbleRequestBody::Unsubscribe(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Register => {
                PlabbleRequestBody::Register(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Identify => {
                PlabbleRequestBody::Identify(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Proxy { .. } => {
                PlabbleRequestBody::Proxy(raw.body.deserialize_into().unwrap())
            },
            RequestPacketType::Custom { .. } => {
                PlabbleRequestBody::Custom(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Opcode { .. } => {
                PlabbleRequestBody::Opcode(raw.body.deserialize_into().unwrap())
            }
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
    use crate::{
        errors::DeserializationError,
        packets::{
            base::settings::CryptoSettings, context::PlabbleConnectionContext,
            request::PlabbleRequestPacket,
        },
    };
    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    #[test]
    fn can_decrypt_and_decrypt_request_packet() {
        // init_logger();
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Get"
            id = "EjRWeJCrze_-3LoJh2VDIQ"

            [body]
            range.Numeric = [1, 5]
        "#,
        )
        .unwrap();

        let context = PlabbleConnectionContext::new();
        let mut config = SerializerConfig::new(Some(context));

        let context = config.data.as_mut().unwrap();
        context.session_key = Some([0u8; 64]);

        let encrypted = packet.to_bytes(Some(&mut config)).unwrap();
        // 22 + 16 byte ciphertext (poly1305 tag is 16 bytes)
        assert_eq!(22 + 16, encrypted.len());

        let decrypted = PlabbleRequestPacket::from_bytes(&encrypted, Some(&mut config)).unwrap();

        // 22-byte plaintext (base+header (2b), 16 byte bucket id, 2x 2-byte number)
        assert_eq!(
            "41021234567890abcdeffedcba098765432100010005",
            hex::encode(decrypted.to_bytes(None).unwrap())
        );
        assert_eq!(packet, decrypted);

        // When encrypted with BucketId in AAD, decryption is different
        let context = config.data.as_mut().unwrap();
        context.get_bucket_key = Some(|_| Some([0u8; 32]));
        context.include_bucket_key_in_auth_data = true;

        let without_bucket_key = hex::encode(&encrypted);
        let encrypted = packet.to_bytes(Some(&mut config)).unwrap();
        let with_bucket_key = hex::encode(&encrypted);

        // Ciphertext should be the same, but the poly1305 mac is different because the AAD is different
        assert_ne!(without_bucket_key, with_bucket_key);
        assert!(with_bucket_key.starts_with("419400c34723ba6a67ac52aaa34c12862beae91764d9"));
        assert!(without_bucket_key.starts_with("419400c34723ba6a67ac52aaa34c12862beae91764d9"));

        // Should automatically fall back on the bucket key if first decryption fails
        let decrypted = PlabbleRequestPacket::from_bytes(&encrypted, Some(&mut config)).unwrap();
        assert_eq!(packet, decrypted);
    }

    #[test]
    fn can_serialize_and_deserialize_request_packet_with_mac() {
        // init_logger();
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Get"
            id = "EjRWeJCrze_-3LoJh2VDIQ"

            [body]
            range.Numeric = []
        "#,
        )
        .unwrap();

        let context = PlabbleConnectionContext::new();
        let mut config = SerializerConfig::new(Some(context));

        let context = config.data.as_mut().unwrap();
        context.session_key = Some([0u8; 64]);

        let packet_b = "01021234567890abcdeffedcba0987654321";
        let mac = "7d671afeb16844378ec2ba55aa1fd6aa";

        // Without MAC
        assert_eq!(packet_b, hex::encode(packet.to_bytes(None).unwrap()));

        // With MAC
        assert_eq!(
            format!("{}{}", packet_b, mac),
            hex::encode(packet.to_bytes(Some(&mut config)).unwrap())
        );

        // Decode with MAC
        let deserialized = PlabbleRequestPacket::from_bytes(
            &hex::decode(format!("{}{}", packet_b, mac)).unwrap(),
            Some(&mut config),
        )
        .unwrap();

        assert_eq!(packet, deserialized);

        // Not possible to decode with wrong MAC
        let wrong = PlabbleRequestPacket::from_bytes(
            &hex::decode(format!(
                "{}{}",
                packet_b, "7d671afeb16844378ec2ba55aa1fd6ab"
            ))
            .unwrap(),
            Some(&mut config),
        );

        assert_eq!(Err(DeserializationError::IntegrityFailed), wrong);

        // Not possible to decode with wrong data
        let wrong = PlabbleRequestPacket::from_bytes(
            &hex::decode(format!("{}{}", "01021234567890abcdeffedcba0987654320", mac)).unwrap(),
            Some(&mut config),
        );

        assert_eq!(Err(DeserializationError::IntegrityFailed), wrong);

        // MAC is different when bucket key is included in AAD
        let context = config.data.as_mut().unwrap();
        context.get_bucket_key = Some(|_| Some([0u8; 32]));
        context.include_bucket_key_in_auth_data = true;

        let serialized = packet.to_bytes(Some(&mut config)).unwrap();
        assert_ne!(format!("{}{}", packet_b, mac), hex::encode(&serialized));
        let mac2 = "fe808fa93a6457bcd7e690db8de49ead";
        assert_eq!(format!("{}{}", packet_b, mac2), hex::encode(&serialized));

        let deserialized =
            PlabbleRequestPacket::from_bytes(&serialized, Some(&mut config)).unwrap();
        assert_eq!(packet, deserialized);

        // Deserialization fails if bucket key getter is not provided and bucket key is included in AAD
        let context = config.data.as_mut().unwrap();
        context.get_bucket_key = None;
        context.include_bucket_key_in_auth_data = false;

        let wrong = PlabbleRequestPacket::from_bytes(&serialized, Some(&mut config));
        assert_eq!(Err(DeserializationError::IntegrityFailed), wrong);

        // With full packet encryption, the header and MAC are encrypted (this example uses double cipher)
        let context = config.data.as_mut().unwrap();
        context.full_encryption = true;

        let mut settings = CryptoSettings::default();
        settings.encrypt_with_aes = true;
        context.crypto_settings = Some(settings);

        let encrypted = packet.to_bytes(Some(&mut config)).unwrap();
        let decrypted = PlabbleRequestPacket::from_bytes(&encrypted, Some(&mut config)).unwrap();

        assert_eq!(packet, decrypted);
    }
}
