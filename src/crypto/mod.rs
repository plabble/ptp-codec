use blake2::{Blake2b512, Blake2bMac512, digest::Mac};

mod encryption;
mod key_exchange;
mod signatures;

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

/// Derive a cryptographic key based on crypto settings (blake2b-512 or blake3)
///
/// # Properties
/// - `blake3`: whether to use Blake3 or Blake2b-512
/// - `ikm`: Input key material (cryptographic key)
/// - `salt`: 16-byte salt
/// - `context`: 16-byte context
///
/// Returns None if blake3 is not supported or hash function failed
/// TODO: check compat with Geralt/libsodium https://www.geralt.xyz/message-authentication
pub fn derive_key(
    blake3: bool,
    ikm: &[u8; 64],
    salt: &[u8; 16],
    context: &[u8; 16],
) -> Option<[u8; 64]> {
    #[cfg(feature = "blake-3")]
    if blake3 {
        use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
        use blake3::Hasher;

        // Encode context with base64-url (no padding)
        let mut kdf = Hasher::new_derive_key(&BASE64_URL_SAFE_NO_PAD.encode(context));
        kdf.update(ikm);
        kdf.update(salt);

        let mut out = [0u8; 64];
        kdf.finalize_xof().fill(&mut out);
        return Some(out);
    }

    #[cfg(not(feature = "blake-3"))]
    if blake3 {
        return None;
    }

    // TODO: is Mac mode correct here? Libsodium?
    
    let kdf = Blake2bMac512::new_with_salt_and_personal(ikm, salt, context).ok()?;

    Some(kdf.finalize().into_bytes().into())
}

pub mod algorithm;
pub mod certificate;
