use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(tag = "packet_type")]
#[repr(u8)]
#[no_discriminator]
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
#[repr(u8)]
#[no_discriminator]
pub enum ResponsePacketType {
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
