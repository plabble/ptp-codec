use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::packets::base::algorithm::{CryptoSignature, KeyExhangeRequest, KeyExhangeResponse};

/// Session request body
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct SessionRequestBody {
    /// PSK expiration Plabble timestamp. Filled if request flag persist_key is set.
    #[toggled_by = "persist_key"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    pub psk_expiration: Option<[u8; 4]>,

    /// Client-generated salt for key derivation. Filled if request flag with_salt is set.
    #[toggled_by = "client_salt"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    pub salt: Option<[u8; 16]>,

    /// Public/encapsulation keys for creating a shared secret with the server
    #[multi_enum]
    pub keys: Vec<KeyExhangeRequest>,
}

#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct SessionResponseBody {
    /// Pre-shared key identifier. Filled if request flag with_psk is set.
    #[toggled_by = "key_persisted"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    psk_id: Option<[u8; 12]>,

    /// Server-generated salt for key derivation. Filled if request flag with_salt is set.
    #[toggled_by = "server_salt"]
    salt: Option<[u8; 16]>,

    /// Public keys or encapsulated secret for creating a shared secret
    #[multi_enum]
    keys: Vec<KeyExhangeResponse>,

    /// Signatures of the request
    #[multi_enum]
    signatures: Vec<CryptoSignature>,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::{request::PlabbleRequestPacket, response::PlabbleResponsePacket};

    #[test]
    fn can_serialize_and_deserialize_simple_session_request() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
        version = 1
        mac = "AAAAAAAAAAAAAAAAAAAAAA"

        [header]
        packet_type = "Session"

        [[body.keys]]
        X25519 = "jW7RHvEpPO0nZG4pCYI0gGZ1MPYQGQu4vLqpsakCtMc"
        "#,
        )
        .unwrap();

        let bytes = packet.to_bytes(None).unwrap();

        // type 0001, flags 0000. 16-byte zero-Mac. Packet type 0001, packet flags 0000. 32-byte x25519 key.
        assert_eq!(
            vec![
                0b0000_0001,
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
                0b0000_0001,
                141,
                110,
                209,
                30,
                241,
                41,
                60,
                237,
                39,
                100,
                110,
                41,
                9,
                130,
                52,
                128,
                102,
                117,
                48,
                246,
                16,
                25,
                11,
                184,
                188,
                186,
                169,
                177,
                169,
                2,
                180,
                199
            ],
            bytes
        );

        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_session_request_with_salt_and_psk() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
        version = 1
        use_encryption = true

        [header]
        packet_type = "Session"
        with_salt = true
        persist_key = true

        [body]
        psk_expiration = "Kk7HXQ"
        salt = "R6Bt7xEAskTkgzk_YmGxpw"

        [[body.keys]]
        X25519 = "jW7RHvEpPO0nZG4pCYI0gGZ1MPYQGQu4vLqpsakCtMc"
        "#,
        )
        .unwrap();

        println!("{:?}", packet);

        let bytes = packet.to_bytes(None).unwrap();

        // Type 0001, flags 0100. Packet type 0001, packet flags 0101. PSK expiration 42, 78, 199, 93. salt 16 bytes, 32-byte x25519 key
        assert_eq!(
            vec![
                0b0100_0001,
                0b0101_0001,
                42,
                78,
                199,
                93,
                71,
                160,
                109,
                239,
                17,
                0,
                178,
                68,
                228,
                131,
                57,
                63,
                98,
                97,
                177,
                167,
                141,
                110,
                209,
                30,
                241,
                41,
                60,
                237,
                39,
                100,
                110,
                41,
                9,
                130,
                52,
                128,
                102,
                117,
                48,
                246,
                16,
                25,
                11,
                184,
                188,
                186,
                169,
                177,
                169,
                2,
                180,
                199
            ],
            bytes
        );

        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_session_response() {
        let packet: PlabbleResponsePacket = toml::from_str(
            r#"
        version = 1
        use_encryption = true

        [header]
        packet_type = "Session"
        request_counter = 2
        with_psk = true

        [body]
        psk_id = "AAAAAAAAAAAAAAAA"

        [[body.keys]]
        X25519 = "jW7RHvEpPO0nZG4pCYI0gGZ1MPYQGQu4vLqpsakCtMc"

        [[body.signatures]]
        Ed25519 = "QijFnI8mL7GXaqIMRkvhJiAEPeUpNggJutV6Jd9ZQtUiXS5JN8lqcs1WYMJT0Oeb11m4nYNDHC_l0VZF10Jzyg"
        "#,
        )
        .unwrap();

        let bytes = packet.to_bytes(None).unwrap();
        // Version 0001, flags 0100. Packet type 0001, packet flags 0001. Request counter 2. PSK ID 12 bytes. 32-byte x25519 key. 64-byte ed25519 signature.
        assert_eq!(
            vec![
                0b0100_0001,
                0b0001_0001,
                0,
                2,
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
                141,
                110,
                209,
                30,
                241,
                41,
                60,
                237,
                39,
                100,
                110,
                41,
                9,
                130,
                52,
                128,
                102,
                117,
                48,
                246,
                16,
                25,
                11,
                184,
                188,
                186,
                169,
                177,
                169,
                2,
                180,
                199,
                66,
                40,
                197,
                156,
                143,
                38,
                47,
                177,
                151,
                106,
                162,
                12,
                70,
                75,
                225,
                38,
                32,
                4,
                61,
                229,
                41,
                54,
                8,
                9,
                186,
                213,
                122,
                37,
                223,
                89,
                66,
                213,
                34,
                93,
                46,
                73,
                55,
                201,
                106,
                114,
                205,
                86,
                96,
                194,
                83,
                208,
                231,
                155,
                215,
                89,
                184,
                157,
                131,
                67,
                28,
                47,
                229,
                209,
                86,
                69,
                215,
                66,
                115,
                202
            ],
            bytes
        );

        let deserialized = PlabbleResponsePacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(packet, deserialized);
    }
}
