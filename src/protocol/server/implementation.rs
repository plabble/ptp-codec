use crate::{
    crypto::KeyExchange,
    packets::{
        base::PlabblePacketBase,
        body::{
            error::PlabbleError, request_body::PlabbleRequestBody,
            response_body::PlabbleResponseBody, session::SessionResponseBody,
        },
        context::PlabbleConnectionContext,
        header::{
            response_header::PlabbleResponseHeader,
            type_and_flags::{RequestPacketType, ResponsePacketType},
        },
        request::PlabbleRequestPacket,
        response::PlabbleResponsePacket,
    },
    protocol::{
        PlabbleConnection,
        error::PlabbleProtocolError,
        options::{get_key_exchange_algorithms, get_signature_algorithms, unsupported_algorithm},
    },
};

impl PlabbleConnection {
    /// Handle Plabble request and produce a Plabble response
    ///
    /// The session handler validates its flags, algorithms, key order and signing configuration.
    /// Other high-level packet handlers are not implemented yet.
    pub fn handle_request(
        &mut self,
        req: PlabbleRequestPacket,
    ) -> Result<PlabbleResponsePacket, PlabbleProtocolError> {
        match req.header.packet_type.clone() {
            RequestPacketType::Session {
                persist_key,
                enable_encryption,
                with_salt,
                request_salt,
            } => {
                // Increment request counter, also if an error occurs
                let context = self.config.data.as_ref().unwrap();
                let counter = context
                    .client_counter
                    .checked_sub(1)
                    .ok_or(PlabbleProtocolError::UnexpectedRequest)?;

                // Get settings from the request or fall back to context session (this will only be if a session was already created). Else, use default.
                let settings = req
                    .base
                    .crypto_settings
                    .or(context.crypto_settings)
                    .unwrap_or_default();

                // Disallow Fire-and-Forget requests for SESSION packets
                if req.base.fire_and_forget {
                    return Err(PlabbleError::InvalidRequest.into());
                }

                // If any of the requested algorithms are unsupported, return an error
                if let Some(name) = unsupported_algorithm(&settings) {
                    return Err(PlabbleError::UnsupportedAlgorithm {
                        name: name.to_owned(),
                    }
                    .into());
                }

                // If full packet encryption is requested but no encryption algorithm is chosen, return error
                if enable_encryption && !settings.encrypt_with_chacha && !settings.encrypt_with_aes
                {
                    return Err(PlabbleError::UnsupportedAlgorithm {
                        name: "full packet encryption requires ChaCha and/or AES".to_owned(),
                    }
                    .into());
                }

                // Check if body is a SESSION request and extract it
                let body = match &req.body {
                    PlabbleRequestBody::Session(body) => body,
                    _ => return Err(PlabbleProtocolError::UnexpectedRequest),
                };

                // Validate that the presence of PSK and salt in the request matches the expected flags
                if persist_key != body.psk_expiration.is_some() || with_salt != body.salt.is_some()
                {
                    return Err(PlabbleError::InvalidRequest.into());
                }

                // Get the list of key exchange algorithms selected by the current crypto settings
                let exchange_algorithms = get_key_exchange_algorithms(&settings);
                if exchange_algorithms.is_empty() || body.keys.len() != exchange_algorithms.len() {
                    return Err(PlabbleError::InvalidRequest.into());
                }

                // Get the list of signature algorithms selected by the current crypto settings and ensure we have the corresponding signing keys
                let signature_algorithms = get_signature_algorithms(&settings);
                for algorithm in &signature_algorithms {
                    if !context
                        .signing_keys
                        .iter()
                        .any(|key| key.get_algorithm() == *algorithm)
                    {
                        return Err(PlabbleError::UnsupportedAlgorithm {
                            name: format!("{algorithm:?}"),
                        }
                        .into());
                    }
                }

                // If any key should be persisted, ensure we have a key provider available (server error)
                let key_provider = context.key_provider.clone();
                if persist_key && key_provider.is_none() {
                    return Err(PlabbleError::InvalidRequest.into());
                }

                // Initialize key exchanges for each selected key exchange algorithm
                let mut key_exchanges: Vec<KeyExchange> = exchange_algorithms
                    .into_iter()
                    .map(KeyExchange::new)
                    .collect();

                // Process each key exchange request and generate the corresponding response
                let exchanges = body
                    .keys
                    .iter()
                    .zip(&mut key_exchanges)
                    .map(|(key, exchange)| {
                        exchange
                            .process_request(key)
                            .ok_or(PlabbleProtocolError::FailedToProcessRequest)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                // If server salt is requested, generate a random value
                let server_salt = request_salt.then(rand::random);
                // Collect the shared secrets from each key exchange
                let shared_secrets: Vec<_> = exchanges.iter().map(|(secret, _)| *secret).collect();
                
                // Generate the session key from the collected shared secrets and salts
                let session_key = PlabbleConnectionContext::derive_session_key(
                    settings.use_blake3,
                    body.salt,
                    server_salt,
                    &shared_secrets,
                );

                // If the session key should be persisted, generate a random PSK ID
                let psk_id = persist_key.then(rand::random);

                let mut response_body = SessionResponseBody {
                    psk_id,
                    salt: server_salt,
                    keys: exchanges
                        .into_iter()
                        .map(|(_, response)| response)
                        .collect(),
                    signatures: Vec::new(),
                };

                // Sign the response and request for authenticity and integrity
                let signed_bytes = response_body.signing_bytes(&req, &settings)?;

                // Generate the signatures for the response using the available signing keys and algorithms
                response_body.signatures = signature_algorithms
                    .iter()
                    .map(|algorithm| {
                        context
                            .signing_keys
                            .iter()
                            .find(|key| key.get_algorithm() == *algorithm)
                            .and_then(|key| key.sign(&signed_bytes))
                            .ok_or(PlabbleProtocolError::FailedToProcessRequest)
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                // Store the updated crypto settings and pending session information in the server context
                let context = self.config.data.as_mut().unwrap();
                context.crypto_settings = Some(settings);
                context.pending_session_key = Some(session_key);
                context.pending_full_encryption = Some(enable_encryption);

                // If a PSK ID was generated, store the PSK in the key provider
                if let Some(psk_id) = psk_id {
                    key_provider.unwrap().store_psk(
                        psk_id,
                        session_key,
                        body.psk_expiration.as_ref().map(|date| date.timestamp()),
                    );
                }

                let response_base = PlabblePacketBase {
                    version: req.base.version,
                    pre_shared_key: req.base.pre_shared_key,
                    use_encryption: req.base.use_encryption,
                    psk_id: req.base.psk_id,
                    psk_salt: req.base.psk_salt,
                    ..Default::default()
                };

                Ok(PlabbleResponsePacket {
                    base: response_base,
                    header: PlabbleResponseHeader::new(
                        ResponsePacketType::Session {
                            with_psk: psk_id.is_some(),
                            with_salt: server_salt.is_some(),
                        },
                        Some(counter),
                    ),
                    body: PlabbleResponseBody::Session(response_body),
                })
            }
            // The high-level dispatcher for other packet types is intentionally
            // not implemented yet. Return a protocol error instead of panicking.
            _ => Err(PlabbleProtocolError::UnexpectedRequest),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        crypto::{
            KeyExchange, KeyExchangeAlgorithm,
            algorithm::{SigningKey, VerificationKey},
        },
        packets::{
            base::PlabblePacketBase,
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
        protocol::{PlabbleConnection, error::PlabbleProtocolError},
    };

    fn request(keys: bool, persist_key: bool) -> PlabbleRequestPacket {
        let keys = if keys {
            let mut exchange = KeyExchange::new(KeyExchangeAlgorithm::X25519);
            vec![exchange.make_request().unwrap()]
        } else {
            Vec::new()
        };
        PlabbleRequestPacket {
            base: PlabblePacketBase::default(),
            header: PlabbleRequestHeader::new(
                RequestPacketType::Session {
                    persist_key,
                    enable_encryption: true,
                    with_salt: false,
                    request_salt: true,
                },
                None,
            ),
            body: PlabbleRequestBody::Session(SessionRequestBody {
                psk_expiration: persist_key.then(|| crate::core::PlabbleDateTime::new(10)),
                salt: None,
                keys,
            }),
        }
    }

    fn connection() -> PlabbleConnection {
        let (tx, _outgoing) = async_channel::unbounded();
        let (_incoming, rx) = async_channel::unbounded();
        let mut connection = PlabbleConnection::new(tx, rx);
        let context = connection.config.data.as_mut().unwrap();
        context.client_counter = 1;
        context.signing_keys = vec![SigningKey::Ed25519([11; 32])];
        connection
    }

    #[test]
    fn session_handler_derives_pending_key_and_signs_response() {
        let mut connection = connection();
        let request = request(true, false);
        let response = connection.handle_request(request.clone()).unwrap();
        assert!(matches!(
            response.header.packet_type,
            ResponsePacketType::Session {
                with_psk: false,
                with_salt: true
            }
        ));
        assert_eq!(response.base.version, request.base.version);
        assert!(!response.base.fire_and_forget);
        assert!(!response.base.specify_crypto_settings);
        assert_eq!(response.base.crypto_settings, None);
        assert_eq!(response.base.pre_shared_key, request.base.pre_shared_key);
        assert_eq!(response.base.psk_id, request.base.psk_id);
        assert_eq!(response.base.psk_salt, request.base.psk_salt);

        let body = match &response.body {
            PlabbleResponseBody::Session(body) => body,
            _ => panic!("expected session response"),
        };
        assert!(body.salt.is_some());
        assert_eq!(body.keys.len(), 1);
        assert_eq!(body.signatures.len(), 1);

        let signing_bytes = body.signing_bytes(&request, &Default::default()).unwrap();
        let verification_key = ed25519_dalek::SigningKey::from_bytes(&[11; 32])
            .verifying_key()
            .to_bytes();
        assert_eq!(
            VerificationKey::Ed25519(verification_key).verify(&signing_bytes, &body.signatures[0]),
            Some(true)
        );

        let context = connection.config.data.as_ref().unwrap();
        assert!(context.pending_session_key.is_some());
        assert_eq!(context.session_key, None);
    }

    #[test]
    fn session_handler_rejects_missing_exchange_key() {
        let mut connection = connection();
        assert_eq!(
            connection.handle_request(request(false, false)),
            Err(PlabbleProtocolError::ProtocolError(
                PlabbleError::InvalidRequest
            ))
        );
        assert!(
            connection
                .config
                .data
                .as_ref()
                .unwrap()
                .pending_session_key
                .is_none()
        );
    }

    #[test]
    fn session_handler_requires_storage_when_psk_is_requested() {
        let mut connection = connection();
        assert_eq!(
            connection.handle_request(request(true, true)),
            Err(PlabbleProtocolError::ProtocolError(
                PlabbleError::InvalidRequest
            ))
        );
    }
}
