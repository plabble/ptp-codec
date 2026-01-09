use crate::default_true;
use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::core::BucketId;
use crate::packets::body::bucket::BucketRange;

/// Bucket Permissions come in 3 flavours:
/// - `public`: everyone on the internet who knows your bucket ID can do this
/// - `protected`: only people who are authenticated using IDENTITY and are on the *access_control_list* can do this
/// - `private`: only people who know the _bucket key_ can do this
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct BucketPermissions {
    /// Allow everyone to read slots from this bucket
    #[serde(default = "default_true")]
    public_read: bool,
    /// Allow everyone to append a slot to the bucket
    #[serde(default)]
    public_append: bool,
    /// Allow everyone to update a slot
    #[serde(default)]
    public_write: bool,
    /// Allow everyone to delete a slot
    #[serde(default)]
    public_delete: bool,
    /// Allow everyone to execute OPCODE scripts interacting with this bucket (read/write/append/delete)
    #[serde(default)]
    public_script_execution: bool,

    /// Allow authenticated users on the ACL to read slots from this bucket
    #[serde(default = "default_true")]
    protected_read: bool,
    /// Allow authenticated users on the ACL to append a slot to the bucket
    #[serde(default)]
    protected_append: bool,
    /// Allow authenticated users on the ACL to update a slot
    #[serde(default)]
    protected_write: bool,
    /// Allow authenticated users on the ACL to delete a slot
    #[serde(default)]
    protected_delete: bool,
    /// Allow authenticated users on the ACL to execute OPCODE scripts interacting with this bucket (read/write/append/delete)
    #[serde(default)]
    protected_script_execution: bool,
    /// Allow authenticated users on the ACL to delete this bucket
    #[serde(default)]
    protected_bucket_delete: bool,

    /// Allow authenticated users using the bucket key to read slots from this bucket
    #[serde(default = "default_true")]
    private_read: bool,
    /// Allow authenticated users using the bucket key to append a slot to the bucket
    #[serde(default = "default_true")]
    private_append: bool,
    /// Allow authenticated users using the bucket key to update a slot
    #[serde(default = "default_true")]
    private_write: bool,
    /// Allow authenticated users using the bucket key to delete a slot
    #[serde(default = "default_true")]
    private_delete: bool,
    /// Allow authenticated users using the bucket key to execute OPCODE scripts interacting with this bucket (read/write/append/delete)
    #[serde(default)]
    private_script_execution: bool,
    /// Allow authenticated users using the bucket key delete this bucket
    #[serde(default = "default_true")]
    private_bucket_delete: bool,

    /// If public read is off and a user queries this bucket, let the server tell
    /// them this bucket does not exist
    #[serde(default)]
    deny_existence: bool, // 6 reserved flags (total: 20 = 5 bytes)
}

/// Bucket settings
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct BucketSettings {
    /// Permissions
    permissions: BucketPermissions,

    /// Access Control List (ACL) with user IDs
    #[serde_as(as = "Vec<Base64<UrlSafe, Unpadded>>")]
    #[dyn_length]
    access_control_list: Vec<[u8; 20]>,
}

/// Bucket create request body
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct PostRequestBody {
    /// The ID the user wants to assign to the bucket
    id: BucketId,

    /// Bucket settings
    settings: BucketSettings,

    /// Bucket range, if subscribe flag is set
    #[toggled_by = "subscribe"]
    #[variant_by = "binary_keys"]
    range: Option<BucketRange>,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::request::PlabbleRequestPacket;

    #[test]
    fn can_serialize_and_deserialize_post_request() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Post"
            subscribe = true

            [body]
            id = "@test"
            range.Numeric = [1]

            settings.access_control_list = [
                "AAAAAAAAAAAAAAAAAAAAAAAAAAA"
            ]

            [body.settings.permissions]
            public_read = true
            public_write = true
            protected_delete = true
            private_append = false
            private_bucket_delete = false
            deny_existence = true
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(deserialized, packet);

        assert_eq!(
            serialized,
            vec![
                0b0100_0001, // version 0001, flags: 0100
                0b0010_0100, // packet type 4, packet flags: 0010
                // 16-byte bucket id
                0x48,
                0x78,
                0xca,
                0x04,
                0x25,
                0xc7,
                0x39,
                0xfa,
                0x42,
                0x7f,
                0x7e,
                0xda,
                0x20,
                0xfe,
                0x84,
                0x5f,
                // settings.permissions, 3 byte//
                0b0010_0101, // wars_dwar
                0b0110_1001, // sdwa_rbsd
                0b0000_0010, // xxxx_xxdb
                1,           // ACL length
                // 20-byte ID(s)
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
                0,
                0,
                0,
                0,
                // numeric range, 2 byte (only start)
                0,
                1
            ]
        )
    }
}
