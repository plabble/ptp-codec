use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::packets::base::settings::CryptoSettings;

pub use crate::protocol::options::get_key_exchange_algorithms;

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Default)]
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
    /// Examples: "!x25519", "aes256", "chacha20", "!ed448", "!ed25519", "blake3", "mldsa44", "mldsa65", "mlkem512", "mlkem768"
    #[serde(default)]
    pub algorithms: Vec<String>,
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
            "ed448" => settings.sign_ed448 = set,
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

/// Check if the given algorithm name is valid.
/// - `alg` is the name of the algorithm to check.
/// - Returns `true` if the algorithm is valid, `false` otherwise.
pub fn is_valid_algorithm(alg: &str) -> bool {
    match alg {
        "x25519" | "chacha20" | "aes256" | "ed25519" | "ed448" | "blake3" | "mldsa44" | "mldsa65" | "mlkem512" | "mlkem768" => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        packets::base::settings::CryptoSettings, protocol::client::options::set_crypto_settings,
    };

    #[test]
    fn session_algorithm_overrides_enable_and_disable_settings() {
        let mut settings = CryptoSettings::default();
        set_crypto_settings(
            &mut settings,
            vec![
                "!x25519".into(),
                "aes256".into(),
                "!chacha20".into(),
                "ed448".into(),
                "mlkem512".into(),
            ],
        );

        assert!(!settings.key_exchange_x25519);
        assert!(settings.encrypt_with_aes);
        assert!(!settings.encrypt_with_chacha);
        assert!(settings.sign_ed448);
        assert!(settings.use_post_quantum);
        assert!(
            settings
                .post_quantum_settings
                .unwrap()
                .key_exchange_pqc_kem_512
        );
    }
}
