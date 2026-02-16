use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

/// Request body for stream operations, which can be either read or write (append) operations on a slot.
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct StreamRequestBody {
    /// Optional data to be written/appended to the slot, present only in write mode
    #[serde(default)]
    #[dyn_length]
    #[toggled_by = "write_mode"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    data: Option<Vec<u8>>,

    /// The slot to be streamed, which can be identified by a numeric ID or a binary key
    /// plus the range of bytes to select
    #[variant_by = "binary_keys"]
    range: SlotRange,
}

/// Response body for stream operations, which can include either the new size of the slot (for writes) or the data read from the slot (for reads).
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct StreamResponseBody {
    /// Optional new size of the slot after a write operation, present only in write mode
    #[serde(default)]
    #[dyn_int]
    #[toggled_by = "write_mode"]
    new_size: Option<u64>,

    /// Optional data read from the slot, present only in read mode
    #[serde(default)]
    #[toggled_by = "!write_mode"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    data: Option<Vec<u8>>,
}

/// Range of bytes within a slot to be streamed. Can be binary or numeric depending on the slot type
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[no_discriminator]
pub enum SlotRange {
    Numeric(
        u16,
        #[serde(default)]
        #[dyn_int]
        Option<u64>,
        #[serde(default)]
        #[dyn_int]
        Option<u64>,
    ),
    Binary(
        #[dyn_length] String,
        #[serde(default)]
        #[dyn_int]
        Option<u64>,
        #[serde(default)]
        #[dyn_int]
        Option<u64>,
    ),
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::{request::PlabbleRequestPacket, response::PlabbleResponsePacket};

    #[test]
    fn can_serialize_and_deserialize_stream_get_request() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Stream"
            id = "AAAAAAAAAAAAAAAAAAAAAA"

            [body]
            # bucket with id AAA.. at slot 7, from byte 08 to byte 0f
            range.Numeric = [7, 8, 0x0f]
        "#,
        )
        .unwrap();

        let bytes = packet.to_bytes(None).unwrap();
        assert_eq!(
            "0103000000000000000000000000000000000007080f",
            hex::encode(&bytes)
        );
        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_stream_write_request() {
        let packet: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Stream"
            id = "AAAAAAAAAAAAAAAAAAAAAA"
            binary_keys = true
            write_mode = true

            [body]
            # bucket with id AAA.. at slot 'test'
            data = "1KKeSJOs"
            range.Binary = ["test"]
        "#,
        )
        .unwrap();

        let data = "d4a29e4893ac";

        let bytes = packet.to_bytes(None).unwrap();
        assert_eq!(
            format!(
                "01930000000000000000000000000000000006{}04{}",
                data,
                hex::encode(b"test")
            ),
            hex::encode(&bytes)
        );
        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_stream_get_response() {
        let packet: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Stream"
            request_counter = 2

            [body]
            data = "1KKeSJOs"
        "#,
        )
        .unwrap();

        let data = "d4a29e4893ac";
        let bytes = packet.to_bytes(None).unwrap();
        assert_eq!(
            format!("01030002{}", data),
            hex::encode(&bytes)
        );

        let deserialized = PlabbleResponsePacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(packet, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_stream_write_response() {
        let packet: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 1

            [header]
            packet_type = "Stream"
            request_counter = 2
            write_mode = true

            [body]
            new_size = 7
        "#,
        )
        .unwrap();

        let bytes = packet.to_bytes(None).unwrap();
        assert_eq!(
            "0113000207",
            hex::encode(&bytes)
        );
        let deserialized = PlabbleResponsePacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(packet, deserialized);
    }
}
