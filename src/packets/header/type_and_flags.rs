use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

/// Plabble request packet type, header flags & optionally other header fields
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(tag = "packet_type")]
#[repr(u8)]
#[no_discriminator]
pub enum RequestPacketType {
    /// Request a certificate from the server.
    /// - full_chain: Request the full certificate chain, or just a single certificate
    /// - full_certs: Request full certificates or just summary
    /// - challenge: Provide a challenge that the server must sign.
    /// - query_mode: Query certificate from server by ID, or the server's main certificate
    Certificate {
        #[serde(default)]
        full_chain: bool,

        #[serde(default)]
        full_certs: bool,

        #[serde(default)]
        #[toggles("challenge")]
        challenge: bool,

        #[serde(default)]
        #[toggles("query_mode")]
        query_mode: bool,
    } = 0,
    /// Start a new session.
    /// - persist_key: Request that the server persist the session key.
    /// - enable_encryption: Request that the server enables full packet encryption for this session.
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
    } = 1,
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
    } = 2,
    /// Start a data stream from or to a bucket
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - subscribe: Subscribe to changes on the requested range.
    /// - range_mode_until: Use range mode until a specified key/index for the subscription
    /// - write_mode: Write/append mode instead of read mode
    Stream {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        subscribe: bool,

        #[serde(default)]
        range_mode_until: bool,

        #[serde(default)]
        #[toggles("write_mode")]
        write_mode: bool,
    } = 3,
    /// Create a new bucket
    /// - binary_keys: Indicate that the bucket (or at least the subscribe range) has binary keys
    /// - subscribe: Subscribe to changes on the bucket
    /// - range_mode_until: Use range mode until a specified key/index for the subscription
    /// - do_not_persist: Indicate that the bucket should not be persisted to disk (RAM bucket)
    Post {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        #[toggles("subscribe")]
        subscribe: bool,

        #[serde(default)]
        range_mode_until: bool,

        #[serde(default)]
        do_not_persist: bool,
    } = 4,
    /// Update bucket settings
    Patch {
        #[serde(default)]
        #[toggles("update_perm")]
        update_permissions: bool,

        #[serde(default)]
        #[toggles("acl_add")]
        add_to_acl: bool,

        #[serde(default)]
        #[toggles("acl_del")]
        remove_from_acl: bool,
    } = 5,
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
    } = 6,
    /// Delete values from a bucket
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - range_mode_until: Use range mode until a specified key/index
    Delete {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        range_mode_until: bool,
    } = 7,
    /// Subscribe to changes on a bucket, or key(s)/key ranges
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - range_mode_until: Use range mode until a specified key/index
    Subscribe {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        range_mode_until: bool,
    } = 8,
    /// Unsubscribe from changes on a bucket, or key(s)/key ranges
    /// - binary_keys: Indicate that the keys are in binary format.
    /// - range_mode_until: Use range mode until a specified key/index
    Unsubscribe {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,

        #[serde(default)]
        range_mode_until: bool,
    } = 9,
    /// Create a new identity on the server
    Register = 10,
    /// Prove identity to the server for the current session
    Identify = 11,
    /// Setup a proxy connection through the server
    /// - init_session: Indicates that this is a initial proxy session setup request.
    /// - keep_connection: Request that the server keeps the connection alive for forwarding/receiving packets.
    /// - select_random_hops: Request that the server selects random hops for the proxy route.
    Proxy {
        #[serde(default)]
        #[toggles("init_session")]
        init_session: bool,

        #[serde(default)]
        keep_connection: bool,

        #[serde(default)]
        #[toggles("random_hops")]
        select_random_hops: bool,
    } = 12,
    /// Custom packet type for supporting sub-protocols
    Custom {
        #[serde(default)]
        flag1: bool,
        #[serde(default)]
        flag2: bool,
        #[serde(default)]
        flag3: bool,
        #[serde(default)]
        flag4: bool,
    } = 13,
    /// Execute a server-side opcode script
    /// - allow_bucket_operations: Allow the script to perform bucket operations.
    /// - allow_eval: Allow the script to use eval functionality (can be dangerous). Only if the server allows it too.
    Opcode {
        #[serde(default)]
        allow_bucket_operations: bool,

        #[serde(default)]
        allow_eval: bool,
    } = 14,
}

/// Plabble response packet types, header flags & optionally other header fields
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(tag = "packet_type")]
#[repr(u8)]
#[no_discriminator]
pub enum ResponsePacketType {
    /// Response to a certificate request.
    Certificate = 0,
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
    } = 1,
    /// Response to a get request.
    /// - binary_keys: Keys in the response are in binary format.
    Get {
        #[serde(default)]
        #[toggles("binary_keys")]
        binary_keys: bool,
    } = 2,
    /// Response to a stream request.
    /// - write_mode: Indicates a write response (so no data returned)
    Stream {
        #[serde(default)]
        #[toggles("write_mode")]
        write_mode: bool,
    } = 3,
    /// Response to a post request.
    Post = 4,
    /// Response to a patch request.
    Patch = 5,
    /// Response to a put request.
    Put = 6,
    /// Response to a delete request.
    Delete = 7,
    /// Response to a subscribe request.
    Subscribe = 8,
    /// Response to an unsubscribe request.
    Unsubscribe = 9,
    /// Response to a register identity request.
    Register = 10,
    /// Response to an identify request.
    Identify = 11,
    /// Response to a proxy request.
    /// - init_session: Indicates that this is a response to an initial proxy session setup request.
    Proxy {
        #[serde(default)]
        #[toggles("init_session")]
        init_session: bool
    } = 12,
    /// Custom packet type for supporting sub-protocols
    Custom {
        #[serde(default)]
        flag1: bool,
        #[serde(default)]
        flag2: bool,
        #[serde(default)]
        flag3: bool,
        #[serde(default)]
        flag4: bool,
    } = 13,
    /// Response to an opcode execution request.
    Opcode = 14,
    /// Error response
    Error = 15,
}
