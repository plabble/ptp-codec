use blake2::{
    Blake2b, Blake2bMac, Digest,
    digest::{
        Mac,
        consts::{U16, U24, U32, U64},
    },
};

pub mod algorithm;
pub mod certificate;
pub mod encryption;
#[cfg(feature = "protocol")]
mod key_exchange;
mod signatures;

type Blake2b128 = Blake2b<U16>;
type Blake2b192 = Blake2b<U24>;
type Blake2b256 = Blake2b<U32>;
type Blake2b512 = Blake2b<U64>;
type Blake2bMac128 = Blake2bMac<U16>;
type Blake2bMac512 = Blake2bMac<U64>;

macro_rules! impl_hash {
    ($name:ident, $size:expr, $blake2_type:ty, $blake3_finalize:expr) => {
        pub fn $name(blake3: bool, data: Vec<&[u8]>) -> [u8; $size] {
            #[cfg(not(feature = "blake-3"))]
            if blake3 {
                panic!("Blake3 is not supported when the 'blake-3' feature is not enabled");
            }

            #[cfg(feature = "blake-3")]
            if blake3 {
                use blake3::Hasher;
                let mut hasher = Hasher::new();
                for d in data {
                    hasher.update(d);
                }
                return $blake3_finalize(hasher);
            }

            let mut hasher = <$blake2_type>::new();
            for d in data {
                hasher.update(d);
            }
            hasher.finalize().into()
        }
    };
}

/// Supported key-exchange algorithms.
///
/// - `X25519` is the standard Diffie-Hellman over Curve25519.
/// - `Kem512` / `Kem768` are optional post-quantum KEM algorithms
///   provided when the `pqc-lite` feature is enabled.
#[cfg(feature = "protocol")]
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
#[cfg(feature = "protocol")]
pub struct KeyExchange {
    algorithm: KeyExchangeAlgorithm,
    secret: Option<Vec<u8>>,
}

/// Derive a cryptographic key based on crypto settings (blake2b-512 or blake3)
///
/// # Parameters
/// - `blake3`: whether to use Blake3 or Blake2b-512
/// - `ikm`: Input key material (cryptographic key)
/// - `salt`: 16-byte salt
/// - `context`: 16-byte context
/// - `extra_key`: optional 64-byte extra key to put into the hash
///
/// Panics if blake3 is not supported or hash function failed
pub fn derive_key(
    blake3: bool,
    ikm: &[u8],
    salt: &[u8; 16],
    context: &[u8; 16],
    extra_key: Option<&[u8; 64]>,
) -> [u8; 64] {
    #[cfg(not(feature = "blake-3"))]
    if blake3 {
        panic!("Blake3 is not supported when the 'blake-3' feature is not enabled");
    }

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
        return out;
    }

    // This is called Mac, but is actually useful for key derivation because of salt/personalization
    let mut kdf = Blake2bMac512::new_with_salt_and_personal(ikm, salt, context)
        .expect("Failed to create KDF hasher");

    if let Some(extra_key) = extra_key {
        kdf.update(extra_key);
    }

    kdf.finalize().into_bytes().into()
}

impl_hash!(hash_128, 16, Blake2b128, |h: blake3::Hasher| {
    let mut out = [0u8; 16];
    h.finalize_xof().fill(&mut out);
    out
});

impl_hash!(hash_192, 24, Blake2b192, |h: blake3::Hasher| {
    let mut out = [0u8; 24];
    h.finalize_xof().fill(&mut out);
    out
});

impl_hash!(hash_256, 32, Blake2b256, |h: blake3::Hasher| {
    *h.finalize().as_bytes()
});

impl_hash!(hash_512, 64, Blake2b512, |h: blake3::Hasher| {
    let mut out = [0u8; 64];
    h.finalize_xof().fill(&mut out);
    out
});

/// Calculate a MAC (Message Authentication Code) using keyed Blake2b-128 or Blake3
///
/// # Parameters
///
/// - `blake3`: whether to use blake3 or blake2b-128
/// - `key`: 64-byte key, altough blake3 only uses the first 32 bytes as key and updates the hash with the other 32 bytes
/// - `data`: The data to calculate a MAC for
/// - `extra_data`: Extra data to update the hasher with
///
/// Returns a MAC
pub fn calculate_mac(
    blake3: bool,
    key: &[u8; 64],
    data: &[u8],
    extra_data: Option<&[u8]>,
) -> [u8; 16] {
    #[cfg(not(feature = "blake-3"))]
    if blake3 {
        panic!("Blake3 is not supported when the 'blake-3' feature is not enabled");
    }

    #[cfg(feature = "blake-3")]
    if blake3 {
        use blake3::Hasher;
        let mut hasher = Hasher::new_keyed(key[..32].try_into().unwrap());
        hasher.update(&key[32..]); // add the rest of the key material
        hasher.update(data);
        if let Some(extra_data) = extra_data {
            hasher.update(extra_data);
        }

        let mut mac = [0u8; 16];
        hasher.finalize_xof().fill(&mut mac);
        return mac;
    }

    let mut hasher = Blake2bMac128::new(key.into());
    if let Some(extra_data) = extra_data {
        hasher.update(extra_data);
    }

    hasher.finalize().into_bytes().into()
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

        let res = derive_key(false, &ikm, &salt, &personal, None);

        // This hash is exactly the same as Geralt (for .NET) produces, which is a wrapper around libsodium
        // https://www.geralt.xyz/key-derivation
        // so this test ensures that "Blake2bMac512" is the same as the KDF mode of libsodium

        assert_eq!(
            BASE64_STANDARD.encode(&res),
            "PiPfpmZbmso8hQM8U/pqeVzJ0C9THDubc5aultGQ4W5brnHKOWBf008vmBxodvL62BLIU5LSvXn+icjRou7MBw=="
        )
    }
}
