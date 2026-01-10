use crate::{core::BucketId, crypto::derive_key, packets::base::PlabblePacketBase};

/// Request context for cryptography, session etc.
#[derive(Clone)]
pub struct PlabblePacketContext {
    /// Get bucket key by bucket ID
    get_bucket_key: fn(&BucketId) -> Option<[u8; 32]>,

    /// Get pre-shared key by ID
    get_psk: fn(&[u8; 12]) -> Option<[u8; 64]>,

    /// Session key, if in a session
    session_key: Option<[u8; 64]>,

    /// Client packet counter
    client_counter: u16,

    /// Server packet counter
    server_counter: u16,
}

impl PlabblePacketContext {
    /// Create a cryptographic key based on the context and packet base for authentication or encryption
    /// 
    /// # Properties
    /// - `base`: Plabble packet base
    /// - `alt_key`: If set, use 0x77 instead of 0x11 as last byte of context (giving another key using same input key)
    /// - `is_request`: If set, use context string `plabble.req.c` instead of `plabble.res.c`. This ensures that,
    /// even if you got the same counter, the request is still encrypted with another key than the response
    pub fn create_key(
        &self,
        base: &PlabblePacketBase,
        alt_key: bool,
        is_request: bool,
    ) -> Option<[u8; 64]> {
        let settings = base.crypto_settings.clone().unwrap_or_default();

        // If session key is not already given/in PSK mode retrieve it from the store using PSK
        let (session_key, salt) = if let Some(session_key) = &self.session_key
            && !base.pre_shared_key
        {
            (*session_key, b"PLABBLE.PROTOCOL")
        } else {
            // If it is not given, use a PSK. If that won't resolve, this function will return none
            let session_key = (self.get_psk)(&base.psk_id?)?;
            (session_key, &base.psk_salt?)
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
        context.push(if alt_key { 0x77 } else { 0x11 });
        let context: &[u8; 16] = &context.try_into().unwrap();

        derive_key(settings.use_blake3, &session_key, salt, context)
    }
}
