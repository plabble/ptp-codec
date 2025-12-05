use binary_codec::{DeserializationError, SerializationError, ToBytes, utils::slice};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::packets::base::settings::CryptoSettings;

/// Cryptographic keys used in various algorithms
/// The keys are stored as fixed-size byte arrays, serialized/deserialized using base64 encoding (when using serde)
/// 
/// # Variants:
/// - ChaCha20: 32 bytes secret key for ChaCha20 or ChaCha20-Poly1305 encryption
/// - Aes: 32 bytes secret key for AES-CTR/AES-GCM encryption
/// - Ed25519: 32 bytes public key for validating Ed25519 signatures
/// - X25519: 32 bytes public key for X25519 key exchange
/// - Dsa44: 1312 bytes public key for DSA-44 post-quantum signatures
/// - Dsa65: 1952 bytes public key for DSA-65 post-quantum signatures
/// - Falcon: 1793 bytes public key for Falcon post-quantum signatures
/// - SlhDsaSha128s: 32 bytes public key for SLH-DSA-SHA128s post-quantum signatures
/// - Kem512: 800 bytes public key for KEM-512 post-quantum key exchange
/// - Kem512Cipher: 768 bytes encapsulated key for KEM-512 post-quantum key exchange
/// - Kem768: 1184 bytes public key for KEM-768 post-quantum key exchange
/// - Kem768Cipher: 1088 bytes encapsulated key for KEM-768 post-quantum key exchange
#[serde_as]
#[derive(ToBytes, Serialize, Deserialize, Debug, PartialEq)]
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
    Kem512Cipher(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 768]),
    Kem768(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1184]),
    Kem768Cipher(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1088]),
}

/// Cryptographic signatures used in various algorithms
/// The signatures are stored as fixed-size byte arrays, serialized/deserialized using base64 encoding (when using serde)
/// 
/// # Variants:
/// - Ed25519: 64 bytes signature for Ed25519 signatures
/// - Dsa44: 2420 bytes signature for DSA-44 post-quantum signatures
/// - Dsa65: 3309 bytes signature for DSA-65 post-quantum signatures
/// - Falcon: 1462 bytes signature for Falcon post-quantum signatures
/// - SlhDsaSha128s: 7856 bytes signature for SLH-DSA-SHA128s post-quantum signatures
#[serde_as]
#[derive(ToBytes, Serialize, Deserialize, Debug, PartialEq)]
#[no_discriminator]
pub enum CryptoSignature {
    Ed25519(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 64]),
    Dsa44(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 2420]),
    Dsa65(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 3309]),
    Falcon(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1462]),
    SlhDsaSha128s(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 7856]),
}

/// Enum of supported cryptographic algorithms
/// 
/// # Variants:
/// - ChaCha20: ChaCha20 or ChaCha20-Poly1305 encryption
/// - Aes: AES-CTR/AES-GCM encryption
/// - Ed25519: Ed25519 signatures
/// - X25519: X25519 key exchange
/// - Dsa44: DSA-44 post-quantum signatures
/// - Dsa65: DSA-65 post-quantum signatures
/// - Falcon: Falcon post-quantum signatures
/// - SlhDsaSha128s: SLH-DSA-SHA128s post-quantum signatures
/// - Kem512Key: KEM-512 post-quantum key exchange (public key)
/// - Kem512Cipher: KEM-512 post-quantum key exchange (encapsulated key)
/// - Kem768Key: KEM-768 post-quantum key exchange (public key)
/// - Kem768Cipher: KEM-768 post-quantum key exchange (encapsulated key)
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
    Kem512Cipher,
    Kem768Key,
    Kem768Cipher,
}

