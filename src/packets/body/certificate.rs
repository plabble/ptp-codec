use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::crypto::algorithm::CryptoSignature;
use crate::crypto::certificate::Certificate;

/// Certificate request body
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct CertificateRequestBody {
    /// Id of the certificate to query
    #[toggled_by = "query_mode"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    id: Option<[u8; 16]>,

    /// Client-side generated random challenge the server MUST sign when provided, to prove its identity
    #[toggled_by = "challenge"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    challenge: Option<[u8; 16]>,
}

/// Certificate response body
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct CertificateResponseBody {
    /// Signatures of the server to prove its identity and authenticity of the message
    /// For each algorithm in the crypto settings header, generate a signature of the challenge (optionally) + all full certificates (in order)
    #[multi_enum]
    signatures: Vec<CryptoSignature>,

    /// Certificate chain (list in order, first certificate = bottom of chain, last certificate = top of chain)
    certificates: Vec<Certificate>,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::{request::PlabbleRequestPacket, response::PlabbleResponsePacket};

    #[test]
    fn can_serialize_and_deserialize_certificate_request_with_query_and_challenge() {
        let req: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 0
            use_encryption = true

            [header]
            packet_type = "Certificate"
            full_certs = true
            challenge = true
            query_mode = true

            [body]
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            challenge = "AQEBAQEBAQEBAQEBAQEBAQ"
        "#,
        )
        .unwrap();

        let serialized = req.to_bytes(None).unwrap();

        // Version 0000, flags 0100. packet type = 0000, packet flags = 1110, 16x 0, 16x 1
        assert_eq!(
            serialized,
            vec![
                0b0100_0000,
                0b1110_0000,
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
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1
            ]
        );

        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();
        assert_eq!(deserialized, req);
    }

    #[test]
    fn can_serialize_and_deserialize_certificate_request_without_query() {
        let req: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 0
            use_encryption = true

            [header]
            packet_type = "Certificate"
            full_certs = true
            challenge = true

            [body]
            challenge = "AQEBAQEBAQEBAQEBAQEBAQ"
        "#,
        )
        .unwrap();

        let serialized = req.to_bytes(None).unwrap();

        // Version 0000, flags 0100. packet type = 0000, packet flags = 0110, 16x 0, 16x 1
        assert_eq!(
            serialized,
            vec![
                0b0100_0000,
                0b0110_0000,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1
            ]
        );

        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();
        assert_eq!(deserialized, req);
    }

    #[test]
    fn can_serialize_and_deserialize_certificate_response_with_ed25519_signatures() {
        let res: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 0
            use_encryption = true

            [header]
            packet_type = "Certificate"
            request_counter = 1

            [[body.signatures]]
            Ed25519 = "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQ"

            [[body.certificates]]
            id = "AgICAgICAgICAgICAgICAg"
            uri = "..."
            valid_from = "2025-05-15T12:00:00+00:00"
            valid_until = "2026-01-01T00:00:00+00:00"
            issuer_uri = "..."
            data = "CA=plabble"

                [[body.certificates.keys]]
                Ed25519 = "AwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwMDAwM"

                [[body.certificates.signatures]]
                Ed25519 = "BAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBA"

            [[body.certificates]]
            root_cert = true
            id = "BQUFBQUFBQUFBQUFBQUFBQ"
            uri = "."
            valid_from = "2025-05-15T12:00:00+00:00"
            valid_until = "2030-01-01T00:00:00+00:00"
            data = "CA=plabble"

                [[body.certificates.keys]]
                Ed25519 = "BgYGBgYGBgYGBgYGBgYGBgYGBgYGBgYGBgYGBgYGBgY"

                [[body.certificates.signatures]]
                Ed25519 = "BwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBwcHBw"
        "#,
        )
        .unwrap();

        let bytes = res.to_bytes(None).unwrap();
        assert_eq!(
            bytes,
            vec![
                0b0100_0000, // version 0000, flags 0100
                0b0000_0000, // packet type 0000, packet flags 0000
                0,
                1, // request counter 1
                // 64x 1 (signatures)
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                1,
                // first certificate flags: 0b00000001
                0b00000001,
                // certificate, id: 16x 2
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                2,
                3, // uri len: 3
                b'.',
                b'.',
                b'.',
                // 2 plabble timestamps
                0,
                177,
                81,
                192,
                1,
                225,
                51,
                128,
                3, // uri len: 3
                b'.',
                b'.',
                b'.',
                10, // data len: 10,
                b'C',
                b'A',
                b'=',
                b'p',
                b'l',
                b'a',
                b'b',
                b'b',
                b'l',
                b'e',
                // 32x 3 (key)
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                3,
                // 64x 4 (signature)
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                4,
                // second certificate flags: 0b00000011
                0b00000011,
                // certificate, id: 16x 5
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                5,
                1, // uri len: 1
                b'.',
                // 2 plabble timestamps
                0,
                177,
                81,
                192,
                9,
                103,
                83,
                0,
                10, // data len: 10,
                b'C',
                b'A',
                b'=',
                b'p',
                b'l',
                b'a',
                b'b',
                b'b',
                b'l',
                b'e',
                // 32x 6 (key)
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                6,
                // 64x 7 (signature)
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
                7,
            ]
        )
    }
}
