use std::sync::Arc;

use crate::{
    core::BucketId,
    crypto::{derive_key, hash_256, hash_512},
    packets::base::{PlabblePacketBase, settings::CryptoSettings},
};

// Key/storage provider for Plabble Connection
pub trait KeyProvider: Send + Sync {
    /// Given a bucket ID serialized as bytes, return the 32-byte bucket key, or None.
    fn get_bucket_key(&self, bucket_id: &[u8; 16]) -> Option<[u8; 32]>;

    /// Given a 12-byte PSK ID, return the 64-byte pre-shared key, or None.
    fn get_psk(&self, psk_id: &[u8; 12]) -> Option<[u8; 64]>;

    /// Store a pre-shared key with the given PSK ID and optional expiration time (as a UNIX timestamp).
    fn store_psk(&self, psk_id: [u8; 12], psk: [u8; 64], expiration: Option<u32>);
}

/// Connection context for cryptography, counters, session etc.
/// This object is used for handling MAC, encryption, key derivation etc.
#[derive(Clone)]
pub struct PlabbleConnectionContext {
    /// Key provider for looking up bucket keys/PSKs
    pub key_provider: Option<Arc<dyn KeyProvider>>,

    /// Session key, if in a session
    pub session_key: Option<[u8; 64]>,

    /// Cryptography settings
    /// Will be remembered for an entire session, but will be overwritten with any packet that specifies crypto settings
    pub crypto_settings: Option<CryptoSettings>,

    /// If full packet encryption is used
    pub full_encryption: bool,

    /// Client packet counter
    pub client_counter: u16,

    /// Server packet counter
    pub server_counter: u16,

    /// When sending a packet, whether to include the bucket key in the authenticated data (for MAC and encryption).
    pub include_bucket_key_in_auth_data: bool,

    /// PSK used in the current connection
    pub session_psk: Option<[u8; 64]>,

    /// PSK salt used in the current connection
    pub session_salt: Option<[u8; 16]>,
}

impl Default for PlabbleConnectionContext {
    fn default() -> Self {
        Self::new()
    }
}

impl PlabbleConnectionContext {
    /// Create new connection context (for new connection)
    pub fn new() -> Self {
        Self {
            key_provider: None,
            session_key: None,
            crypto_settings: None,
            full_encryption: false,
            client_counter: 0,
            server_counter: 0,
            include_bucket_key_in_auth_data: false,
            session_psk: None,
            session_salt: None,
        }
    }

    /// Increment client/server counter based on packet type
    pub fn increment(&mut self, is_request: bool) {
        if is_request {
            self.client_counter = self.client_counter.wrapping_add(1);
        } else {
            self.server_counter = self.server_counter.wrapping_add(1);
        }
    }

    /// Indicates if current context crypto settings require blake3 hashing (for MAC and key derivation)
    pub fn use_blake3(&self) -> bool {
        self.crypto_settings.as_ref().is_some_and(|s| s.use_blake3)
    }

    /// Create authenticated data for the packet, based on the base and header bytes and optionally bucket key
    ///
    /// The authenticated data is used for MAC and encryption, to ensure integrity and authenticity of the packet.
    /// The authenticated data is created by hashing the base and header bytes, and optionally the bucket key if available.
    ///
    /// # Parameters
    /// - `raw_base_and_header`: The raw bytes of the packet base and header,
    /// - `bucket_id`: The bucket ID, used to retrieve the bucket key if available. If not given or not found, the bucket key is not included in the authenticated data.
    ///
    /// # Returns
    /// The authenticated data as a 32-byte array, or None if hashing failed (when blake3 is requested but not supported by server).
    pub fn create_authenticated_data(
        &self,
        raw_base_and_header: &[u8],
        bucket_id: Option<&BucketId>,
    ) -> [u8; 32] {
        let bucket_key = bucket_id.and_then(|id| {
            self.key_provider
                .as_ref()
                .and_then(|provider| provider.get_bucket_key(&id.data))
        });

        let mut data = Vec::new();
        data.push(raw_base_and_header);
        if let Some(ref bucket_key) = bucket_key {
            data.push(bucket_key);
        }

        hash_256(self.use_blake3(), data)
    }

