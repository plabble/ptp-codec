use binary_codec::{
    BinaryDeserializer, BinarySerializer, BitStreamReader, BitStreamWriter, SerializerConfig,
};

use crate::{
    errors::{DeserializationError, SerializationError},
    packets::{
        base::{PlabblePacketBase, settings::CryptoSettings},
        context::PlabbleConnectionContext,
    },
};

pub mod base;
pub mod body;
pub mod context;
pub mod header;
pub mod request;
pub mod response;

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
fn read_base_packet(
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

    // If crypto settings are provided in the packet, overwrite context settings
    if let Some(settings) = &base.crypto_settings
        && let Some(ctx) = config.data.as_mut()
    {
        ctx.crypto_settings = base.crypto_settings.clone();
        settings.apply_to(config);
    } else {
        CryptoSettings::default().apply_to(config);
    }

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
fn write_base_packet(
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

    // If crypto settings are provided in the packet, overwrite context settings
    if let Some(settings) = &base.crypto_settings
        && let Some(ctx) = config.data.as_mut()
    {
        ctx.crypto_settings = base.crypto_settings.clone();
        settings.apply_to(config);
    } else {
        CryptoSettings::default().apply_to(config);
    }

    // If encryption enabled (and context provided), try set it (might overwrite the full packet encryption key, if that was the case)
    if base.use_encryption
        && let Some(ctx) = &config.data
    {
        stream.set_crypto(ctx.create_crypto_stream(Some(&base), true));
    }

    Ok(())
}
