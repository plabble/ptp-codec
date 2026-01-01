use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

/// Cryptographic keys used for key exchange request
///
/// # Variants
/// - X25519: 32 bytes public key for X25519 key exchange
/// - Kem512: 800 bytes public key for KEM-512 post-quantum key exchange
/// - Kem768: 1184 bytes public key for KEM-768 post-quantum key exchange
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
#[no_discriminator]
pub enum KeyExhangeRequest {
    #[toggled_by = "x25519"]
    X25519(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 32]),

    #[toggled_by = "kem512"]
    Kem512(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 800]),

    #[toggled_by = "kem758"]
    Kem768(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1184]),
}

/// Cryptographic keys used for key exchange request
///
/// # Variants
/// - X25519: 32 bytes public key for X25519 key exchange
/// - Kem512: 768 bytes encapsulated secret for KEM-512 post-quantum key exchange
/// - Kem768: 1088 bytes encapsulated secret for KEM-768 post-quantum key exchange
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
#[no_discriminator]
pub enum KeyExhangeResponse {
    #[toggled_by = "x25519"]
    X25519(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 32]),

    #[toggled_by = "kem512"]
    Kem512(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 768]),

    #[toggled_by = "kem758"]
    Kem768(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1088]),
}

/// Cryptographic signatures used in various algorithms
/// The signatures are stored as fixed-size byte arrays, serialized/deserialized using base64 encoding (when using serde)
///
/// # Variants:
/// - Ed25519: 64 bytes signature for Ed25519 signatures
/// - Dsa44: 2420 bytes signature for DSA-44 post-quantum signatures
/// - Dsa65: 3309 bytes signature for DSA-65 post-quantum signatures
/// - Falcon: 1462 bytes signature for Falcon post-quantum signatures
/// - SlhDsaSha128s: 7856 bytes signature for SLH-DSA-SHA128s post-quantum signatures
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
#[no_discriminator]
pub enum CryptoSignature {
    #[toggled_by = "ed25519"]
    Ed25519(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 64]),

    #[toggled_by = "dsa44"]
    Dsa44(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 2420]),

    #[toggled_by = "dsa65"]
    Dsa65(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 3309]),

    #[toggled_by = "falcon"]
    Falcon(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1462]),

    #[toggled_by = "slh_dsa"]
    SlhDsaSha128s(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 7856]),
}
