use crate::{
    crypto::KeyExchange,
    packets::{
        body::{request_body::PlabbleRequestBody, response_body::PlabbleResponseBody, session::SessionResponseBody},
        header::{
            response_header::PlabbleResponseHeader,
            type_and_flags::{RequestPacketType, ResponsePacketType},
        },
        request::PlabbleRequestPacket,
        response::PlabbleResponsePacket,
    },
    protocol::{
        PlabbleConnection, client::options::get_key_exchange_algorithms,
        error::PlabbleProtocolError,
    },
};

impl PlabbleConnection {
    /// Handle Plabble request and produce a Plabble response
    ///
    /// This method assumes that basic checks (Plabble version, supported crypto algorithms, request not malformed) are already performed
    pub fn handle_request(
        &mut self,
        req: PlabbleRequestPacket,
    ) -> Result<PlabbleResponsePacket, PlabbleProtocolError> {
        match req.header.packet_type {
            RequestPacketType::Certificate {
                full_chain,
                full_certs,
                challenge,
                query_mode,
            } => todo!(),
            RequestPacketType::Session {
                persist_key,
                enable_encryption,
                with_salt: _,
                request_salt,
            } => {
                let context = self.config.data.as_mut().unwrap();
                let counter = context.client_counter - 1;

                if let PlabbleRequestBody::Session(body) = req.body {
                    let settings = req.base.crypto_settings.unwrap_or_default();

                    let mut key_exchanges: Vec<KeyExchange> =
                        get_key_exchange_algorithms(&settings)
                            .into_iter()
                            .map(|alg| KeyExchange::new(alg))
                            .collect();

                    // TODO: collect also signature algorithms and make helper method to sign request for each algorithm

                    let shared_secrets = body
                        .keys
                        .into_iter()
                        .enumerate()
                        .map(|(idx, key)| {
                            key_exchanges
                                .get_mut(idx)
                                .and_then(|kx| kx.process_request(&key))
                                .ok_or(PlabbleProtocolError::FailedToProcessRequest)
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    let server_salt = if request_salt {
                        Some(rand::random())
                    } else {
                        None
                    };

                    context.create_session_key(
                        settings.use_blake3,
                        body.salt,
                        server_salt.clone(),
                        shared_secrets.iter().map(|s| s.0).collect(),
                    );

                    let mut psk_id = None;

                    if persist_key {
                        if let Some(provider) = &context.key_provider {
                            let psk = rand::random();
                            psk_id = Some(psk);
                            provider.store_psk(
                                psk,
                                context.session_key.clone().unwrap(),
                                body.psk_expiration.map(|d| d.timestamp()),
                            );
                        }
                    }

                    return Ok(PlabbleResponsePacket {
                        base: req.base,
                        header: PlabbleResponseHeader::new(
                            ResponsePacketType::Session {
                                with_psk: psk_id.is_some(),
                                with_salt: request_salt,
                            },
                            Some(counter)
                        ),
                        body: PlabbleResponseBody::Session(SessionResponseBody {
                            psk_id,
                            salt: server_salt,
                            keys: shared_secrets.into_iter().map(|s|s.1).collect(),
                            signatures: vec![]
                        })
                    });
                }

                Err(PlabbleProtocolError::UnexpectedRequest)
            }
            RequestPacketType::Get {
                binary_keys,
                subscribe,
                range_mode_until,
                with_limit,
            } => todo!(),
            RequestPacketType::Stream {
                binary_keys,
                subscribe,
                range_mode_until,
                write_mode,
            } => todo!(),
            RequestPacketType::Post {
                binary_keys,
                subscribe,
                range_mode_until,
                do_not_persist,
            } => todo!(),
            RequestPacketType::Patch {
                update_permissions,
                add_to_acl,
                remove_from_acl,
            } => todo!(),
            RequestPacketType::Put {
                binary_keys,
                subscribe,
                assert_keys,
                append,
            } => todo!(),
            RequestPacketType::Delete {
                binary_keys,
                range_mode_until,
                with_limit,
                return_deleted,
            } => todo!(),
            RequestPacketType::Subscribe {
                binary_keys,
                range_mode_until,
                unsubscribe,
            } => todo!(),
            RequestPacketType::Whisper { whisper_type } => todo!(),
            RequestPacketType::Register => todo!(),
            RequestPacketType::Identify => todo!(),
            RequestPacketType::Proxy {
                init_session,
                keep_connection,
                select_random_hops,
            } => todo!(),
            RequestPacketType::Opcode {
                allow_bucket_operations,
                allow_eval,
            } => todo!(),
            RequestPacketType::Custom {
                flag1,
                flag2,
                flag3,
                flag4,
            } => todo!(),
        }
    }
}
