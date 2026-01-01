use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Plabble request packet types.
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(tag = "packet_type")]
#[repr(u8)]
#[no_discriminator]
pub enum RequestPacketType {
    /// Request a certificate from the server.
    /// - full_chain: Request the full certificate chain.
    /// - challenge: Provide a challenge that the server must sign.
    /// - query_mode: Query certificate from server by ID
    Certificate {
        #[serde(default)]
        full_chain: bool,

        #[serde(default)]
        challenge: bool,

        #[serde(default)]
        query_mode: bool,
    },
    /// Start a new session.
    /// - persist_key: Request that the server persist the session key.
    /// - enable_encryption: Request that the server enable encryption for this session.
    /// - with_salt: Provide a salt to the server for session key derivation.
    /// - request_salt: Request that the server (also) provides a salt for session key derivation
    Session {
        #[serde(default)]
        #[toggles("persist_key")]
        persist_key: bool,

        #[serde(default)]
        enable_encryption: bool,

        #[serde(default)]
        #[toggles("client_salt")]
        with_salt: bool,

        #[serde(default)]
        request_salt: bool,
    },
    /// Retrieve values from a bucket
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - subscribe: Subscribe to changes on the requested keys.
    /// - range_mode_until: Use range mode until a specified key/index
    Get {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        subscribe: bool,

        #[serde(default)]
        range_mode_until: bool,
    },
    /// Start a data stream from or to the server
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - subscribe: Subscribe to changes on the requested keys.
    /// - range_mode_until: Use range mode until a specified key/index for the subscription
    /// - stream_append: Write/append mode instead of read mode
    Stream {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        subscribe: bool,

        #[serde(default)]
        range_mode_until: bool,

        #[serde(default)]
        stream_append: bool,
    },
    /// Create a new bucket
    /// - binary_keys: Indicate that the bucket has binary keys
    /// - subscribe: Subscribe to changes on the bucket
    /// - range_mode_until: Use range mode until a specified key/index for the subscription
    /// - do_not_persist: Indicate that the bucket should not be persisted to disk (RAM bucket)
    Post {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        subscribe: bool,

        #[serde(default)]
        range_mode_until: bool,

        #[serde(default)]
        do_not_persist: bool,
    },
    /// Update bucket settings
    Patch,
    /// Insert or update values in a bucket
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - subscribe: Subscribe to changes on the inserted/updated keys.
    /// - assert_keys: Use append mode but provide the keys, and fail if they already exist.
    /// - append: Use append mode to add new entries to the bucket (without keys)
    Put {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        subscribe: bool,

        #[serde(default)]
        assert_keys: bool,

        #[serde(default)]
        append: bool,
    },
    /// Delete values from a bucket
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - range_mode_until: Use range mode until a specified key/index
    Delete {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        range_mode_until: bool,
    },
    /// Subscribe to changes on a bucket, or key(s)/key ranges
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - range_mode_until: Use range mode until a specified key/index
    Subscribe {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        range_mode_until: bool,
    },
    /// Unsubscribe from changes on a bucket, or key(s)/key ranges
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - range_mode_until: Use range mode until a specified key/index
    Unsubscribe {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        range_mode_until: bool,
    },
    /// Create a new identity on the server
    Register,
    /// Prove identity to the server for the current session
    Identify,
    /// Setup a proxy connection through the server
    /// - init_session: Indicates that this is a initial proxy session setup request.
    /// - keep_connection: Request that the server keeps the connection alive for forwarding/receiving packets.
    /// - select_random_hops: Request that the server selects random hops for the proxy route.
    Proxy {
        #[serde(default)]
        init_session: bool,

        #[serde(default)]
        keep_connection: bool,

        #[serde(default)]
        select_random_hops: bool,
    },
    _Reserved13,
    /// Execute a server-side opcode script
    /// - allow_bucket_operations: Allow the script to perform bucket operations.
    /// - allow_eval: Allow the script to use eval functionality (can be dangerous).
    Opcode {
        #[serde(default)]
        allow_bucket_operations: bool,

        #[serde(default)]
        allow_eval: bool,
    },
    _Reserved15,
}

/// Plabble request packet types.
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(tag = "packet_type")]
#[repr(u8)]
#[no_discriminator]
pub enum ResponsePacketType {
    /// Response to a certificate request.
    Certificate,
    /// Response to a session initiation request.
    /// - with_psk: Response contains a PSK ID the server created
    /// - with_salt: Response contains a salt for session key derivation
    Session {
        #[serde(default)]
        #[toggles("key_persisted")]
        with_psk: bool,

        #[serde(default)]
        #[toggles("client_salt")]
        with_salt: bool,
    },
    /// Response to a get request.
    /// - binary_keys: Keys in the response are in binary format.
    Get {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,
    },
    /// Response to a stream request.
    Stream,
    /// Response to a post request.
    Post,
    /// Response to a patch request.
    Patch,
    /// Response to a put request.
    Put,
    /// Response to a delete request.
    Delete,
    /// Response to a subscribe request.
    Subscribe,
    /// Response to an unsubscribe request.
    Unsubscribe,
    /// Response to a register identity request.
    Register,
    /// Response to an identify request.
    Identify,
    /// Response to a proxy request.
    /// - include_hop_info: Response includes information about the selected hops.
    Proxy {
        #[serde(default)]
        include_hop_info: bool,
    },
    _Reserved13,
    /// Response to an opcode execution request.
    Opcode,
    /// Error response
    Error,
}
