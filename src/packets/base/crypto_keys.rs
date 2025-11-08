use binary_codec::{DeserializationError, SerializationError, ToBytes, utils::slice};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::{Unpadded};

use crate::packets::base::settings::CryptoSettings;

#[serde_as]
#[derive(ToBytes, Serialize, Deserialize)]
#[no_discriminator]
pub enum CryptoKey {
    ChaCha20(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 32]),
    Aes(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 32]),
    Ed25519(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 32]),
    X25519(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 32]),
    Dsa44(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1312]),
    Dsa65(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1952]),
    Falcon(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1793]),
    SlhDsaSha128s(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 32]),
    Kem512(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 800]),
    Kem512Secret(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 768]),
    Kem768(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1184]),
    Kem768Secret(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1088])
}

#[serde_as]
#[derive(ToBytes, Serialize, Deserialize)]
#[no_discriminator]
pub enum CryptoSignature {
    Ed25519(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 64]),
    Dsa44(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 2420]),
    Dsa65(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 3309]),
    Falcon(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1462]),
    SlhDsaSha128s(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 7856]),
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Algorithm {
    ChaCha20,
    Aes,
    Ed25519,
    X25519,
    Dsa44,
    Dsa65,
    Falcon,
    SlhDsaSha128s,
    Kem512Key,
    Kem512Secret,
    Kem768Key,
    Kem768Secret
}

impl CryptoKey {
    pub fn read_keys(bytes: &[u8], key_types: Vec<Algorithm>, config: &mut binary_codec::SerializerConfig) -> Result<Vec<CryptoKey>, DeserializationError> {
        let mut keys = Vec::new();
        for key_type in key_types {
            keys.push(match key_type {
                Algorithm::X25519 => Self::read_fixed_n(config, bytes, CryptoKey::X25519),
                Algorithm::ChaCha20 => Self::read_fixed_n(config, bytes, CryptoKey::ChaCha20),
                Algorithm::Aes => Self::read_fixed_n(config, bytes, CryptoKey::Aes),
                Algorithm::Ed25519 => Self::read_fixed_n(config, bytes, CryptoKey::Ed25519),
                Algorithm::Dsa44 => Self::read_fixed_n(config, bytes, CryptoKey::Dsa44),
                Algorithm::Dsa65 => Self::read_fixed_n(config, bytes, CryptoKey::Dsa65),
                Algorithm::Falcon => Self::read_fixed_n(config, bytes, CryptoKey::Falcon),
                Algorithm::SlhDsaSha128s => Self::read_fixed_n(config, bytes, CryptoKey::SlhDsaSha128s),
                Algorithm::Kem512Key => Self::read_fixed_n(config, bytes, CryptoKey::Kem512),
                Algorithm::Kem768Key => Self::read_fixed_n(config, bytes, CryptoKey::Kem768),
                Algorithm::Kem512Secret => Self::read_fixed_n(config, bytes, CryptoKey::Kem512Secret),
                Algorithm::Kem768Secret => Self::read_fixed_n(config, bytes, CryptoKey::Kem768Secret),
            }?);
        }

        Ok(keys)
    }

    pub fn read_signatures(bytes: &[u8], signature_types: Vec<Algorithm>, config: &mut binary_codec::SerializerConfig) -> Result<Vec<CryptoSignature>, DeserializationError> {
        let mut signatures = Vec::new();
        for signature_type in signature_types {
            signatures.push(match signature_type {
                Algorithm::Ed25519 => Self::read_fixed_n(config, bytes, CryptoSignature::Ed25519),
                Algorithm::Dsa44 => Self::read_fixed_n(config, bytes, CryptoSignature::Dsa44),
                Algorithm::Dsa65 => Self::read_fixed_n(config, bytes, CryptoSignature::Dsa65),
                Algorithm::Falcon => Self::read_fixed_n(config, bytes, CryptoSignature::Falcon),
                Algorithm::SlhDsaSha128s => Self::read_fixed_n(config, bytes, CryptoSignature::SlhDsaSha128s),
                other => Err(DeserializationError::InvalidData(format!("{:?} is not a signature algorithm", other)))
            }?);
        }

        Ok(signatures)
    }

