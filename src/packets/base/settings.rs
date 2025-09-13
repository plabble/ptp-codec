use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use crate::{default_true};

#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct EncryptionSettings {
    /// If true, encrypt with ChaCha20 (Poly1305).
    /// This is the default if no encryption settings are specified.
    #[serde(default = "default_true")]
    encrypt_with_cha_cha20: bool,

    /// If true, encrypt with AES-CTR or AES-GCM.
    #[serde(default)]
    encrypt_with_aes_ctr: bool,

    /// Use 32-byte hashes and MACs instead of 16-byte ones.
    #[serde(default)]
    larger_hashes: bool,

    /// Use Blake3 for hashing, MAC and key derivation instead of Blake2.
    #[serde(default)]
    use_blake3: bool,

    /// Sign with Ed25519 (default)
    #[serde(default = "default_true")]
    sign_ed25519: bool,

    /// Key exchange with X25519 (default)
    #[serde(default = "default_true")]
    key_exchange_x25519: bool,

    /// Reserved for future use
    #[serde(default)]
    flag_64: bool,

    /// Use post-quantum cryptography (e.g., Kyber etc.)
    /// This adds the Post-Quantum settings
    #[serde(default)]
    use_post_quantum: bool,

    /// Post-Quantum settings
    #[toggled_by = "use_post_quantum"]
    post_quantum_settings: Option<PostQuantumSettings>,
}

#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct PostQuantumSettings {
    /// Sign with ML-DSA-44, public key size 1312 B, signature 2420 B.
    /// Super fast, NIST level 1 security.
    #[serde(default)]
    sign_pqc_dsa_44: bool,

    /// Sign with ML-DSA-65, public key size 1952 B, signature 3309 B.
    /// Super fast, NIST level 3 security.
    #[serde(default)]
    sign_pqc_dsa_65: bool,

    /// Sign with Falcon-1024, public key size 1793 B, signature 1462 B.
    /// 3x slower than ML-DSA, NIST level 5 security.
    #[serde(default)]
    sign_pqc_falcon: bool,

    /// Sign with SLH-DSA-SHA128s, public key size 32 B, signature 7856 B. 
    /// Very slow, but might be more secure because its based on hash functions only.
    /// NIST level 1 security.
    #[serde(default)]
    sign_pqc_slh_dsa: bool,

    /// Use ML-KEM-512 for key exchange, public key size 800 B, ciphertext size 768 B
    #[serde(default)]
    key_exchange_pqc_kem_512: bool,

    /// Use ML-KEM-768 for key exchange, public key size 1184 B, ciphertext size 1088 B
    #[serde(default)]
    key_exchange_pqc_kem_768: bool,
    
    /// Reserved for future use
    #[serde(default)]
    flag_64: bool,

    /// Reserved for future use
    #[serde(default)]
    flag_128: bool,
}