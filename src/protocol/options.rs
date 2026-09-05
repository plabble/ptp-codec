use crate::{crypto::SignatureAlgorithm, packets::base::settings::CryptoSettings};

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
    }
    algs
}