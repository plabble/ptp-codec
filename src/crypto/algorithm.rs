use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

// Visit https://openquantumsafe.org/liboqs/algorithms

/// Cryptographic keys used for key exchange request
///
/// # Variants
/// - X25519: 32 bytes public key for X25519 key exchange
/// - Kem512: 800 bytes public key for ML-KEM-512 post-quantum key exchange
/// - Kem768: 1184 bytes public key for ML-KEM-768 post-quantum key exchange
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
/// - Kem512: 768 bytes encapsulated secret for ML-KEM-512 post-quantum key exchange
/// - Kem768: 1088 bytes encapsulated secret for ML-KEM-768 post-quantum key exchange
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
/// - Dsa44: 2420 bytes signature for ML-DSA-44 post-quantum signatures
/// - Dsa65: 3309 bytes signature for ML-DSA-65 post-quantum signatures
/// - Falcon: 1462 bytes signature for Falcon-1024 post-quantum signatures
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

/// Public verification keys used in various algorithms for verifying a digital signature
/// The signatures are stored as fixed-size byte arrays, serialized/deserialized using base64 encoding (when using serde)
///
/// # Variants:
/// - Ed25519: 32 bytes key for Ed25519
/// - Dsa44: 1312 bytes key for ML-DSA-44
/// - Dsa65: 1952 bytes key for ML-DSA-65
/// - Falcon: 1793 bytes key for Falcon-1024 which is bigger than the signature :D
/// - SlhDsaSha128s: 32 bytes key for SLH-DSA-SHA128s
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
#[no_discriminator]
pub enum VerificationKey {
    #[toggled_by = "ed25519"]
    Ed25519(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 32]),

    #[toggled_by = "dsa44"]
    Dsa44(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1312]),

    #[toggled_by = "dsa65"]
    Dsa65(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1952]),

    #[toggled_by = "falcon"]
    Falcon(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 1793]),

    #[toggled_by = "slh_dsa"]
    SlhDsaSha128s(#[serde_as(as = "Base64<UrlSafe, Unpadded>")] [u8; 32]),
}

/// Secret siging keys used in various algorithms for creating a digital signature
/// The signatures are stored as fixed-size byte arrays, serialized/deserialized using base64 encoding (when using serde)
///
/// # Variants:
/// - Ed25519: 32 bytes key for Ed25519
/// - Dsa44: 2560 bytes key for ML-DSA-44
/// - Dsa65: 4032 bytes key for ML-DSA-65
/// - Falcon: 2305 bytes key for Falcon-1024
/// - SlhDsaSha128s: 64 bytes key for SLH-DSA-SHA128s
#[derive(Debug, PartialEq)]
pub enum SigningKey {
    Ed25519([u8; 32]),
    Dsa44([u8; 2560]),
    Dsa65([u8; 4032]),
    Falcon([u8; 2305]),
    SlhDsaSha128s([u8; 64]),
}
