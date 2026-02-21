use crate::{
    core::BucketId,
    crypto::{derive_key, hash_256},
    packets::base::{PlabblePacketBase, settings::CryptoSettings},
};

/// Connection context for cryptography, counters, session etc.
/// This object is used for handling MAC, encryption, key derivation etc.
#[derive(Debug, Clone)]
pub struct PlabbleConnectionContext {
    /// Get bucket key by bucket ID
    pub get_bucket_key: Option<fn(&BucketId) -> Option<[u8; 32]>>,

    /// Get pre-shared key by ID
    pub get_psk: Option<fn(&[u8; 12]) -> Option<[u8; 64]>>,

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
            get_bucket_key: None,
            get_psk: None,
            session_key: None,
            crypto_settings: None,
            full_encryption: false,
            client_counter: 0,
            server_counter: 0,
            include_bucket_key_in_auth_data: false,
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
        let bucket_key = bucket_id.and_then(|id| self.get_bucket_key.and_then(|f| f(id)));

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
            let pre_shared_key = (self.get_psk?)(&base?.psk_id?)?;
            (pre_shared_key, &base?.psk_salt?)
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
        derive_key(settings.use_blake3, &session_key, salt, context, None)
    }
}

#[cfg(test)]
mod tests {
    use crate::packets::{base::PlabblePacketBase, context::PlabbleConnectionContext};

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

        context.get_psk = Some(|_| Some([0u8; 64]));
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
