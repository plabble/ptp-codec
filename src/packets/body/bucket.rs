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
/// - `range`: A `BucketRange` enum representing the range of data to query within the bucket.
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct BucketQuery {
    #[variant_by = "binary_keys"]
    range: BucketRange,
}

/// Bucket put request structure used for inserting data into a bucket
/// with a specific ID and body. (for PUT request)
///
/// # Members
/// - `body`: A `BucketBody` enum representing the data to be inserted into the bucket.
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct PutRequestBody {
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
    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    use crate::{
        core::BucketId,
        packets::{
            header::type_and_flags::RequestPacketType, request::PlabbleRequestPacket,
            response::PlabbleResponsePacket,
        },
    };

    #[test]
    fn can_serialize_and_deserialize_get_request_numeric() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Get"
            id = "AAAAAAAAAAAAAAAAAAAAAA"

            [body]
            range.Numeric = [5, 25]
        "#,
        )
        .unwrap();

        let mut config = SerializerConfig::new(None);
        let serialized = packet.to_bytes(Some(&mut config)).unwrap();
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
            id = "@test"
            range_mode_until = true

            [body]
            range.Numeric = [25]
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);
        assert_eq!(
            deserialized.header.id,
            Some(BucketId::parse("@test").unwrap())
        );

        // version = 0001, flags = 0100. Packet type = Get (0010), flags: 0100. 16 bytes id, end 0,25
        assert_eq!(
            vec![
                0b0100_0001,
                0b0100_0010,
                72,
                120,
                202,
                4,
                37,
                199,
                57,
                250,
                66,
                127,
                126,
                218,
                32,
                254,
                132,
                95,
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
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            binary_keys = true

            [body]
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

    // empty numeric, empty binary, single binary (but not necessary in GET, can also be other bucket)

    #[test]
    fn can_serialize_and_deserialize_get_numeric_response() {
        let packet: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Get"
            request_counter = 1

            [body.Numeric]
            5 = "AAAAAA"
            7 = "AAAAAAAA"
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();

        // version = 0001, flags = 0100. Packet type = Get (0010), flags: 0000.
        // request counter: 0,1  nx [ key: X,X (0,5) or (0,7) (we don't know the hashmap order for sure, it really can differ per time...)
        // length (dynint, in this case 1 byte) then bytes ]
        let case1 = vec![
            0b0100_0001,
            0b0000_0010,
            0,
            1,
            0,
            5,
            4,
            0,
            0,
            0,
            0,
            0,
            7,
            6,
            0,
            0,
            0,
            0,
            0,
            0,
        ];
        let case2 = vec![
            0b0100_0001,
            0b0000_0010,
            0,
            1,
            0,
            7,
            6,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            5,
            4,
            0,
            0,
            0,
            0,
        ];

        assert!(serialized == case1 || serialized == case2);

        let deserialized = PlabbleResponsePacket::from_bytes(&serialized, None).unwrap();
        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_get_binary_response() {
        let packet: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Get"
            request_counter = 1
            binary_keys = true

            [body.Binary]
            name = "AAAA"
            alias = "AAAAAA"
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();

        // version = 0001, flags = 0100. Packet type = Get (0010), flags: 0001.
        // request counter: 0,1  nx [ key length, key, length (dynint, in this case 1 byte) then bytes ]
        let case1 = vec![
            0b0100_0001,
            0b0001_0010,
            0,
            1,
            4,
            b'n',
            b'a',
            b'm',
            b'e',
            3,
            0,
            0,
            0,
            5,
            b'a',
            b'l',
            b'i',
            b'a',
            b's',
            4,
            0,
            0,
            0,
            0,
        ];
        let case2 = vec![
            0b0100_0001,
            0b0001_0010,
            0,
            1,
            5,
            b'a',
            b'l',
            b'i',
            b'a',
            b's',
            4,
            0,
            0,
            0,
            0,
            4,
            b'n',
            b'a',
            b'm',
            b'e',
            3,
            0,
            0,
            0,
        ];

        assert!(serialized == case1 || serialized == case2);

        let deserialized = PlabbleResponsePacket::from_bytes(&serialized, None).unwrap();
        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_put_request_numeric() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Put"
            id = "AAAAAAAAAAAAAAAAAAAAAA"

            [body]
            body.Numeric = { 5 = "AAAAAA" }
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        assert_eq!(
            "41060000000000000000000000000000000000050400000000",
            hex::encode(&serialized)
        );
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_put_request_binary() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Put"
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            binary_keys = true

            [body]
            body.Binary = { name = "AAAA", alias = "AAAAAA" }
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let hexed = hex::encode(&serialized);
        let expected1 =
            "411600000000000000000000000000000000046e616d650300000005616c6961730400000000";
        let expected2 =
            "41160000000000000000000000000000000005616c6961730400000000046e616d6503000000";
        assert!(hexed == expected1 || hexed == expected2, "got {}", hexed);
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_delete_request_numeric() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Delete"
            id = "AAAAAAAAAAAAAAAAAAAAAA"

            [body]
            range.Numeric = [5, 25]
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        assert_eq!(
            "41070000000000000000000000000000000000050019",
            hex::encode(&serialized)
        );
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_subscribe_request_numeric() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Subscribe"
            id = "AAAAAAAAAAAAAAAAAAAAAA"

            [body]
            range.Numeric = [5, 25]
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_unsubscribe_request_numeric() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            use_encryption = true

            [header]
            packet_type = "Unsubscribe"
            id = "AAAAAAAAAAAAAAAAAAAAAA"

            [body]
            range.Numeric = [5, 25]
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();

        assert_eq!(packet, deserialized);
    }
}
