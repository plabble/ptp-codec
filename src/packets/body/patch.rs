use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::packets::body::post::BucketPermissions;

/// Change bucket permissions or ACL entries.
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct PatchRequestBody {
    /// If toggled by flags, update the permissions of the bucket
    #[toggled_by = "update_perm"]
    permissions: Option<BucketPermissions>,

    /// If toggled by flags, add the following ACL entries (user IDs/user certificate IDs) to the bucket (without overwriting existing ACL)
    #[toggled_by = "acl_add"]
    #[serde_as(as = "Option<Vec<Base64<UrlSafe, Unpadded>>>")]
    #[dyn_length]
    acl_add: Option<Vec<[u8; 16]>>,

    /// If toggled by flags, remove the following ACL entries (user IDs/user certificate IDs) from the bucket
    #[toggled_by = "acl_del"]
    #[serde_as(as = "Option<Vec<Base64<UrlSafe, Unpadded>>>")]
    #[dyn_length]
    acl_del: Option<Vec<[u8; 16]>>,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::request::PlabbleRequestPacket;

    #[test]
    fn can_serialize_and_deserialize_patch_request_permissions() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Patch"
            id = "@test"
            update_permissions = true

            [body]
            [body.permissions]
            public_read = true
            public_write = true
            protected_delete = true
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_patch_request_acl_add() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Patch"
            id = "@test"
            add_to_acl = true

            [body]
            acl_add = ["AAAAAAAAAAAAAAAAAAAAAA"]
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_patch_request_acl_add_and_del() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Patch"
            id = "@test"
            add_to_acl = true
            remove_from_acl = true

            [body]
            acl_add = ["AAAAAAAAAAAAAAAAAAAAAA"]
            acl_del = ["AAAAAAAAAAAAAAAAAAAAAA"]
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);
    }
}