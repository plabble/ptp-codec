use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::core::PlabbleDateTime;
use crate::crypto::algorithm::CryptoSignature;
use crate::crypto::certificate::Certificate;

/// Prove identity to server (based on certificate obtained from earlier REGISTER)
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct IdentifyRequestBody {
    /// The timestamp, the server will check if it is within an acceptible range (e.g. a few minutes) to prevent replay attacks
    timestamp: PlabbleDateTime,

    /// Signatures of the timestamp, the server ID (certificate ID of the server certificate) and the Session Key.
    /// Algorithms based on crypto_settings in base packet
    #[multi_enum]
    signatures: Vec<CryptoSignature>,

    /// Certificate chain (list in order, first certificate = bottom of chain, last certificate = top of chain)
    /// This is mostly of the time just a single (partial/non-full) certificate that only contains the certificate header data
    /// Its goal is to make the server able to verify the signatures and get the identity of the user (certificate ID)
    certificates: Vec<Certificate>,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};
    use chrono::{TimeZone, Utc};

    use crate::{core::PlabbleDateTime, packets::request::PlabbleRequestPacket};

    #[test]
    fn can_serialize_and_deserialize_identify_request() {
        let sig = "e3ac1f8cd2446b1af84bd264d267cb0d1d3456dff442ffe341040302e3ca273cccb3da10df24aa8cd4f5b36c8236070c9eef9e68152585a209dd9afc3ab617d8";
        let uid = "9d1261e48580418400d1ccd9a889ed37";
        let uri = hex::encode(b"plabble:plabble.org/certs/@maus");
        let timestamp = hex::encode(PlabbleDateTime(Utc.with_ymd_and_hms(2026, 1, 1, 0, 0, 0).unwrap()).timestamp().to_be_bytes()); 

        let request: PlabbleRequestPacket = toml::from_str(r#"
            version = 1
            specify_crypto_settings = true

            [crypto_settings]
            sign_ed25519 = true

            [header]
            packet_type = "Identify"

            [body]
            timestamp = "2026-01-01T00:00:00+00:00"

            [[body.signatures]]
            Ed25519 = "46wfjNJEaxr4S9Jk0mfLDR00Vt_0Qv_jQQQDAuPKJzzMs9oQ3ySqjNT1s2yCNgcMnu-eaBUlhaIJ3Zr8OrYX2A"

            [[body.certificates]]
            full_cert = false
            id = "nRJh5IWAQYQA0czZqIntNw"
            uri = "plabble:plabble.org/certs/@maus"
        "#).unwrap();

        let bytes = request.to_bytes(None).unwrap();
        assert_eq!(format!("81310b{}{}00{}1f{}", timestamp, sig, uid, uri), hex::encode(&bytes));

        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }
}
