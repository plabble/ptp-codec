use binary_codec::{BinarySerializer, DeserializationError, SerializationError, utils::slice};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::{Unpadded};

use crate::packets::base::crypto_keys::CryptoSignature;
use crate::packets::body::SerializableResponseBody;
use crate::packets::header::type_and_flags::ResponsePacketType;
use crate::packets::{base::crypto_keys::CryptoKey, body::SerializableRequestBody, header::type_and_flags::RequestPacketType};

/// Session request body
#[derive(Serialize, Deserialize)]
pub struct SessionRequestBody {
    /// PSK expiration Plabble timestamp. Filled if request flag persist_key is set.
    psk_expiration: Option<[u8; 4]>,

    /// Public/encapsulation keys for creating a shared secret with the server
    keys: Vec<CryptoKey>,
}

impl SerializableRequestBody for SessionRequestBody {
    fn to_bytes(&self, context: super::RequestSerializationContext) -> Result<Vec<u8>, SerializationError> {
        let mut bytes: Vec<u8> = Vec::new();
        if let RequestPacketType::Session { persist_key, .. } = context.header.packet_type {
            if persist_key && self.psk_expiration.is_none() {
                return Err(SerializationError::InvalidData(String::from("psk_expiration should be set if persist_key flag is set")));
            }

            if let Some(expiration_bytes) = self.psk_expiration {
                bytes.extend_from_slice(&expiration_bytes);
                context.config.pos += expiration_bytes.len();
            }
        } else {
            return Err(SerializationError::InvalidData(String::from("Header type did not match body")));
        }

        let crypto_settings = context.packet.crypto_settings.clone().unwrap_or_default();
        let key_types = CryptoKey::get_key_exchange_key_types(&crypto_settings, true);
        CryptoKey::verify_keys(key_types, &self.keys)?;

        for key in self.keys.iter() {
            key.write_bytes(&mut bytes, Some(context.config))?;
        }

        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8], context: super::RequestSerializationContext) -> Result<Self, DeserializationError> where Self: Sized {
        if let RequestPacketType::Session { persist_key, .. } = context.header.packet_type {
            let psk_expiration = if persist_key {
                Some(slice(context.config, bytes, 4, true)?.try_into().unwrap())
            } else {
                None
            };

            // TODO: get_crypto_settings method
            let crypto_settings = context.packet.crypto_settings.clone().unwrap_or_default();
            let key_types = CryptoKey::get_key_exchange_key_types(&crypto_settings, true);
            let keys = CryptoKey::read_keys(bytes, key_types, context.config)?;

            Ok(Self {
                psk_expiration,
                keys
            })
        } else {
            Err(DeserializationError::InvalidData(String::from("Header type did not match body")))
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SessionResponseBody {
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    psk_id: Option<[u8; 12]>,

    /// Public keys or encapsulated secret for creating a shared secret
    keys: Vec<CryptoKey>,
    
    /// Signatures of the request
    signatures: Vec<CryptoSignature>
}

impl SerializableResponseBody for SessionResponseBody {
    fn to_bytes(&self, context: super::ResponseSerializationContext) -> Result<Vec<u8>, SerializationError> {
        let mut bytes: Vec<u8> = Vec::new();
        if let ResponsePacketType::Session { with_psk } = context.header.packet_type {
            if with_psk && self.psk_id.is_none() {
                return Err(SerializationError::InvalidData(String::from("psk_id should be set if with_psk flag is set")));
            }

            if let Some(psk_id_bytes) = self.psk_id {
                bytes.extend_from_slice(&psk_id_bytes);
                context.config.pos += psk_id_bytes.len();
            }
        } else {
            return Err(SerializationError::InvalidData(String::from("Header type did not match body")));
        }

        let crypto_settings = context.packet.crypto_settings.clone().unwrap_or_default();

        let key_types = CryptoKey::get_key_exchange_key_types(&crypto_settings, false);
        CryptoKey::verify_keys(key_types, &self.keys)?;
        let signature_types = CryptoKey::get_signature_types(&crypto_settings);
        CryptoKey::verify_signatures(signature_types, &self.signatures)?;

        for key in self.keys.iter() {
            key.write_bytes(&mut bytes, Some(context.config))?;
        }

        for signature in self.signatures.iter() {
            signature.write_bytes(&mut bytes, Some(context.config))?;
        }

        Ok(bytes)
    }

    fn from_bytes(bytes: &[u8], context: super::ResponseSerializationContext) -> Result<Self, DeserializationError> where Self: Sized {
         if let ResponsePacketType::Session { with_psk } = context.header.packet_type {
            let psk_id = if with_psk {
                Some(slice(context.config, bytes, 12, true)?.try_into().unwrap())
            } else {
                None
            };

            // TODO: get_crypto_settings method
            let crypto_settings = context.packet.crypto_settings.clone().unwrap_or_default();
            let key_types = CryptoKey::get_key_exchange_key_types(&crypto_settings, false);
            let signature_types = CryptoKey::get_signature_types(&crypto_settings);

            let keys = CryptoKey::read_keys(bytes, key_types, context.config)?;
            let signatures = CryptoKey::read_signatures(bytes, signature_types, context.config)?;

            Ok(Self {
                psk_id,
                keys,
                signatures
            })
        } else {
            Err(DeserializationError::InvalidData(String::from("Header type did not match body")))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{base::PlabblePacketBase, body::session::SessionRequestBody, header::request_header::PlabbleRequestHeader};

    #[test]
    fn can_serialize_and_deserialize_session_request() {
        // Packet
        let base: PlabblePacketBase = toml::from_str(r#"
        version = 0
        use_encryption = true
        specify_crypto_settings = true
        crypto_settings.use_post_quantum = true
        
        [crypto_settings.post_quantum_settings]
        sign_pqc_dsa_44 = true
        key_exchange_pqc_kem_512 = true
        "#).unwrap();

        // [header]
        let header : PlabbleRequestHeader = toml::from_str(r#"
        packet_type = "Session"
        persist_key = true
        "#).unwrap();

        // [body]
        let body : SessionRequestBody = toml::from_str(r#"
        psk_expiration = [1,2,3,4]

        [[keys]]
        X25519 = "..."
        "#).unwrap();
    }
}