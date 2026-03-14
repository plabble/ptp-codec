use crate::{
    core::PlabbleDateTime,
    crypto::KeyExchange,
    packets::{
        base::{PlabblePacketBase, settings::CryptoSettings},
        body::{
            request_body::PlabbleRequestBody, response_body::PlabbleResponseBody,
            session::SessionRequestBody,
        },
        header::{
            request_header::PlabbleRequestHeader,
            type_and_flags::{RequestPacketType, ResponsePacketType},
        },
        request::PlabbleRequestPacket,
    },
    protocol::{
        PlabbleConnection,
        client::options::{SessionOptions, get_key_exchange_algorithms, set_crypto_settings},
        error::PlabbleProtocolError,
    },
};

/// Client protocol implementation for [`PlabbleConnection`].
impl PlabbleConnection {
    /// Start a new session with the given options. Returns the PSK ID if a pre-shared key is created.
    ///
    /// - `options` is a JSON (or TOML) string containing session options. See [`SessionOptions`] for details.
    /// - Returns the PSK ID as a 12-byte array if a pre-shared key is created, or None if no PSK is used.
    pub async fn start_session(
        &mut self,
        options: Option<SessionOptions>,
    ) -> Result<Option<[u8; 12]>, PlabbleProtocolError> {
        let options = options.unwrap_or_default();
        let mut settings = CryptoSettings::default();
        set_crypto_settings(&mut settings, options.algorithms);

        let mut key_exchanges: Vec<KeyExchange> = get_key_exchange_algorithms(&settings)
            .into_iter()
            .map(|alg| KeyExchange::new(alg))
            .collect();

        let client_salt = if options.client_salt {
            Some(rand::random())
        } else {
            None
        };

        let mut base = PlabblePacketBase::default();
        if settings != CryptoSettings::default() {
            base.specify_crypto_settings = true;
            base.crypto_settings = Some(settings);
        }

        if let Some(psk_id) = options.psk_id {
            base.pre_shared_key = true;
            base.psk_id = Some(psk_id);
            base.psk_salt = Some(rand::random());
        }

        let psk_expiration = options
            .stored_key_lifetime
            .map(|d| PlabbleDateTime::from_now(d));

        let req = PlabbleRequestPacket {
            base,
            header: PlabbleRequestHeader::new(
                RequestPacketType::Session {
                    persist_key: options.stored_key_lifetime.is_some(),
                    enable_encryption: options.enable_full_encryption,
                    with_salt: options.client_salt,
                    request_salt: options.server_salt,
                },
                None,
            ),
            body: PlabbleRequestBody::Session(SessionRequestBody {
                psk_expiration: psk_expiration.clone(),
                salt: client_salt,
                keys: key_exchanges
                    .iter_mut()
                    .map(|kx| kx.make_request().expect("Unsupported algorithm"))
                    .collect(),
            }),
        };

        let res = self.send_and_recv(req).await?;
        if let ResponsePacketType::Session { with_psk, .. } = res.header.packet_type {
            if let PlabbleResponseBody::Session(body) = res.body {
                let shared_secrets = body
                    .keys
                    .into_iter()
                    .enumerate()
                    .map(|(idx, key)| {
                        key_exchanges
                            .get(idx)
                            .and_then(|kx| kx.process_response(&key))
                            .ok_or(PlabbleProtocolError::FailedToProcessResponse)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                let context = self.config.data.as_mut().unwrap();
                context.create_session_key(
                    settings.use_blake3,
                    client_salt,
                    body.salt,
                    shared_secrets,
                );

                if with_psk {
                    let psk_id = body.psk_id.expect("Expected PSK ID");
                    if let Some(store_psk) = &context.store_psk {
                        store_psk(
                            psk_id,
                            context.session_key.clone().unwrap(),
                            psk_expiration.map(|d| d.timestamp()),
                        );
                    }
                    return Ok(Some(psk_id));
                } else {
                    return Ok(None);
                }
            }
        }

        Err(PlabbleProtocolError::UnexpectedResponse)
    }
}
