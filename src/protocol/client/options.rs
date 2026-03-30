use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::crypto::KeyExchangeAlgorithm;
use crate::packets::base::settings::CryptoSettings;

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
pub struct SessionOptions {
    /// If true, switch full packet encryption after key exchange
    #[serde(default)]
    pub enable_full_encryption: bool,

    /// Stored key lifetime (PSK) in seconds from now, if you want to save the session key for future use.
    /// If not set, the session key will only be used for the current session.
    #[serde(default)]
    pub stored_key_lifetime: Option<u32>,

    /// If true, the client will generate a random salt and include it in the session request.
    #[serde(default)]
    pub client_salt: bool,

    /// If true, the server will generate a random salt and include it in the session response.
    #[serde(default)]
    pub server_salt: bool,

    /// If set, the Session packets will be encrypted using a earlier pre-shared key
    #[serde(default)]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    pub psk_id: Option<[u8; 12]>,

    /// List of cryptographic algorithm names (lowercase) to (not) use. If emtpy, default crypto settings will be used
    /// Examples: "!x25519", "aes256", "chacha20", "!ed25519", "blake3", "mldsa44", "mldsa65", "mlkem512", "mlkem768"
    #[serde(default)]
    pub algorithms: Vec<String>,
}

impl Default for SessionOptions {
    fn default() -> Self {
        SessionOptions {
            enable_full_encryption: false,
            stored_key_lifetime: None,
            client_salt: false,
            server_salt: false,
            psk_id: None,
            algorithms: Vec::new(),
        }
    }
}

/// Set crypto settings based on algorithm string list
pub fn set_crypto_settings(settings: &mut CryptoSettings, algorithms: Vec<String>) {
    for alg in algorithms {
        let set = !alg.starts_with('!');
        let alg = alg.trim_start_matches('!');
        match alg {
            "x25519" => settings.key_exchange_x25519 = set,
            "chacha20" => settings.encrypt_with_chacha = set,
            "aes256" => settings.encrypt_with_aes = set,
            "ed25519" => settings.sign_ed25519 = set,
            "blake3" => settings.use_blake3 = set,
            "mldsa44" | "mldsa65" | "mlkem512" | "mlkem768" => {
                let mut pq_settings = settings.post_quantum_settings.unwrap_or_default();
                settings.use_post_quantum = true;

                match alg {
                    "mldsa44" => pq_settings.sign_pqc_dsa_44 = set,
                    "mldsa65" => pq_settings.sign_pqc_dsa_65 = set,
                    "mlkem512" => pq_settings.key_exchange_pqc_kem_512 = set,
                    "mlkem768" => pq_settings.key_exchange_pqc_kem_768 = set,
                    _ => {}
                }
                settings.post_quantum_settings = Some(pq_settings);
            }
            _ => {}
        }
    }
}

/// Get key exchange algorithm according to crypto settings
pub fn get_key_exchange_algorithms(settings: &CryptoSettings) -> Vec<KeyExchangeAlgorithm> {
    let mut algs = Vec::new();
    if settings.key_exchange_x25519 {
        algs.push(KeyExchangeAlgorithm::X25519);
    }
    if let Some(pq_settings) = settings.post_quantum_settings {
        if pq_settings.key_exchange_pqc_kem_512 {
            algs.push(KeyExchangeAlgorithm::Kem512);
        }
        if pq_settings.key_exchange_pqc_kem_768 {
            algs.push(KeyExchangeAlgorithm::Kem768);
        }
    }
    algs
}
