use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::packets::base::crypto_keys::{CryptoSignature, KeyExhangeRequest, KeyExhangeResponse};

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

    use crate::packets::request::PlabbleRequestPacket;


    #[test]
    fn can_serialize_session_request() {
        let packet: PlabbleRequestPacket = toml::from_str(r#"
        version = 1
        mac = "AAAAAAAAAAAAAAAAAAAAAA"

        [header]
        packet_type = "Session"

        [[body.keys]]
        X25519 = "jW7RHvEpPO0nZG4pCYI0gGZ1MPYQGQu4vLqpsakCtMc"
        "#).unwrap();

        let bytes = packet.to_bytes().unwrap();
        // type 0001, flags 0000. Packet type 0000, packet flags 0000. 16-byte zero-Mac. 32-byte x25519 key.
        assert_eq!(vec![0b0000_0001, 0b0000_0000, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 141, 110, 209, 30, 241, 41, 60, 237, 39, 100, 110, 41, 9, 130, 52, 128, 102, 117, 48, 246, 16, 25, 11, 184, 188, 186, 169, 177, 169, 2, 180, 199], bytes);
        
        let deserialized = PlabbleRequestPacket::from_bytes(&bytes).unwrap();
        assert_eq!(packet, deserialized);
    }
}