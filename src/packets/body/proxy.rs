use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::crypto::algorithm::{CryptoSignature, KeyExhangeRequest, KeyExhangeResponse};

/// Proxy request body
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[no_discriminator]
pub enum ProxyRequestBody {
    /// Send a packet through an existing tunnel
    Tunnel {
        /// Tunnel identifier provided by the server during initialization
        #[dyn_int]
        tunnel_id: u32,

        /// Encrypted raw data to send through the tunnel
        #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
        packet: Vec<u8>,
    },
    /// Initialize a new proxy session
    Initialize {
        /// Target server / service to connect to
        #[dyn_length]
        target: String,

        /// How many hops to use for the tunnel
        #[length_for("hop_count")]
        hop_count: u8,

        /// If no random hops are requested, specify which hops to use in the route
        #[val_dyn_length]
        #[toggled_by = "!random_hops"]
        #[length_by = "hop_count"]
        via: Option<Vec<String>>,

        /// Public keys or encapsulation keys for creating shared secrets with each hop
        #[multi_enum]
        keys: Vec<KeyExhangeRequest>
    },
}

/// Proxy response body
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[no_discriminator]
pub enum ProxyResponseBody {
    /// Response to a packet sent through an existing tunnel
    Tunnel {
        /// Tunnel identifier for the tunnel the packet was sent through
        #[dyn_int]
        tunnel_id: u32,

        /// Encrypted raw response data
        #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
        packet: Vec<u8>,
    },
    /// Response to a proxy initialization request.
    Initialize {
        /// Tunnel identifier for the newly created tunnel
        #[dyn_int]
        tunnel_id: u32,

        /// Information about the selected hops in the route
        #[key_dyn_length]
        hops: HashMap<String, HopInfo>
    },
}

/// Hop information
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct HopInfo {
    /// Public keys or encapsulated secret for creating a shared secret
    #[multi_enum]
    keys: Vec<KeyExhangeResponse>,

    /// Signatures of the request public key
    #[multi_enum]
    signatures: Vec<CryptoSignature>,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::{request::PlabbleRequestPacket, response::PlabbleResponsePacket};

    #[test]
    fn can_serialize_and_deserialize_proxy_init_request_with_hops() {
        let key = "82c26445e178bba57f5d44abb236688ff8ff5a8ac77a5514b7eb3849ed3ed0c9";
        
        let request: PlabbleRequestPacket = toml::from_str(r#"
            version = 1

            [header]
            packet_type = "Proxy"
            init_session = true
            select_random_hops = false

            [body.Initialize]
            target = "test"
            hop_count = 2
            via = ["hop1", "hop2"]

            [[body.Initialize.keys]]
            X25519 = "gsJkReF4u6V_XUSrsjZoj_j_WorHelUUt-s4Se0-0Mk"
        "#).unwrap();
        let bytes = request.to_bytes(None).unwrap();
        assert_eq!(bytes[1], 0b0001_1100);

        assert_eq!(format!("011c04746573740204686f703104686f7032{}", key), hex::encode(&bytes));
        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_proxy_init_request_with_random_hops() {
        let key = "82c26445e178bba57f5d44abb236688ff8ff5a8ac77a5514b7eb3849ed3ed0c9";
        
        let request: PlabbleRequestPacket = toml::from_str(r#"
            version = 1

            [header]
            packet_type = "Proxy"
            init_session = true
            select_random_hops = true

            [body.Initialize]
            target = "test"
            hop_count = 2

            [[body.Initialize.keys]]
            X25519 = "gsJkReF4u6V_XUSrsjZoj_j_WorHelUUt-s4Se0-0Mk"
        "#).unwrap();
        let bytes = request.to_bytes(None).unwrap();
        assert_eq!(bytes[1], 0b0101_1100);

        assert_eq!(format!("015c047465737402{}", key), hex::encode(&bytes));
        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_proxy_tunnel_request() {
        let request: PlabbleRequestPacket = toml::from_str(r#"
            version = 1

            [header]
            packet_type = "Proxy"
            keep_connection = true

            [body.Tunnel]
            tunnel_id = 7
            packet = "9V0FzpQi"
        "#).unwrap();
        let bytes = request.to_bytes(None).unwrap();
        assert_eq!(bytes[1], 0b0010_1100);

        assert_eq!(format!("012c07f55d05ce9422"), hex::encode(&bytes));
        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_proxy_tunnel_response() {
        let request: PlabbleResponsePacket = toml::from_str(r#"
            version = 1

            [header]
            packet_type = "Proxy"
            request_counter = 5

            [body.Tunnel]
            tunnel_id = 7
            packet = "9V0FzpQi"
        "#).unwrap();
        let bytes = request.to_bytes(None).unwrap();
        assert_eq!(bytes[1], 0b0000_1100);

        assert_eq!(format!("010c000507f55d05ce9422"), hex::encode(&bytes));
        let deserialized = PlabbleResponsePacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_proxy_init_response() {
        let key1 = "10c0dd08b7ee50f59b24fa3e5743da54abba5fbfa19074fc2cd2a4352b3e2f8c";
        let sig1 = "3aff61b1ff45b9ad2c47b22f3bf0f996266ad65dac4689afea11329b1a599ff44555f885dda5e32f028203d44aad8f8a3dbbefbd14c460a50430350843688ea3";
        let key2 = "7ed37c55a8153540676b350758cc33d80edf00d99eea5ac79b183cda3430f8bf";
        let sig2 = "e6f111d94310aa76a649ef214edfbdd6c9d13b7e0a139d5fe3e8a3060706891e8f9af143c0417d9db8148fd26a92ba4631b3ecabff87f9750bb06bb4e975289f";

        let response: PlabbleResponsePacket = toml::from_str(r#"
            version = 1

            [header]
            packet_type = "Proxy"
            init_session = true
            request_counter = 5

            [body.Initialize]
            tunnel_id = 7

            [[body.Initialize.hops.hop1.keys]]
            X25519 = "EMDdCLfuUPWbJPo-V0PaVKu6X7-hkHT8LNKkNSs-L4w"

            [[body.Initialize.hops.hop1.signatures]]
            Ed25519 = "Ov9hsf9Fua0sR7IvO_D5liZq1l2sRomv6hEymxpZn_RFVfiF3aXjLwKCA9RKrY-KPbvvvRTEYKUEMDUIQ2iOow"

            [[body.Initialize.hops.hop2.keys]]
            X25519 = "ftN8VagVNUBnazUHWMwz2A7fANme6lrHmxg82jQw-L8"

            [[body.Initialize.hops.hop2.signatures]]
            Ed25519 = "5vER2UMQqnamSe8hTt-91snRO34KE51f4-ijBgcGiR6PmvFDwEF9nbgUj9JqkrpGMbPsq_-H-XULsGu06XUonw"
        "#).unwrap();
        let bytes = response.to_bytes(None).unwrap();
        assert_eq!(bytes[1], 0b0001_1100);

        let hop1 = hex::encode(b"hop1");
        let hop2 = hex::encode(b"hop2");

        let possibilities = vec![
            format!("011c00050704{}{}{}04{}{}{}", hop1, key1, sig1, hop2, key2, sig2),
            format!("011c00050704{}{}{}04{}{}{}", hop2, key2, sig2, hop1, key1, sig1)
        ];

        assert!(possibilities.contains(&hex::encode(&bytes)));

        let deserialized = PlabbleResponsePacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(response, deserialized);
    }
}