/// CryptoKey implementation
impl CryptoKey {
    /// Reads multiple CryptoKeys from bytes based on the provided key types
    /// 
    /// # Arguments
    /// - `bytes`: The byte slice to read from
    /// - `key_types`: A vector of Algorithm variants indicating which keys to read
    /// - `config`: The serializer configuration to use
    /// 
    /// # Returns
    /// A Result containing a vector of CryptoKeys or a DeserializationError
    pub fn read_keys(
        bytes: &[u8],
        key_types: Vec<Algorithm>,
        config: &mut binary_codec::SerializerConfig,
    ) -> Result<Vec<CryptoKey>, DeserializationError> {
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
                Algorithm::SlhDsaSha128s => {
                    Self::read_fixed_n(config, bytes, CryptoKey::SlhDsaSha128s)
                }
                Algorithm::Kem512Key => Self::read_fixed_n(config, bytes, CryptoKey::Kem512),
                Algorithm::Kem768Key => Self::read_fixed_n(config, bytes, CryptoKey::Kem768),
                Algorithm::Kem512Cipher => {
                    Self::read_fixed_n(config, bytes, CryptoKey::Kem512Cipher)
                }
                Algorithm::Kem768Cipher => {
                    Self::read_fixed_n(config, bytes, CryptoKey::Kem768Cipher)
                }
            }?);
        }

        Ok(keys)
    }

    /// Reads multiple CryptoSignatures from bytes based on the provided signature types
    /// 
    /// # Arguments
    /// - `bytes`: The byte slice to read from
    /// - `signature_types`: A vector of Algorithm variants indicating which signatures to read
    /// - `config`: The serializer configuration to use
    /// 
    /// # Returns
    /// A Result containing a vector of CryptoSignatures or a DeserializationError
    pub fn read_signatures(
        bytes: &[u8],
        signature_types: Vec<Algorithm>,
        config: &mut binary_codec::SerializerConfig,
    ) -> Result<Vec<CryptoSignature>, DeserializationError> {
        let mut signatures = Vec::new();
        for signature_type in signature_types {
            signatures.push(match signature_type {
                Algorithm::Ed25519 => Self::read_fixed_n(config, bytes, CryptoSignature::Ed25519),
                Algorithm::Dsa44 => Self::read_fixed_n(config, bytes, CryptoSignature::Dsa44),
                Algorithm::Dsa65 => Self::read_fixed_n(config, bytes, CryptoSignature::Dsa65),
                Algorithm::Falcon => Self::read_fixed_n(config, bytes, CryptoSignature::Falcon),
                Algorithm::SlhDsaSha128s => {
                    Self::read_fixed_n(config, bytes, CryptoSignature::SlhDsaSha128s)
                }
                other => Err(DeserializationError::InvalidData(format!(
                    "{:?} is not a signature algorithm",
                    other
                ))),
            }?);
        }

        Ok(signatures)
    }

    /// Verifies that the provided CryptoKeys match the expected algorithms
    /// 
    /// # Arguments
    /// - `expected`: A vector of Algorithm variants indicating the expected key types
    /// - `actual`: A reference to a vector of CryptoKeys to verify
    /// 
    /// # Returns
    /// A Result indicating success or a SerializationError if verification fails
    pub fn verify_keys(
        expected: Vec<Algorithm>,
        actual: &Vec<CryptoKey>,
    ) -> Result<(), SerializationError> {
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
                CryptoKey::Kem512Cipher(_) => Algorithm::Kem512Cipher,
                CryptoKey::Kem768Cipher(_) => Algorithm::Kem768Cipher,
            };

            if expected.get(i).cloned() != Some(variant) {
                return Err(SerializationError::InvalidData(format!(
                    "Unexpected algorithm at position {}: expected type {:?}, got {:?}",
                    i,
                    expected.get(i),
                    variant
                )));
            }
        }

        if expected.len() != actual.len() {
            return Err(SerializationError::InvalidData(String::from(
                "Missing required algorithms",
            )));
        }

        Ok(())
    }

    /// Verifies that the provided CryptoSignatures match the expected algorithms
    /// 
    /// # Arguments
    /// - `expected`: A vector of Algorithm variants indicating the expected signature types
    /// - `actual`: A reference to a vector of CryptoSignatures to verify
    /// 
    /// # Returns
    /// A Result indicating success or a SerializationError if verification fails
    pub fn verify_signatures(
        expected: Vec<Algorithm>,
        actual: &Vec<CryptoSignature>,
    ) -> Result<(), SerializationError> {
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
                    i,
                    expected.get(i),
                    variant
                )));
            }
        }

        if expected.len() != actual.len() {
            return Err(SerializationError::InvalidData(String::from(
                "Missing required algorithms",
            )));
        }

        Ok(())
    }

    /// Helper function to get expected key exchange types based on settings
    /// 
    /// # Arguments
    /// - `settings`: Reference to CryptoSettings
    /// - `is_request`: Boolean indicating if it's for a request (true) or response (false)
    /// 
    /// # Returns
    /// A vector of Algorithm variants indicating the expected key exchange types
    pub fn get_key_exchange_key_types(
        settings: &CryptoSettings,
        is_request: bool,
    ) -> Vec<Algorithm> {
        let mut expected = Vec::new();
        if settings.key_exchange_x25519 {
            expected.push(Algorithm::X25519);
        }

        // For KEM, we do not need the response to also contain an encapsulation key
        if settings.use_post_quantum
            && let Some(settings) = &settings.post_quantum_settings
        {
            if settings.key_exchange_pqc_kem_512 {
                expected.push(if is_request {
                    Algorithm::Kem512Key
                } else {
                    Algorithm::Kem512Cipher
                });
            }

            if settings.key_exchange_pqc_kem_768 {
                expected.push(if is_request {
                    Algorithm::Kem768Key
                } else {
                    Algorithm::Kem768Cipher
                });
            }
        }

        expected
    }

    /// Helper function to get expected signature types based on settings
    /// 
    /// # Arguments
    /// - `settings`: Reference to CryptoSettings
    /// 
    /// # Returns
    /// A vector of Algorithm variants indicating the expected signature types
    pub fn get_signature_types(settings: &CryptoSettings) -> Vec<Algorithm> {
        let mut expected = Vec::new();
        if settings.sign_ed25519 {
            expected.push(Algorithm::Ed25519);
        }

        if settings.use_post_quantum
            && let Some(settings) = &settings.post_quantum_settings
        {
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

    /// Helper function to read fixed amount of bytes
    /// 
    /// # Arguments
    /// - `config`: The serializer configuration to use
    /// - `bytes`: The byte slice to read from
    /// - `constructor`: A function that constructs the desired type from a fixed-size byte array
    /// 
    /// # Returns
    /// A Result containing the constructed type or a DeserializationError
    fn read_fixed_n<const N: usize, F, C>(
        config: &mut binary_codec::SerializerConfig,
        bytes: &[u8],
        constructor: F,
    ) -> Result<C, DeserializationError>
    where
        F: FnOnce([u8; N]) -> C,
    {
        let data: [u8; N] = slice(config, bytes, N, true)?.try_into().unwrap();
        Ok(constructor(data))
    }
}
