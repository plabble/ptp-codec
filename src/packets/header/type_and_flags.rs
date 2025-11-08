use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(tag = "packet_type")]
#[repr(u8)]
#[no_discriminator]
pub enum RequestPacketType {
    Certificate {
        #[serde(default)]
        full_chain: bool,
        #[serde(default)]
        challenge: bool,
        #[serde(default)]
        query_mode: bool,
    },
    Session {
        #[serde(default)]
        persist_key: bool,
        #[serde(default)]
        enable_encryption: bool,
    },
    Get {
        #[serde(default)]
        binary_keys: bool,
        #[serde(default)]
        subscribe: bool,
        #[serde(default)]
        range_mode_until: bool,
    },
    Stream {
        #[serde(default)]
        binary_keys: bool,
        #[serde(default)]
        subscribe: bool,
        #[serde(default)]
        range_mode_until: bool,
        #[serde(default)]
        stream_append: bool,
    },
    Post {
        #[serde(default)]
        binary_keys: bool,
        #[serde(default)]
        subscribe: bool,
        #[serde(default)]
        range_mode_until: bool,
        #[serde(default)]
        do_not_persist: bool,
    },
    Patch,
    Put {
        #[serde(default)]
        binary_keys: bool,
        #[serde(default)]
        subscribe: bool,
        #[serde(default)]
        with_keys: bool,
        #[serde(default)]
        append: bool,
    },
    Delete {
        #[serde(default)]
        binary_keys: bool,
        #[serde(default)]
        range_mode_until: bool,
    },
    Subscribe {
        #[serde(default)]
        binary_keys: bool,
        #[serde(default)]
        range_mode_until: bool,
    },
    Unsubscribe {
        #[serde(default)]
        binary_keys: bool,
        #[serde(default)]
        range_mode_until: bool,
    },
    Register,
    Identify,
    Proxy {
        #[serde(default)]
        init_session: bool,
        #[serde(default)]
        keep_connection: bool,
        #[serde(default)]
        select_random_hops: bool,
    },
    _Reserved13,
    _Reserved14,
    _Reserved15,
}

#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(tag = "packet_type")]
#[repr(u8)]
#[no_discriminator]
pub enum ResponsePacketType {
    Certificate,
    Session {
        #[serde(default)]
        with_psk: bool,
    },
    Get {
        #[serde(default)]
        binary_keys: bool,
    },
    Stream,
    Post,
    Patch,
    Put,
    Delete,
    Subscribe,
    Unsubscribe,
    Register,
    Identify,
    Proxy {
        #[serde(default)]
        include_hop_info: bool,
    },
    _Reserved13,
    _Reserved14,
    Error,
}
