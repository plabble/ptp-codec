use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::{DisplayFromStr, serde_as};

/// Bucket query structure used for querying bucket data
/// with a specific ID and range.
/// The range can be either numeric or binary, depending on the bucket type.
///
/// # Members
/// - `id`: A 16-byte array representing the unique identifier of the bucket.
/// - `range`: A `BucketRange` enum representing the range of data to query within the bucket.
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct BucketQuery {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    id: [u8; 16],

    #[variant_by = "binary_keys"]
    range: BucketRange,
}

/// Bucket put request structure used for inserting data into a bucket
/// with a specific ID and body. (for PUT request)
///
/// # Members
/// - `id`: A 16-byte array representing the unique identifier of the bucket.
/// - `body`: A `BucketBody` enum representing the data to be inserted into the bucket.
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct PutRequestBody {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    id: [u8; 16],

    #[variant_by = "binary_keys"]
    body: BucketBody,
}

/// Bucket body structure used for representing the data within a bucket.
/// The body can be either numeric or binary, depending on the bucket type.
///
/// This is used for writing or reading data from bucket slots
///
/// # Members
/// - `Numeric`: A hashmap where the key is a `u16` representing the slot number,
///   and the value is a vector of bytes representing the data stored in that slot.
/// - `Binary`: A hashmap where the key is a `String` representing the slot identifier,
///   and the value is a vector of bytes representing the data stored in that slot.
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[no_discriminator]
pub enum BucketBody {
    Numeric(
        #[val_dyn_length]
        #[serde_as(as = "HashMap<DisplayFromStr, Base64<UrlSafe, Unpadded>>")]
        HashMap<u16, Vec<u8>>,
    ),
    Binary(
        #[val_dyn_length]
        #[key_dyn_length]
        #[serde_as(as = "HashMap<_, Base64<UrlSafe, Unpadded>>")]
        HashMap<String, Vec<u8>>,
    ),
}

/// Bucket range structure used for specifying the range of data
/// to query within a bucket.
/// The range can be either numeric or binary, depending on the bucket type.
///
/// # Members
/// - `Numeric`: A tuple containing two optional `u16` values representing optionally
///  the start and/or end of the numeric range
/// - `Binary`: A tuple containing two optional `String` values representing optionally
///  the start and/or end of the binary range.
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[no_discriminator]
pub enum BucketRange {
    Numeric(#[serde(default)] Option<u16>, #[serde(default)] Option<u16>),
    Binary(
        #[dyn_length]
        #[serde(default)]
        Option<String>,
        #[serde(default)] Option<String>,
    ),
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::{
        header::type_and_flags::RequestPacketType, request::PlabbleRequestPacket,
    };

    #[test]
    fn can_serialize_and_deserialize_get_request_numeric() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Get"

            [body]
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            range.Numeric = [5, 25]
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);

        // version = 0001, flags = 0100. Packet type = Get (0010), flags: 0000. 16 bytes id, start 0,5 end 0,25
        assert_eq!(
            vec![
                0b0100_0001,
                0b0000_0010,
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
                5,
                0,
                25
            ],
            serialized
        );
    }

    #[test]
    fn can_serialize_and_deserialize_get_request_numeric_omitting_from() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Get"
            range_mode_until = true

            [body]
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            range.Numeric = [25]
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);

        // version = 0001, flags = 0100. Packet type = Get (0010), flags: 0100. 16 bytes id, end 0,25
        assert_eq!(
            vec![
                0b0100_0001,
                0b0100_0010,
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
                25
            ],
            serialized
        );
        assert!(matches!(
            deserialized.header.packet_type,
            RequestPacketType::Get {
                binary_keys: false,
                subscribe: false,
                range_mode_until: true
            }
        ))
    }

    #[test]
    fn can_serialize_and_deserizalize_get_request_binary() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Get"
            binary_keys = true

            [body]
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            range.Binary = ["key_start", "key_end"]
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);

        // version = 0001, flags = 0100. Packet type = Get (0010), flags: 0001. 16 bytes id, start key_start, end key_end
        assert_eq!(
            vec![
                0b0100_0001,
                0b0001_0010,
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
                9, // key_start length
                b'k',
                b'e',
                b'y',
                b'_',
                b's',
                b't',
                b'a',
                b'r',
                b't',
                b'k',
                b'e',
                b'y',
                b'_',
                b'e',
                b'n',
                b'd'
            ],
            serialized
        );
    }
}
