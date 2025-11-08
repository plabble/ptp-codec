use binary_codec::DeserializationError;

use crate::packets::base::settings::CryptoSettings;

pub enum CryptoKey {
    ChaCha20([u8; 32]),
    Aes([u8; 32]),
    Ed25519([u8; 32]),
    X25519([u8; 32]),
    Dsa44([u8; 1312]),
    Dsa65([u8; 1952]),
    Falcon([u8; 1793]),
    SlhDsaSha128s([u8; 32]),
    Kem512([u8; 800]),
    Kem768([u8; 1184])
}

impl CryptoKey {
    pub fn read_public_encapsulation_keys(bytes: &[u8], settings: CryptoSettings, config: &mut binary_codec::SerializerConfig) -> Result<Vec<CryptoKey>, DeserializationError> {
        let mut keys = Vec::new();
        if settings.key_exchange_x25519 {
            // TODO: helper for reading n bytes, else DeserializationError::NotEnoughBytes
            // Then: .try_into().unwrap() can be done safely
        }

        Ok(keys)
    }
}