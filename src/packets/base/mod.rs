use binary_codec::{
    BinaryDeserializer, BinarySerializer, BitStreamReader, BitStreamWriter, FromBytes,
    SerializerConfig, ToBytes,
};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use settings::CryptoSettings;

use crate::errors::{DeserializationError, SerializationError};
use crate::packets::context::PlabbleConnectionContext;

pub mod settings;

/// Plabble Protocol Packet base
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[codec_ser_error("crate::errors::SerializationError")]
#[codec_de_error("crate::errors::DeserializationError")]
pub struct PlabblePacketBase {
    /// Plabble Protocol version
    /// 0 = debug
    #[bits = 4]
    pub version: u8,

    /// If set to true, this packet is sent outside of a session
    /// and no follow-up responses are expected.
    #[serde(default)]
    #[toggles("fire_and_forget")]
    pub fire_and_forget: bool,

    /// If set to true, this packet uses a pre-shared key for encryption.
    #[serde(default)]
    #[toggles("pre_shared_key")]
    pub pre_shared_key: bool,

    /// If set to true, this packet uses encryption. If false, add a MAC (Message Authentication Code) to the packet
    #[serde(default)]
    pub use_encryption: bool,

    /// If set to true, use custom encryption settings.
    #[serde(default)]
    #[toggles("crypto_settings")]
    pub specify_crypto_settings: bool,

    /// Encryption settings
    #[toggled_by = "crypto_settings"]
    pub crypto_settings: Option<CryptoSettings>,

    /// Pre-shared key ID, if using a pre-shared key
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    #[toggled_by = "pre_shared_key"]
    pub psk_id: Option<[u8; 12]>,

    /// Pre-shared key salt, if using a pre-shared key
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    #[toggled_by = "pre_shared_key"]
    pub psk_salt: Option<[u8; 16]>,
}

impl Default for PlabblePacketBase {
    fn default() -> Self {
        PlabblePacketBase {
            version: 1,
            fire_and_forget: false,
            pre_shared_key: false,
            use_encryption: false,
            specify_crypto_settings: false,
            crypto_settings: None,
            psk_id: None,
            psk_salt: None,
        }
    }
}

/// Helper function to start decrypting a packet by reading the base and applying crypto settings from it and/or the context
///
/// This is used by both request and response deserialization, as they share the same base structure and crypto settings application logic
/// This also sets the offset for the MAC if that is enabled, so that the packet body can be read and decrypted before verifying the MAC at the end of the packet
///
/// # Arguments
/// - `stream`: The bit stream reader to read from
/// - `config`: The optional serializer config, which may contain a context with crypto settings and
///
/// # Returns
/// - `PlabblePacketBase`: The deserialized packet base, with crypto settings applied to the stream if needed
pub fn read_base_packet(
    stream: &mut BitStreamReader,
    config: &mut SerializerConfig<PlabbleConnectionContext>,
) -> Result<PlabblePacketBase, DeserializationError> {
    // If full encryption is enabled (in provided context), try set it
    if let Some(ctx) = &config.data
        && ctx.full_encryption
    {
        stream.set_crypto(ctx.create_crypto_stream(None, true));
    }

    let base = PlabblePacketBase::read_bytes(stream, Some(config))?;

    // If the packet omits crypto settings, keep using the negotiated settings from the connection.
    apply_effective_crypto_settings(config, base.crypto_settings);

    // If encryption enabled (and context provided), try set it (might overwrite the full packet encryption key, if that was the case)
    if base.use_encryption
        && let Some(ctx) = &config.data
    {
        stream.set_crypto(ctx.create_crypto_stream(Some(&base), true));
    }

    // If MAC is enabled (and context provided), keep an offset of 16 on the reader
    if !base.use_encryption && config.data.is_some() {
        stream.set_offset_end(16);
    }

    Ok(base)
}

/// Helper function to write the base packet and apply crypto settings from it and/or the context
///
/// This is used by both request and response serialization, as they share the same base structure and crypto settings application logic
/// This also sets the crypto stream for the packet if encryption is enabled, so that the packet body can be encrypted as it is written to the stream
///
/// # Arguments
/// - `stream`: The bit stream writer to write to
/// - `base`: The packet base to write, which may contain crypto settings to apply to the stream
/// - `config`: The optional serializer config, which may contain a context with crypto settings and full encryption toggle
///
/// # Returns
/// - `Result<(), SerializationError>`: Ok if the base was written successfully, Err if writing failed
pub fn write_base_packet(
    stream: &mut BitStreamWriter,
    base: &PlabblePacketBase,
    config: &mut SerializerConfig<PlabbleConnectionContext>,
) -> Result<(), SerializationError> {
    // If full encryption is enabled (in provided context), try set it
    if let Some(ctx) = &config.data
        && ctx.full_encryption
    {
        stream.set_crypto(ctx.create_crypto_stream(None, true));
    }

    // Write base packet
    base.write_bytes(stream, Some(config))?;

    // If the packet omits crypto settings, keep using the negotiated settings from the connection.
    apply_effective_crypto_settings(config, base.crypto_settings);

    // If encryption enabled (and context provided), try set it (might overwrite the full packet encryption key, if that was the case)
    if base.use_encryption
        && let Some(ctx) = &config.data
    {
        stream.set_crypto(ctx.create_crypto_stream(Some(base), true));
    }

    Ok(())
}

