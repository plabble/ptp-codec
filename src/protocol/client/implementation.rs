use crate::{
    core::PlabbleDateTime,
    crypto::KeyExchange,
    packets::{
        base::{PlabblePacketBase, settings::CryptoSettings},
        body::{
            error::PlabbleError, request_body::PlabbleRequestBody,
            response_body::PlabbleResponseBody, session::SessionRequestBody,
        },
        header::{
            request_header::PlabbleRequestHeader,
            type_and_flags::{RequestPacketType, ResponsePacketType},
        },
        request::PlabbleRequestPacket,
    },
    protocol::{
        PlabbleConnection,
        client::options::{
            SessionOptions, get_key_exchange_algorithms, is_valid_algorithm, set_crypto_settings,
        },
        error::PlabbleProtocolError,
        options::{get_signature_algorithms, unsupported_algorithm},
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
        if let Some(name) = options.algorithms.iter().find_map(|algorithm| {
            let name = algorithm.strip_prefix('!').unwrap_or(algorithm);
            (!is_valid_algorithm(name)).then_some(name)
        }) {
            return Err(PlabbleError::UnsupportedAlgorithm {
                name: name.to_owned(),
            }
            .into());
        }

        let mut settings = self
            .config
            .data
            .as_ref()
            .and_then(|context| context.crypto_settings)
            .unwrap_or_default();

        set_crypto_settings(&mut settings, options.algorithms);

        if let Some(name) = unsupported_algorithm(&settings) {
            return Err(PlabbleError::UnsupportedAlgorithm {
                name: name.to_owned(),
            }
            .into());
        }

        let key_exchange_algorithms = get_key_exchange_algorithms(&settings);
        if key_exchange_algorithms.is_empty() {
            return Err(PlabbleError::InvalidRequest.into());
        }

        let mut key_exchanges: Vec<KeyExchange> = key_exchange_algorithms
            .into_iter()
            .map(KeyExchange::new)
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
            base.use_encryption = true;
            base.psk_id = Some(psk_id);
            base.psk_salt = Some(rand::random());
        }

        let psk_expiration = options.stored_key_lifetime.map(PlabbleDateTime::from_now);

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
                    .map(|kx| {
                        kx.make_request()
                            .ok_or(PlabbleProtocolError::FailedToProcessRequest)
                    })
                    .collect::<Result<Vec<_>, _>>()?,
            }),
        };

        let signing_request = req.clone();
        let res = self.send_and_recv(req).await?;

        if let PlabbleResponseBody::Error(error) = &res.body {
            return Err(error.clone().into());
        }

        // TODO: better errors i.e. UntrustedHost

        if res.base.version != signing_request.base.version
            || res.base.fire_and_forget
            || res.base.use_encryption != signing_request.base.use_encryption
            || res.base.pre_shared_key != signing_request.base.pre_shared_key
            || res.base.psk_id != signing_request.base.psk_id
            || res.base.psk_salt != signing_request.base.psk_salt
        {
            return Err(PlabbleProtocolError::UnexpectedResponse);
        }

        if let Some(response_settings) = res.base.crypto_settings
            && response_settings != settings
        {
            return Err(PlabbleProtocolError::UnexpectedResponse);
        }

        let (with_psk, with_salt) = match res.header.packet_type {
            ResponsePacketType::Session {
                with_psk,
                with_salt,
            } => (with_psk, with_salt),
            _ => return Err(PlabbleProtocolError::UnexpectedResponse),
        };

        let body = match res.body {
            PlabbleResponseBody::Session(body) => body,
            _ => return Err(PlabbleProtocolError::UnexpectedResponse),
        };

        if with_psk != body.psk_id.is_some()
            || with_psk != options.stored_key_lifetime.is_some()
            || with_salt != body.salt.is_some()
            || with_salt != options.server_salt
            || body.keys.len() != key_exchanges.len()
        {
            return Err(PlabbleProtocolError::UnexpectedResponse);
        }

        let signature_algorithms = get_signature_algorithms(&settings);
        if body.signatures.len() != signature_algorithms.len()
            || body
                .signatures
                .iter()
                .zip(&signature_algorithms)
                .any(|(signature, algorithm)| signature.get_algorithm() != *algorithm)
        {
            return Err(PlabbleProtocolError::FailedToProcessResponse);
        }

        let signed_bytes = body.signing_bytes(&signing_request, &settings)?;

        let context = self.config.data.as_ref().unwrap();
        if !context.verification_keys.is_empty() {
            for (algorithm, signature) in signature_algorithms.iter().zip(&body.signatures) {
                let key = context
                    .verification_keys
                    .iter()
                    .find(|key| key.get_algorithm() == *algorithm)
                    .ok_or(PlabbleProtocolError::FailedToProcessResponse)?;
                if key.verify(&signed_bytes, signature) != Some(true) {
                    return Err(PlabbleProtocolError::FailedToProcessResponse);
                }
            }
        }

        let shared_secrets = body
            .keys
            .iter()
            .zip(&key_exchanges)
            .map(|(key, exchange)| {
                exchange
                    .process_response(key)
                    .ok_or(PlabbleProtocolError::FailedToProcessResponse)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let session_key = crate::packets::context::PlabbleConnectionContext::derive_session_key(
            settings.use_blake3,
            client_salt,
            body.salt,
            &shared_secrets,
        );

        let context = self.config.data.as_mut().unwrap();
        context.crypto_settings = Some(settings);
        context.activate_session(session_key, options.enable_full_encryption);

        if let Some(psk_id) = body.psk_id
            && let Some(provider) = &context.key_provider
        {
            provider.store_psk(
                psk_id,
                session_key,
                psk_expiration.as_ref().map(PlabbleDateTime::timestamp),
            );
        }

        Ok(body.psk_id)
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use crate::{
        packets::body::error::PlabbleError,
        protocol::{
            PlabbleConnection, client::options::SessionOptions, error::PlabbleProtocolError,
        },
    };

    #[test]
    fn start_session_rejects_unknown_algorithm_before_sending() {
        block_on(async {
            let (outgoing, wire) = async_channel::unbounded();
            let (_incoming, receiver) = async_channel::unbounded();
            let mut connection = PlabbleConnection::new(outgoing, receiver);
            let options = SessionOptions {
                algorithms: vec!["future-kem".into()],
                ..Default::default()
            };

            assert_eq!(
                connection.start_session(Some(options)).await,
                Err(PlabbleProtocolError::ProtocolError(
                    PlabbleError::UnsupportedAlgorithm {
                        name: "future-kem".into()
                    }
                ))
            );
            assert!(wire.is_empty());
        });
    }
}
