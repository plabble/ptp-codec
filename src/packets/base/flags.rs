use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(tag = "packet_type")]
#[no_disc_prefix]
pub enum RequestPacketType {
    Certificate { full_chain: bool, challenge: bool, query_mode: bool },
    Session { persist_key: bool, enable_encryption: bool },
    Get { binary_keys: bool, subscribe: bool, range_mode_until: bool },
    Stream { binary_keys: bool, subscribe: bool, range_mode_until: bool, stream_append: bool },
    Post { binary_keys: bool, subscribe: bool, range_mode_until: bool, do_not_persist: bool },
    Patch,
    Put { binary_keys: bool, subscribe: bool, with_keys: bool, append: bool },
    Delete { binary_keys: bool, range_mode_until: bool },
    Subscribe { binary_keys: bool, range_mode_until: bool },
    Unsubscribe { binary_keys: bool, range_mode_until: bool },
    Register,
    Identify,
    Proxy { init_session: bool, keep_connection: bool, select_random_hops: bool },
    _Reserved13,
    _Reserved14,
    _Reserved15
}

#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(tag = "packet_type")]
#[no_disc_prefix]
pub enum ResponsePacketFlags {
    Certificate,
    Session { with_psk: bool },
    Get { binary_keys: bool },
    Stream,
    Post,
    Patch,
    Put,
    Delete,
    Subscribe,
    Unsubscribe,
    Register,
    Identify,
    Proxy { include_hop_info: bool },
    _Reserved13,
    _Reserved14,
    Error
}


enum RequestBody {
    Session { keys: Vec<Vec<u8>>, psk_expiration: Option<u32> },
    Bucket { bucket_id: [u8; 16], from: Option<u32>, to: Option<u32> },
    BucketBinary { bucket_id: [u8; 16], from: Option<u32>, to: Option<u32> }
    // etc.
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: not yet sure how to fix this
    // The _type can be with postprocess method or custom Deserialize impl
    // But we need to make difference between Request and Response packets
    // Maybe -> PlabblePacket.request() and PlabblePacket.response()
    // For TOML we want separate RequestPacket and ResponsePacket structs inheriting PacketBase

    /*
        PlabbleRequestPacket {
            #[serde(flatten)]
            base: PlabblePacketBase,

            header: RequestPacketHeader,
            body: ... // The problem we had in the header is repeated here...
            // But we can use the auto-tagging here! Because bodies are not really unique
        }
    
     */

    #[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
    struct RequestPacketTest {
        #[serde(skip_serializing, skip_deserializing)]
        #[bits = 4]
        _type: u8,

        #[serde(flatten)]
        #[variant_by = "_type"]
        packet_type: RequestPacketType,
    }

    #[test]
    fn test_flags_toml_representation() {
        let toml = r#"
        packet_type = "Identify"
        "#;

        let deserialized: RequestPacketTest = toml::from_str(&toml).unwrap();

        let t = RequestPacketTest {
            _type: 11,
            packet_type: RequestPacketType::Identify
        };

        assert_eq!(t, deserialized);


        let toml_str = toml::to_string_pretty(&t).unwrap();
        println!("{}", toml_str);

        let bytes = t.to_bytes().unwrap();
        println!("{:?}", bytes.iter().map(|b| format!("{:08b}", b)).collect::<Vec<_>>());

        let deserialized_bytes = RequestPacketTest::from_bytes(&bytes).unwrap();
        assert_eq!(t, deserialized_bytes);

        println!("{:?}", deserialized_bytes);
    }
}