/// Apply active crypto settings to serializer toggles.
///
/// Precedence:
/// 1. If the packet specifies crypto settings, they are applied immediately and persisted to context.
/// 2. Else, previously negotiated context settings are reused.
/// 3. Else, protocol defaults are used.
fn apply_effective_crypto_settings(
    config: &mut SerializerConfig<PlabbleConnectionContext>,
    packet_settings: Option<CryptoSettings>,
) {
    if let Some(ctx) = config.data.as_mut()
        && let Some(settings) = packet_settings
    {
        ctx.crypto_settings = Some(settings);
    }

    let settings = packet_settings
        .or_else(|| config.data.as_ref().and_then(|ctx| ctx.crypto_settings))
        .unwrap_or_default();
    settings.apply_to(config);
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    use crate::errors::{DeserializationError, SerializationError};
    use crate::packets::context::PlabbleConnectionContext;

    use super::*;

    #[test]
    fn can_serialize_empty_base_packet() {
        let toml = r#"
        version = 1
        fire_and_forget = true
        use_encryption = true
        "#;

        let packet: PlabblePacketBase = toml::from_str(toml).unwrap();
        let bytes = BinarySerializer::<(), SerializationError>::to_bytes(&packet, None).unwrap();

        let deserialized_packet =
            BinaryDeserializer::<(), DeserializationError>::from_bytes(&bytes, None).unwrap();
        assert_eq!(packet, deserialized_packet);
        assert_eq!(packet.fire_and_forget, true);
        assert_eq!(packet.use_encryption, true);

        // Check some defaults
        assert_eq!(packet.pre_shared_key, false);
        assert_eq!(packet.specify_crypto_settings, false);

        assert_eq!(vec![0b0101_0001], bytes);
    }

    #[test]
    fn can_serialize_packet_with_full_settings_and_psk_id() {
        let toml = r#"
        version = 1
        use_encryption = true
        pre_shared_key = true
        specify_crypto_settings = true

        psk_id = "AQIDBAUGBwgJEBES"
        psk_salt = "BwAAAAAAAAAAAAAAAAAABw"

        [crypto_settings]
        use_post_quantum = true

        [crypto_settings.post_quantum_settings]
        sign_pqc_dsa_44 = true
        "#;

        let packet: PlabblePacketBase = toml::from_str(toml).unwrap();
        let mut config = SerializerConfig::<()>::new(None);
        let bytes = packet.to_bytes(Some(&mut config)).unwrap();
        let deserialized_packet =
            BinaryDeserializer::<(), DeserializationError>::from_bytes(&bytes, None).unwrap();
        assert_eq!(packet, deserialized_packet);

        assert_eq!(config.get_toggle("fire_and_forget"), Some(false));
        assert_eq!(config.get_toggle("pre_shared_key"), Some(true));
        assert_eq!(config.get_toggle("crypto_settings"), Some(true));
        assert_eq!(config.get_toggle("dsa44"), Some(true));
        assert_eq!(config.get_toggle("dsa65"), Some(false));

        assert_eq!(
            vec![
                0b1110_0001,
                0b1011_0001,
                0b0000_0001,
                1,
                2,
                3,
                4,
                5,
                6,
                7,
                8,
                9,
                0x10,
                0x11,
                0x12,
                7,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                7
            ],
            bytes
        );
    }

    #[test]
    fn packet_crypto_settings_override_context_settings() {
        let mut config = SerializerConfig::new(Some(PlabbleConnectionContext::new()));
        let context = config.data.as_mut().unwrap();
        context.crypto_settings = Some(CryptoSettings {
            sign_ed25519: true,
            sign_ed448: false,
            key_exchange_x25519: true,
            ..Default::default()
        });

        let packet_settings = CryptoSettings {
            sign_ed25519: false,
            sign_ed448: true,
            key_exchange_x25519: false,
            ..Default::default()
        };

        apply_effective_crypto_settings(&mut config, Some(packet_settings));

        let context = config.data.as_ref().unwrap();
        assert_eq!(context.crypto_settings, Some(packet_settings));
        assert_eq!(config.get_toggle("ed25519"), Some(false));
        assert_eq!(config.get_toggle("ed448"), Some(true));
        assert_eq!(config.get_toggle("x25519"), Some(false));
    }

    #[test]
    fn context_crypto_settings_are_used_when_packet_omits_them() {
        let mut config = SerializerConfig::new(Some(PlabbleConnectionContext::new()));
        let context_settings = CryptoSettings {
            sign_ed25519: false,
            sign_ed448: true,
            key_exchange_x25519: false,
            ..Default::default()
        };
        config.data.as_mut().unwrap().crypto_settings = Some(context_settings);

        apply_effective_crypto_settings(&mut config, None);

        assert_eq!(
            config.data.as_ref().unwrap().crypto_settings,
            Some(context_settings)
        );
        assert_eq!(config.get_toggle("ed25519"), Some(false));
        assert_eq!(config.get_toggle("ed448"), Some(true));
        assert_eq!(config.get_toggle("x25519"), Some(false));
    }
}
