use blake2::{Blake2bMac512, digest::Mac};

pub mod algorithm;
pub mod certificate;
pub mod encryption;
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
/// - `extra_key`: optional 64-byte extra key to put into the hash
///
/// Returns None if blake3 is not supported or hash function failed
pub fn derive_key(
    blake3: bool,
    ikm: &[u8; 64],
    salt: &[u8; 16],
    context: &[u8; 16],
    extra_key: Option<&[u8; 64]>,
) -> Option<[u8; 64]> {
    #[cfg(feature = "blake-3")]
    if blake3 {
        use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
        use blake3::Hasher;

        // Encode context with base64-url (no padding)
        let mut kdf = Hasher::new_derive_key(&BASE64_URL_SAFE_NO_PAD.encode(context));
        kdf.update(ikm);
        kdf.update(salt);

        if let Some(extra_key) = extra_key {
            kdf.update(extra_key);
        }

        let mut out = [0u8; 64];
        kdf.finalize_xof().fill(&mut out);
        return Some(out);
    }

    #[cfg(not(feature = "blake-3"))]
    if blake3 {
        return None;
    }

    // This is called Mac, but is actually useful for key derivation because of salt/personalization
    let mut kdf = Blake2bMac512::new_with_salt_and_personal(ikm, salt, context).ok()?;
    if let Some(extra_key) = extra_key {
        kdf.update(extra_key);
    }

    Some(kdf.finalize().into_bytes().into())
}

#[cfg(test)]
mod tests {
    use base64::{Engine, prelude::BASE64_STANDARD};

    use crate::crypto::derive_key;

    #[test]
    fn can_derive_blake2b_key() {
        let ikm = [1u8; 64];
        let personal = [2u8; 16];
        let salt = [3u8; 16];

        let res = derive_key(false, &ikm, &salt, &personal, None).unwrap();

        // This hash is exactly the same as Geralt (for .NET) produces, which is a wrapper around libsodium
        // https://www.geralt.xyz/key-derivation
        // so this test ensures that "Blake2bMac512" is the same as the KDF mode of libsodium

        assert_eq!(
            BASE64_STANDARD.encode(&res),
            "PiPfpmZbmso8hQM8U/pqeVzJ0C9THDubc5aultGQ4W5brnHKOWBf008vmBxodvL62BLIU5LSvXn+icjRou7MBw=="
        )
    }
}
