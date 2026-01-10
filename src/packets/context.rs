use crate::{
    core::BucketId,
    crypto::derive_key,
    packets::base::{PlabblePacketBase, settings::CryptoSettings},
};

/// Connection context for cryptography, counters, session etc.
/// This object is used for handling MAC, encryption, key derivation etc.
#[derive(Clone)]
pub struct PlabbleConnectionContext {
    /// Get bucket key by bucket ID
    pub get_bucket_key: Option<fn(&BucketId) -> Option<[u8; 32]>>,

    /// Get pre-shared key by ID
    pub get_psk: Option<fn(&[u8; 12]) -> Option<[u8; 64]>>,

    /// Session key, if in a session
    pub session_key: Option<[u8; 64]>,

    /// Cryptography settings
    /// altough crypto settings need to be sent in each Plabble packet, it is remembered for the
    /// next packet to make full packet encryption possible. Ofcourse it will be overwritten on each request
    pub crypto_settings: Option<CryptoSettings>,

    /// If full packet encryption is used
    pub full_encryption: bool,

    /// Client packet counter
    pub client_counter: u16,

    /// Server packet counter
    pub server_counter: u16,
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
        }
    }

    /// Create a cryptographic key based on the context and packet base for authentication or encryption
    /// TODO: how to handle full packet encryption?
    ///
    /// # Properties
    /// - `base`: Plabble packet base, if it is available
    /// - `alt_byte`: The byte to add to the context part to randomize the key. 0x77 for the first key and +1 for each other key
    /// - `is_request`: If set, use context string `plabble.req.c` instead of `plabble.res.c`. This ensures that,
    /// even if you got the same counter, the request is still encrypted with another key than the response
    pub fn create_key(
        &self,
        base: Option<&PlabblePacketBase>,
        alt_byte: u8,
        is_request: bool,
    ) -> Option<[u8; 64]> {
        // Get crypto settings from current connection, or from base packet, or get default
        let settings = self
            .crypto_settings
            .clone()
            .or_else(|| base?.crypto_settings.clone())
            .unwrap_or_default();

        // If session key is not already given/in PSK mode retrieve it from the store using PSK
        let (session_key, salt) = if let Some(session_key) = &self.session_key
            && !base
                .and_then(|b| Some(b.pre_shared_key))
                .unwrap_or_default()
        {
            (*session_key, b"PLABBLE.PROTOCOL")
        } else {
            // If it is not given, use a PSK. If that won't resolve, this function will return none
            let session_key = (self.get_psk?)(&base?.psk_id?)?;
            (session_key, &base?.psk_salt?)
        };

        // Context/persona is: `plabble.req.c`/`plabble.res.c` (ASCII) + client/server counter + 0x11 or 0x77 = 16 bytes
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
