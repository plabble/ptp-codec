mod key_exchange;
mod signatures;
mod encryption;

/// Supported key-exchange algorithms.
///
/// - `X25519` is the standard Diffie-Hellman over Curve25519.
/// - `Kem512` / `Kem768` are optional post-quantum KEM algorithms
///   provided when the `pqc-lite` feature is enabled.
pub enum KeyExchangeAlgorithm {
    X25519,
    Kem512,
    Kem768,
}

/// A small stateful helper to run a key exchange for a chosen algorithm.
///
/// The struct stores the selected algorithm and, for initiators, the
/// secret material generated when creating a request. The secret is kept
/// as raw bytes so it can be used by the `process_response` method to
/// compute the final shared secret.
pub struct KeyExchange {
    algorithm: KeyExchangeAlgorithm,
    secret: Option<Vec<u8>>,
}

pub mod algorithm;
pub mod certificate;