    /// Create a cryptographic key based on the context and packet base for authentication or encryption
    /// - The keys are never reused, for each part of the packet is a new key generated thanks to `alt_byte`
    /// - Every packet has a unique key thanks to the counters
    /// - Request and response packet with same counter and alt have still a different key thanks to `is_request`
    ///
    /// # Properties
    /// - `base`: Plabble packet base, if it is available
    /// - `alt_byte`: The byte to add to the context part to randomize the key.
    /// - `is_request`: If set, use context string `plabble.req.c` instead of `plabble.res.c`. This ensures that,
    /// even if you got the same counter, the request is still encrypted with another key than the response
    pub fn create_key(
        &self,
        base: Option<&PlabblePacketBase>,
        alt_byte: u8,
        is_request: bool,
    ) -> Option<[u8; 64]> {
        // Get crypto settings from current connection, or from base packet (if already available), or get default
        let settings = self
            .crypto_settings
            .or_else(|| base?.crypto_settings)
            .unwrap_or_default();

        // If not session key is already given/session is in PSK mode, retrieve it from the store using PSK
        let (session_key, salt) = if let Some(session_key) = &self.session_key
            && !base.map(|b| b.pre_shared_key).unwrap_or_default()
        {
            (*session_key, b"PLABBLE.PROTOCOL")
        } else {
            // If it is not given, use a PSK. If that won't resolve, this function will return none
            // Try from base packet first if pre_shared_key is set, otherwise try session PSK
            if let Some(base) = base
                && base.pre_shared_key
            {
                let psk = self.key_provider.as_ref()?.get_psk(&base.psk_id?)?;
                (psk, &base.psk_salt?)
            } else {
                (self.session_psk?, &self.session_salt?)
            }
        };

        // Context/persona is: `plabble.req.c`/`plabble.res.c` (ASCII) + client/server counter + (alt byte) = 16 bytes
        let mut context = Vec::new();
        context.extend_from_slice(if is_request {
            b"plabble.req.c"
        } else {
            b"plabble.res.c"
        });
        context.extend_from_slice(
            &(if is_request {
                self.client_counter
            } else {
                self.server_counter
            })
            .to_be_bytes(),
        );
        context.push(alt_byte);

        let context: &[u8; 16] = &context.try_into().unwrap();
        let key = derive_key(settings.use_blake3, &session_key, salt, context, None);
        Some(key)
    }

    /// Create session key from shared secrets and salts, and store it in the context
    pub fn create_session_key(
        &mut self,
        blake_3: bool,
        client_salt: Option<[u8; 16]>,
        server_salt: Option<[u8; 16]>,
        shared_secrets: Vec<[u8; 32]>,
    ) {
        let salt = client_salt.or(server_salt).unwrap_or(*b"PLABBLE-PROTOCOL");
        let context = server_salt.unwrap_or(*b"PROTOCOL.PLABBLE");
        let ikm = hash_512(
            blake_3,
            shared_secrets.iter().map(|s| s.as_slice()).collect(),
        );
        let session_key = derive_key(blake_3, &ikm, &salt, &context, None);
        self.session_key = Some(session_key);
    }

    /// Generate a bucket key for the given bucket ID, using the session key and a fixed context string.
    pub fn create_bucket_key(&self, blake_3: bool, bucket_id_bytes: &[u8; 16]) -> Option<[u8; 64]> {
        let key = derive_key(
            blake_3,
            self.session_key.as_ref()?,
            b"PLABBLE___BUCKET",
            bucket_id_bytes,
            None,
        );
        Some(key)
    }
}

#[cfg(test)]
pub mod helpers {
    use crate::packets::context::KeyProvider;

    pub struct ExampleKeyProvider;
    impl KeyProvider for ExampleKeyProvider {
        fn get_bucket_key(&self, _bucket_id: &[u8; 16]) -> Option<[u8; 32]> {
            Some([0; 32])
        }

        fn get_psk(&self, _psk_id: &[u8; 12]) -> Option<[u8; 64]> {
            Some([0; 64])
        }

        fn store_psk(&self, _psk_id: [u8; 12], _psk: [u8; 64], _expiration: Option<u32>) {
            // Do nothing for testing
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::packets::{base::PlabblePacketBase, context::{PlabbleConnectionContext, helpers::ExampleKeyProvider}};

    #[test]
    fn keys_are_unique_by_alt_byte_and_is_request() {
        let mut context = PlabbleConnectionContext::new();
        context.session_key = Some([0u8; 64]);
        let key1 = context.create_key(None, 0, true).unwrap();
        let key2 = context.create_key(None, 0, false).unwrap();
        let key3 = context.create_key(None, 1, true).unwrap();
        let key4 = context.create_key(None, 1, false).unwrap();

        assert_ne!(key1, key2);
        assert_ne!(key1, key3);
        assert_ne!(key1, key4);
        assert_ne!(key2, key3);
        assert_ne!(key2, key4);
        assert_ne!(key3, key4);

        // Same alt_byte, is_request and session_key = same key
        let key1b = context.create_key(None, 0, true).unwrap();
        assert_eq!(key1, key1b);

        // But not if the session key changed
        context.session_key = Some([1u8; 64]);
        let key1c = context.create_key(None, 0, true).unwrap();
        assert_ne!(key1, key1c);

        // Base packet with PSK ID should override session key
        let mut base = PlabblePacketBase {
            version: 0,
            fire_and_forget: false,
            use_encryption: false,
            pre_shared_key: true,
            specify_crypto_settings: false,
            crypto_settings: None,
            psk_id: Some([0u8; 12]),
            psk_salt: Some([0u8; 16]),
        };

        context.key_provider = Some(Arc::new(ExampleKeyProvider));
        let key4 = context.create_key(Some(&base), 0, true).unwrap();
        assert_ne!(key1, key4);
        assert_ne!(key2, key4);
        assert_ne!(key3, key4);

        // If the salt is different, the key should be different too
        base.psk_salt = Some([1u8; 16]);
        let key5 = context.create_key(Some(&base), 0, true).unwrap();
        assert_ne!(key4, key5);
    }
}
