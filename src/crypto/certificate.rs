use crate::default_true;
use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::{
    core::PlabbleDateTime,
    crypto::algorithm::{CryptoSignature, VerificationKey},
};

/// Plabble Certificate
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, PartialEq, Serialize, Deserialize)]
pub struct Certificate {
    /// If set to true, this is a full certificate
    #[toggles("full_cert")]
    #[serde(default = "default_true")]
    full_cert: bool,

    /// If set to true, this is a root certificate
    #[toggles("root_cert")]
    #[serde(default)]
    root_cert: bool,

    /// The unique certificate ID
    /// This is a hash of the following certificate data:
    /// - Blake2b_128(valid_from, valid_to, issuer_uri, data)
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    id: [u8; 16],

    /// URI where this certificate can be found
    #[dyn_length]
    uri: String,

    /// Certificate content, if full certificate
    #[toggled_by = "full_cert"]
    #[serde(flatten)]
    body: Option<CertificateBody>,
}

/// Plabble Certificate body
#[derive(Debug, FromBytes, ToBytes, PartialEq, Serialize, Deserialize)]
pub struct CertificateBody {
    /// From when the certificate was valid
    valid_from: PlabbleDateTime,

    /// Until when the certificate is valid
    valid_until: PlabbleDateTime,

    /// Who issued the certificate (if not a root cert or self-signed)
    #[toggled_by = "!root_cert"]
    #[dyn_length]
    issuer_uri: Option<String>,

    /// The certificate data, for instance CA=plabble;CN=Root certificate
    #[dyn_length]
    data: String,

    /// The public keys this certificate contains/are issued with this certificate
    #[multi_enum]
    keys: Vec<VerificationKey>,

    /// Signatures for each algorithm in this certificate, by the issuer
    /// Every signature contains the following data:
    /// - The public key for the specific algorithm (the issued key)
    /// - Blake2b_128(valid_from, valid_to, issuer_uri, data)
    #[multi_enum]
    signatures: Vec<CryptoSignature>,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    use crate::crypto::certificate::Certificate;

    #[test]
    fn can_serialize_and_deserialize_non_full_certificate() {
        let cert: Certificate = toml::from_str(
            r#"
            full_cert = false
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            uri = "https://certs.plabble.org/{id}.crt"
        "#,
        )
        .unwrap();

        assert!(cert.body.is_none());

        let mut config = SerializerConfig::<()>::new(None);
        let bytes = cert.to_bytes(Some(&mut config)).unwrap();

        // 0000_0000, 16x 0, [len = 34], URL
        assert_eq!(
            bytes,
            vec![
                0x0000_0000,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                34,
                b'h',
                b't',
                b't',
                b'p',
                b's',
                b':',
                b'/',
                b'/',
                b'c',
                b'e',
                b'r',
                b't',
                b's',
                b'.',
                b'p',
                b'l',
                b'a',
                b'b',
                b'b',
                b'l',
                b'e',
                b'.',
                b'o',
                b'r',
                b'g',
                b'/',
                b'{',
                b'i',
                b'd',
                b'}',
                b'.',
                b'c',
                b'r',
                b't',
            ]
        );

        let deserialized = Certificate::from_bytes(&bytes, Some(&mut config)).unwrap();
        assert_eq!(cert, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_certificate() {
        // This is NOT a valid certificate - the signature does not match the content - just for testing the serialization
        let cert: Certificate = toml::from_str(r#"
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            uri = "https://certs.plabble.org/{id}.crt"

            valid_from = "2025-05-15T12:30:00+00:00"
            valid_until = "2161-02-07T06:28:15+00:00"
            issuer_uri = "https://certs.plabble.org/root.crt"
            data = "CA=P;CN=tst"

            [[keys]]
            Ed25519 = "VHaAg2DtpB267PC0X9mF8gmrLlh0nQtPPbGS_z0N1VE"

            [[signatures]]
            Ed25519 = "3eOHFAPx5lMev8MJ-gXEPdlRMLBM3IUTnOxRIyvjtcvYhOFv7SUv0byqc5EKy6XWqAbNNYGHoMhWh5vRwlEARA"
        "#).unwrap();

        assert!(cert.body.is_some());

        let mut config = SerializerConfig::<()>::new(None);
        config.set_toggle("ed25519", true);

        let bytes = cert.to_bytes(Some(&mut config)).unwrap();

        let mut config = SerializerConfig::<()>::new(None);
        config.set_toggle("ed25519", true);
        let deserialized = Certificate::from_bytes(&bytes, Some(&mut config)).unwrap();
        assert_eq!(cert, deserialized);

        // 0000_0001, 16x 0, [len = 34], URL, 0, 177, 88, 200 (First timestamp), 0xffffffff (second timestamp),
        // [len=34], URL, [len = 11] data, 32 byte ed25519 key, 64 byte ed25519 key
        assert_eq!(
            bytes,
            vec![
                0x0000_0001,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                34,
                b'h',
                b't',
                b't',
                b'p',
                b's',
                b':',
                b'/',
                b'/',
                b'c',
                b'e',
                b'r',
                b't',
                b's',
                b'.',
                b'p',
                b'l',
                b'a',
                b'b',
                b'b',
                b'l',
                b'e',
                b'.',
                b'o',
                b'r',
                b'g',
                b'/',
                b'{',
                b'i',
                b'd',
                b'}',
                b'.',
                b'c',
                b'r',
                b't',
                0,
                177,
                88,
                200,
                255,
                255,
                255,
                255,
                34,
                b'h',
                b't',
                b't',
                b'p',
                b's',
                b':',
                b'/',
                b'/',
                b'c',
                b'e',
                b'r',
                b't',
                b's',
                b'.',
                b'p',
                b'l',
                b'a',
                b'b',
                b'b',
                b'l',
                b'e',
                b'.',
                b'o',
                b'r',
                b'g',
                b'/',
                b'r',
                b'o',
                b'o',
                b't',
                b'.',
                b'c',
                b'r',
                b't',
                11,
                b'C',
                b'A',
                b'=',
                b'P',
                b';',
                b'C',
                b'N',
                b'=',
                b't',
                b's',
                b't',
                84,
                118,
                128,
                131,
                96,
                237,
                164,
                29,
                186,
                236,
                240,
                180,
                95,
                217,
                133,
                242,
                9,
                171,
                46,
                88,
                116,
                157,
                11,
                79,
                61,
                177,
                146,
                255,
                61,
                13,
                213,
                81,
                221,
                227,
                135,
                20,
                3,
                241,
                230,
                83,
                30,
                191,
                195,
                9,
                250,
                5,
                196,
                61,
                217,
                81,
                48,
                176,
                76,
                220,
                133,
                19,
                156,
                236,
                81,
                35,
                43,
                227,
                181,
                203,
                216,
                132,
                225,
                111,
                237,
                37,
                47,
                209,
                188,
                170,
                115,
                145,
                10,
                203,
                165,
                214,
                168,
                6,
                205,
                53,
                129,
                135,
                160,
                200,
                86,
                135,
                155,
                209,
                194,
                81,
                0,
                68
            ]
        );
    }
}
