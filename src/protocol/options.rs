use crate::{
    crypto::{KeyExchangeAlgorithm, SignatureAlgorithm},
    packets::base::settings::CryptoSettings,
};

/// Get key-exchange algorithms in their protocol-defined wire order.
pub fn get_key_exchange_algorithms(settings: &CryptoSettings) -> Vec<KeyExchangeAlgorithm> {
    let mut algorithms = Vec::new();
    if settings.key_exchange_x25519 {
        algorithms.push(KeyExchangeAlgorithm::X25519);
    }
    if let Some(post_quantum) = settings.post_quantum_settings {
        if post_quantum.key_exchange_pqc_kem_512 {
            algorithms.push(KeyExchangeAlgorithm::Kem512);
        }
        if post_quantum.key_exchange_pqc_kem_768 {
            algorithms.push(KeyExchangeAlgorithm::Kem768);
        }
    }
    algorithms
}

/// Get signature algorithm according to crypto settings
pub fn get_signature_algorithms(settings: &CryptoSettings) -> Vec<SignatureAlgorithm> {
    let mut algs = Vec::new();
    if settings.sign_ed25519 {
        algs.push(SignatureAlgorithm::Ed25519);
    }
    if settings.sign_ed448 {
        algs.push(SignatureAlgorithm::Ed448);
    }
    if let Some(pq_settings) = settings.post_quantum_settings {
        if pq_settings.sign_pqc_dsa_44 {
            algs.push(SignatureAlgorithm::Dsa44);
        }
        if pq_settings.sign_pqc_dsa_65 {
            algs.push(SignatureAlgorithm::Dsa65);
        }
        if pq_settings.sign_pqc_falcon {
            algs.push(SignatureAlgorithm::Falcon);
        }
        if pq_settings.sign_pqc_slh_dsa {
            algs.push(SignatureAlgorithm::SlhDsaSha128s);
        }
    }
    algs
}

#[cfg(test)]
mod tests {
    use crate::{
        crypto::{KeyExchangeAlgorithm, SignatureAlgorithm},
        packets::base::settings::{CryptoSettings, PostQuantumSettings},
        protocol::options::{
            get_key_exchange_algorithms, get_signature_algorithms,
        },
    };

    #[test]
    fn algorithms_follow_the_wire_order_from_the_readme() {
        let settings = CryptoSettings {
            sign_ed448: true,
            use_post_quantum: true,
            post_quantum_settings: Some(PostQuantumSettings {
                sign_pqc_dsa_44: true,
                sign_pqc_dsa_65: true,
                key_exchange_pqc_kem_512: true,
                key_exchange_pqc_kem_768: true,
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            get_key_exchange_algorithms(&settings),
            vec![
                KeyExchangeAlgorithm::X25519,
                KeyExchangeAlgorithm::Kem512,
                KeyExchangeAlgorithm::Kem768,
            ]
        );
        assert_eq!(
            get_signature_algorithms(&settings),
            vec![
                SignatureAlgorithm::Ed25519,
                SignatureAlgorithm::Ed448,
                SignatureAlgorithm::Dsa44,
                SignatureAlgorithm::Dsa65,
            ]
        );
    }
}