    pub fn verify_keys(expected: Vec<Algorithm>, actual: &Vec<CryptoKey>) -> Result<(), SerializationError> {
        for (i, key) in actual.iter().enumerate() {
            let variant = match key {
                CryptoKey::X25519(_) => Algorithm::X25519,
                CryptoKey::Kem512(_) => Algorithm::Kem512Key,
                CryptoKey::Kem768(_) => Algorithm::Kem768Key,
                CryptoKey::ChaCha20(_) => Algorithm::ChaCha20,
                CryptoKey::Aes(_) => Algorithm::Aes,
                CryptoKey::Ed25519(_) => Algorithm::Ed25519,
                CryptoKey::Dsa44(_) => Algorithm::Dsa44,
                CryptoKey::Dsa65(_) => Algorithm::Dsa65,
                CryptoKey::Falcon(_) => Algorithm::Falcon,
                CryptoKey::SlhDsaSha128s(_) => Algorithm::SlhDsaSha128s,
                CryptoKey::Kem512Secret(_) => Algorithm::Kem512Secret,
                CryptoKey::Kem768Secret(_) => Algorithm::Kem768Secret,
            };

            if expected.get(i).cloned() != Some(variant) {
                return Err(SerializationError::InvalidData(format!(
                    "Unexpected algorithm at position {}: expected type {:?}, got {:?}",
                    i, expected.get(i), variant
                )));
            }
        }

        if expected.len() != actual.len() {
            return Err(SerializationError::InvalidData(String::from("Missing required algorithms")));
        }

        Ok(())
    }

    pub fn verify_signatures(expected: Vec<Algorithm>, actual: &Vec<CryptoSignature>) -> Result<(), SerializationError> {
        for (i, signature) in actual.iter().enumerate() {
            let variant = match signature {
                CryptoSignature::Ed25519(_) => Algorithm::Ed25519,
                CryptoSignature::Dsa44(_) => Algorithm::Dsa44,
                CryptoSignature::Dsa65(_) => Algorithm::Dsa65,
                CryptoSignature::Falcon(_) => Algorithm::Falcon,
                CryptoSignature::SlhDsaSha128s(_) => Algorithm::SlhDsaSha128s,
            };

            if expected.get(i).cloned() != Some(variant) {
                return Err(SerializationError::InvalidData(format!(
                    "Unexpected algorithm at position {}: expected type {:?}, got {:?}",
                    i, expected.get(i), variant
                )));
            }
        }

        if expected.len() != actual.len() {
            return Err(SerializationError::InvalidData(String::from("Missing required algorithms")));
        }

        Ok(())
    }

    pub fn get_key_exchange_key_types(settings: &CryptoSettings, is_request: bool) -> Vec<Algorithm> {
        let mut expected = Vec::new();
        if settings.key_exchange_x25519 {
            expected.push(Algorithm::X25519);
        }

        // For KEM, we do not need the response to also contain an encapsulation key
        if settings.use_post_quantum && let Some(settings) = &settings.post_quantum_settings {
            if settings.key_exchange_pqc_kem_512 {
                expected.push(if is_request { Algorithm::Kem512Key } else { Algorithm::Kem512Secret });
            }

            if settings.key_exchange_pqc_kem_768 {
                expected.push(if is_request { Algorithm::Kem768Key } else { Algorithm::Kem768Secret });
            }
        }

        expected
    }

    pub fn get_signature_types(settings: &CryptoSettings) -> Vec<Algorithm> {
        let mut expected = Vec::new();
        if settings.sign_ed25519 {
            expected.push(Algorithm::Ed25519);
        }

        if settings.use_post_quantum && let Some(settings) = &settings.post_quantum_settings {
            if settings.sign_pqc_dsa_44 {
                expected.push(Algorithm::Dsa44);
            }
            if settings.sign_pqc_dsa_65 {
                expected.push(Algorithm::Dsa65);
            }
            if settings.sign_pqc_falcon {
                expected.push(Algorithm::Falcon);
            }
            if settings.sign_pqc_slh_dsa {
                expected.push(Algorithm::SlhDsaSha128s);
            }
        }

        expected
    }

    fn read_fixed_n<const N: usize, F, C>(
        config: &mut binary_codec::SerializerConfig,
        bytes: &[u8],
        constructor: F,
    ) -> Result<C, DeserializationError>
    where
        F: FnOnce([u8; N]) -> C,
    {
        let data: [u8; N] = slice(config, bytes, N, true)?
            .try_into()
            .unwrap();
        Ok(constructor(data))
    }
}