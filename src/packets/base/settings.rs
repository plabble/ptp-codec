use crate::default_true;
use binary_codec::{FromBytes, SerializerConfig, ToBytes};
use serde::{Deserialize, Serialize};

/// Cryptography settings for a session, request or response
#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct CryptoSettings {
    /// If true, encrypt with ChaCha20 and ChaCha20-Poly1305
    /// This is the default if no encryption settings are specified.
    #[serde(default = "default_true")]
    pub encrypt_with_cha_cha20: bool,

    /// If true, encrypt with AES-CTR and AES-GCM.
    #[serde(default)]
    pub encrypt_with_aes: bool,

    /// Reserved for future use
    #[serde(default)]
    pub flag_4: bool,

    /// Use Blake3 for hashing, MAC and key derivation instead of Blake2.
    #[serde(default)]
    pub use_blake3: bool,

    /// Sign with Ed25519 (default), 32 B keys, signature 64 B.
    #[serde(default = "default_true")]
    #[toggles("ed25519")]
    pub sign_ed25519: bool,

    /// Key exchange with X25519 (default), 32 B keys.
    #[serde(default = "default_true")]
    #[toggles("x25519")]
    pub key_exchange_x25519: bool,

    /// Reserved for future use
    #[serde(default)]
    pub flag_64: bool,

    /// Use post-quantum cryptography (e.g., Kyber etc.)
    /// This adds the Post-Quantum settings
    #[serde(default)]
    #[toggles("pqc")]
    pub use_post_quantum: bool,

    /// Post-Quantum settings
    #[toggled_by = "pqc"]
    pub post_quantum_settings: Option<PostQuantumSettings>,
}

/// Post-Quantum cryptography settings
#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PostQuantumSettings {
    /// Sign with ML-DSA-44, public key size 1312 B, signature 2420 B.
    /// Super fast, NIST level 1 security.
    #[serde(default)]
    #[toggles("dsa44")]
    pub sign_pqc_dsa_44: bool,

    /// Sign with ML-DSA-65, public key size 1952 B, signature 3309 B.
    /// Super fast, NIST level 3 security.
    #[serde(default)]
    #[toggles("dsa65")]
    pub sign_pqc_dsa_65: bool,

    /// Sign with Falcon-1024, public key size 1793 B, signature 1462 B.
    /// 3x slower than ML-DSA, NIST level 5 security.
    #[serde(default)]
    #[toggles("falcon")]
    pub sign_pqc_falcon: bool,

    /// Sign with SLH-DSA-SHA128s, public key size 32 B, signature 7856 B.
    /// Very slow, but might be more secure because its based on hash functions only.
    /// NIST level 1 security.
    #[serde(default)]
    #[toggles("slh_dsa")]
    pub sign_pqc_slh_dsa: bool,

    /// Use ML-KEM-512 for key exchange, public key size 800 B, ciphertext size 768 B
    #[serde(default)]
    #[toggles("kem512")]
    pub key_exchange_pqc_kem_512: bool,

    /// Use ML-KEM-768 for key exchange, public key size 1184 B, ciphertext size 1088 B
    #[serde(default)]
    #[toggles("kem768")]
    pub key_exchange_pqc_kem_768: bool,

    /// Reserved for future use
    #[serde(default)]
    pub flag_64: bool,

    /// Reserved for future use
    #[serde(default)]
    pub flag_128: bool,
}

impl CryptoSettings {
    pub fn apply_defaults<T: Clone>(config: &mut SerializerConfig<T>) {
        let defaults = Self::default();
        if defaults.sign_ed25519 {
            config.set_toggle("ed25519", true);
        }

        if defaults.key_exchange_x25519 {
            config.set_toggle("x25519", true);
        }
    }
}

impl Default for CryptoSettings {
    fn default() -> Self {
        Self {
            encrypt_with_cha_cha20: true,
            encrypt_with_aes: false,
            flag_4: false,
            use_blake3: false,
            sign_ed25519: true,
            key_exchange_x25519: true,
            flag_64: false,
            use_post_quantum: false,
            post_quantum_settings: None,
        }
    }
}

impl Default for PostQuantumSettings {
    fn default() -> Self {
        Self {
            sign_pqc_dsa_44: false,
            sign_pqc_dsa_65: false,
            sign_pqc_falcon: false,
            sign_pqc_slh_dsa: false,
            key_exchange_pqc_kem_512: false,
            key_exchange_pqc_kem_768: false,
            flag_64: false,
            flag_128: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    #[test]
    fn can_serialize_encryption_settings_with_pqc() {
        let toml = r#"
        encrypt_with_cha_cha20 = true
        encrypt_with_aes = false
        flag_4 = true
        use_blake3 = false
        sign_ed25519 = true
        key_exchange_x25519 = true
        use_post_quantum = true

        [post_quantum_settings]
        sign_pqc_dsa_44 = true
        sign_pqc_dsa_65 = false
        sign_pqc_falcon = true
        sign_pqc_slh_dsa = false
        key_exchange_pqc_kem_512 = true
        key_exchange_pqc_kem_768 = false
        "#;

        let settings: CryptoSettings = toml::from_str(toml).unwrap();
        let mut config = SerializerConfig::<()>::new(None);
        let bytes = settings.to_bytes(Some(&mut config)).unwrap();

        let deserialized_settings = BinaryDeserializer::<()>::from_bytes(&bytes, None).unwrap();
        assert_eq!(settings, deserialized_settings);
        assert_eq!(vec![0b1011_0101, 0b0001_0101], bytes);

        assert_eq!(config.get_toggle("ed25519"), Some(true));
        assert_eq!(config.get_toggle("x25519"), Some(true));
        assert_eq!(config.get_toggle("dsa44"), Some(true));
        assert_eq!(config.get_toggle("dsa65"), Some(false));
        assert_eq!(config.get_toggle("falcon"), Some(true));
        assert_eq!(config.get_toggle("slh_dsa"), Some(false));
        assert_eq!(config.get_toggle("kem512"), Some(true));
        assert_eq!(config.get_toggle("kem768"), Some(false));

        // Not yet added toggles
        assert_eq!(config.get_toggle("chacha20"), None);
        assert_eq!(config.get_toggle("aes"), None);
        assert_eq!(config.get_toggle("blake3"), None);
    }

    #[test]
    fn can_serialize_encryption_settings_without_pqc_and_with_defaults() {
        let toml = r#"
        encrypt_with_aes = true
        flag_4 = false
        use_blake3 = true
        use_post_quantum = false
        "#;

        let settings: CryptoSettings = toml::from_str(toml).unwrap();
        let bytes = BinarySerializer::<()>::to_bytes(&settings, None).unwrap();

        let deserialized_settings = BinaryDeserializer::<()>::from_bytes(&bytes, None).unwrap();
        assert_eq!(settings, deserialized_settings);
        assert_eq!(settings.encrypt_with_cha_cha20, true);
        assert_eq!(settings.sign_ed25519, true);
        assert_eq!(settings.key_exchange_x25519, true);
        assert_eq!(settings.flag_64, false);

        assert_eq!(vec![0b0011_1011], bytes);
    }
}
