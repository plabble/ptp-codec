use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::core::{PlabbleDateTime, node_address::NodeAddress};
use crate::crypto::algorithm::VerificationKey;
use crate::packets::base::settings::CryptoSettings;

#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NodeInfo {
    /// Node identifier (same as certificate ID)
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub id: [u8; 16],

    /// Node address (IP and port)
    pub address: NodeAddress,

    /// Last seen timestamp
    pub last_seen: PlabbleDateTime,

    /// Crypto settings used by the node (e.g., supported algorithms)
    #[serde(default)]
    pub crypto_settings: CryptoSettings,

    /// Public keys
    #[multi_enum]
    pub verification_keys: Vec<VerificationKey>,
}

#[cfg(test)]
mod tests {
    use std::net::{Ipv4Addr, SocketAddrV4};

    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    use crate::{
        core::{PlabbleDateTime, node_address::NodeAddress},
        crypto::algorithm::VerificationKey,
        network::node_info::NodeInfo,
    };

    #[test]
    fn can_serialize_and_deserialize_nodeinfo_with_ipv4() {
        let node_info = NodeInfo {
            id: [0u8; 16],
            address: NodeAddress::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 1234)),
            last_seen: PlabbleDateTime::new(0),
            crypto_settings: Default::default(),
            verification_keys: vec![VerificationKey::Ed25519([1u8; 32])],
        };

        let serialized = r#"
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            last_seen = "2025-01-01T00:00:00Z"

            [address]
            V4 = "127.0.0.1:1234"

            [[verification_keys]]
            Ed25519 = "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE"
        "#;

        let deserialized: NodeInfo = toml::from_str(serialized).unwrap();
        assert_eq!(node_info, deserialized);

        let serialized = deserialized
            .to_bytes(None::<&mut SerializerConfig>)
            .unwrap();

        assert_eq!(
            format!(
                // 00 ipv4
                // 7f000001 127.0.0.1
                // 04d2 port 1234
                // 00000000 timestamp
                // 31 default crypto settings
                "{}007f00000104d20000000031{}",
                "00".repeat(16),
                "01".repeat(32)
            ),
            hex::encode(&serialized)
        );

        let deserialized = NodeInfo::from_bytes(&serialized, None::<&mut SerializerConfig>).unwrap();
        assert_eq!(node_info, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_nodeinfo_with_ipv6() {
        let obj: NodeInfo = toml::from_str(r#"
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            last_seen = "2025-01-01T00:00:00Z"

            [address]
            V6 = "[::1]:1234"

            [[verification_keys]]
            Ed25519 = "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE"
        "#).unwrap();

       
        let serialized = obj
            .to_bytes(None::<&mut SerializerConfig>)
            .unwrap();

        assert_eq!(
            format!(
                // 01 ipv6
                // 00000000000000000000000000000001 ::1
                // 04d2 port 1234
                // 00000000 timestamp
                // 31 default crypto settings
                "{}010000000000000000000000000000000104d20000000031{}",
                "00".repeat(16),
                "01".repeat(32)
            ),
            hex::encode(&serialized)
        );

        let deserialized = NodeInfo::from_bytes(&serialized, None::<&mut SerializerConfig>).unwrap();
        assert_eq!(obj, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_nodeinfo_with_domain() {
        let obj: NodeInfo = toml::from_str(r#"
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            last_seen = "2025-01-01T00:00:00Z"

            [address]
            Domain = ["example.com", 1234]

            [[verification_keys]]
            Ed25519 = "AQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQEBAQE"
        "#).unwrap();

       
        let serialized = obj
            .to_bytes(None::<&mut SerializerConfig>)
            .unwrap();

        assert_eq!(
            format!(
                // 02 domain
                // 0b length of domain (11 for "example.com")
                // 6578616d706c652e636f6d example.com
                // 00000000000000000000000000000001 ::1
                // 04d2 port 1234
                // 00000000 timestamp
                // 31 default crypto settings
                "{}020b6578616d706c652e636f6d04d20000000031{}",
                "00".repeat(16),
                "01".repeat(32)
            ),
            hex::encode(&serialized)
        );

        let deserialized = NodeInfo::from_bytes(&serialized, None::<&mut SerializerConfig>).unwrap();
        assert_eq!(obj, deserialized);
    }
}
