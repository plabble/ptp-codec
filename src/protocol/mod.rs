use std::collections::HashMap;

use async_channel::{Receiver, Sender};
use binary_codec::SerializerConfig;
use serde::{Deserialize, Serialize};

pub mod error;

#[cfg(feature = "implementation")]
pub mod options;

use crate::packets::{context::PlabbleConnectionContext, response::PlabbleResponsePacket};

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "client")]
pub mod client;

/// Plabble Connection
pub struct PlabbleConnection {
    pub hooks: HashMap<u16, Sender<PlabbleResponsePacket>>,
    pub config: SerializerConfig<PlabbleConnectionContext>,
    pub tx: Sender<Vec<u8>>,
    pub rx: Receiver<Vec<u8>>,
}

/// Implementation of common functionality for [`PlabbleConnection`].
impl PlabbleConnection {
    /// Creates a new [`PlabbleConnection`] with the given binary sender and receiver.
    pub fn new(tx: Sender<Vec<u8>>, rx: Receiver<Vec<u8>>) -> Self {
        Self {
            config: SerializerConfig::new(Some(PlabbleConnectionContext::new())),
            tx,
            rx,
            hooks: HashMap::new(),
        }
    }
}

// ── Helpers ─────────────────────────────────────────────────────────────────
use crate::protocol::error::PlabbleProtocolError;

/// Deserialize a packet from a JSON or TOML string (depending on enabled features).
pub fn deserialize_input<T: for<'a> Deserialize<'a>>(
    data: &str,
) -> Result<T, PlabbleProtocolError> {
    #[cfg(feature = "use-json")]
    {
        serde_json::from_str(data).map_err(|_| PlabbleProtocolError::InputParsingFailed)
    }

    #[cfg(all(feature = "use-toml", not(feature = "use-json")))]
    {
        toml::from_str(data).map_err(|_| PlabbleProtocolError::InputParsingFailed)
    }

    #[cfg(not(any(feature = "use-json", feature = "use-toml")))]
    Err(PlabbleProtocolError::InputParsingFailed)
}

/// Serialize an object to a JSON or TOML string (depending on enabled features).
pub fn serialize_output<T: Serialize>(data: &T) -> Result<String, PlabbleProtocolError> {
    #[cfg(feature = "use-json")]
    {
        serde_json::to_string(data).map_err(|_| PlabbleProtocolError::OutputSerializationFailed)
    }

    #[cfg(all(feature = "use-toml", not(feature = "use-json")))]
    {
        toml::to_string(data).map_err(|_| PlabbleProtocolError::OutputSerializationFailed)
    }

    #[cfg(not(any(feature = "use-json", feature = "use-toml")))]
    Err(PlabbleProtocolError::OutputSerializationFailed)
}

#[cfg(all(
    test,
    feature = "client",
    feature = "server",
    feature = "implementation"
))]
mod tests {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    use futures::executor::block_on;

    use crate::{
        crypto::algorithm::{SigningKey, VerificationKey},
        packets::{
            base::PlabblePacketBase,
            body::{
                custom::CustomBody, request_body::PlabbleRequestBody,
                response_body::PlabbleResponseBody,
            },
            header::{
                request_header::PlabbleRequestHeader,
                response_header::PlabbleResponseHeader,
                type_and_flags::{RequestPacketType, ResponsePacketType},
            },
            request::PlabbleRequestPacket,
            response::PlabbleResponsePacket,
        },
        protocol::{PlabbleConnection, client::options::SessionOptions},
        providers::KeyProvider,
    };

    type StoredPsk = ([u8; 64], Option<u32>);

    #[derive(Default)]
    struct MemoryKeyProvider {
        keys: Mutex<HashMap<[u8; 12], StoredPsk>>,
    }

    impl KeyProvider for MemoryKeyProvider {
        fn get_bucket_key(&self, _bucket_id: &[u8; 16]) -> Option<[u8; 32]> {
            None
        }

        fn get_psk(&self, psk_id: &[u8; 12]) -> Option<[u8; 64]> {
            self.keys.lock().unwrap().get(psk_id).map(|entry| entry.0)
        }

        fn store_psk(&self, psk_id: [u8; 12], psk: [u8; 64], expiration: Option<u32>) {
            self.keys.lock().unwrap().insert(psk_id, (psk, expiration));
        }
    }

    #[test]
    fn complete_signed_session_and_encrypted_packet_flow() {
        block_on(async {
            let (client_tx, server_rx) = async_channel::unbounded();
            let (server_tx, client_rx) = async_channel::unbounded();
            let mut client = PlabbleConnection::new(client_tx, client_rx);
            let mut server = PlabbleConnection::new(server_tx, server_rx);

            let signing_seed = [7u8; 32];
            let verification_key = ed25519_dalek::SigningKey::from_bytes(&signing_seed)
                .verifying_key()
                .to_bytes();
            let ed448_seed = [8u8; 57];
            let ed448_signing = ed448_goldilocks::SigningKey::try_from(&ed448_seed[..]).unwrap();
            let ed448_verification = ed448_signing.verifying_key().to_bytes();
            server.config.data.as_mut().unwrap().signing_keys = vec![
                SigningKey::Ed25519(signing_seed),
                SigningKey::Ed448(ed448_seed),
            ];
            client.config.data.as_mut().unwrap().verification_keys = vec![
                VerificationKey::Ed25519(verification_key),
                VerificationKey::Ed448(ed448_verification),
            ];

            let client_keys = Arc::new(MemoryKeyProvider::default());
            let server_keys = Arc::new(MemoryKeyProvider::default());
            client.config.data.as_mut().unwrap().key_provider = Some(client_keys.clone());
            server.config.data.as_mut().unwrap().key_provider = Some(server_keys.clone());

            let options = SessionOptions {
                enable_full_encryption: true,
                stored_key_lifetime: Some(3600),
                client_salt: true,
                server_salt: true,
                algorithms: vec!["ed448".into()],
                ..Default::default()
            };
            let (client_result, server_result) =
                futures::join!(client.start_session(Some(options)), async {
                    let result = async {
                        let request = server.recv_request().await?;
                        let response = server.handle_request(request)?;
                        server.send_response(response).await
                    }
                    .await;
                    if result.is_err() {
                        server.tx.close();
                    }
                    result
                });
            server_result.unwrap();
            let psk_id = client_result
                .unwrap()
                .expect("server should persist the key");

            let client_context = client.config.data.as_ref().unwrap();
            let server_context = server.config.data.as_ref().unwrap();
            assert_eq!(client_context.session_key, server_context.session_key);
            assert!(client_context.full_encryption && server_context.full_encryption);
            assert_eq!(
                (client_context.client_counter, client_context.server_counter),
                (0, 0)
            );
            assert_eq!(
                (server_context.client_counter, server_context.server_counter),
                (0, 0)
            );
            assert_eq!(client_keys.get_psk(&psk_id), client_context.session_key);
            assert_eq!(server_keys.get_psk(&psk_id), server_context.session_key);

            let request = PlabbleRequestPacket {
                base: PlabblePacketBase::default(),
                header: PlabbleRequestHeader::new(
                    RequestPacketType::Custom {
                        flag1: true,
                        flag2: false,
                        flag3: false,
                        flag4: false,
                    },
                    None,
                ),
                body: PlabbleRequestBody::Custom(CustomBody {
                    protocol: 42,
                    data: b"encrypted request".to_vec(),
                }),
            };
            client.send_request(request.clone()).await.unwrap();
            assert_eq!(server.recv_request().await.unwrap(), request);

            let response = PlabbleResponsePacket {
                base: PlabblePacketBase::default(),
                header: PlabbleResponseHeader::new(
                    ResponsePacketType::Custom {
                        flag1: true,
                        flag2: false,
                        flag3: false,
                        flag4: false,
                    },
                    Some(0),
                ),
                body: PlabbleResponseBody::Custom(CustomBody {
                    protocol: 42,
                    data: b"encrypted response".to_vec(),
                }),
            };
            server.send_response(response.clone()).await.unwrap();
            assert_eq!(client.recv_response().await.unwrap(), response);

            // A second handshake can use the persisted key while the old
            // session is fully encrypted. The new session replaces it only
            // after the response has crossed the wire.
            let rekey_options = SessionOptions {
                enable_full_encryption: true,
                psk_id: Some(psk_id),
                ..Default::default()
            };
            let (client_result, server_result) =
                futures::join!(client.start_session(Some(rekey_options)), async {
                    let request = server.recv_request().await?;
                    let response = server.handle_request(request)?;
                    assert!(!response.base.specify_crypto_settings);
                    assert_eq!(response.base.crypto_settings, None);
                    server.send_response(response).await
                });
            server_result.unwrap();
            assert_eq!(client_result.unwrap(), None);
            assert_eq!(
                client.config.data.as_ref().unwrap().session_key,
                server.config.data.as_ref().unwrap().session_key
            );
            assert_eq!(
                (
                    client.config.data.as_ref().unwrap().client_counter,
                    client.config.data.as_ref().unwrap().server_counter,
                ),
                (0, 0)
            );
        });
    }

    #[test]
    fn client_rejects_a_session_signed_by_an_untrusted_key() {
        block_on(async {
            let (client_tx, server_rx) = async_channel::unbounded();
            let (server_tx, client_rx) = async_channel::unbounded();
            let mut client = PlabbleConnection::new(client_tx, client_rx);
            let mut server = PlabbleConnection::new(server_tx, server_rx);

            server.config.data.as_mut().unwrap().signing_keys = vec![SigningKey::Ed25519([7; 32])];
            let wrong_key = ed25519_dalek::SigningKey::from_bytes(&[8; 32])
                .verifying_key()
                .to_bytes();
            client.config.data.as_mut().unwrap().verification_keys =
                vec![VerificationKey::Ed25519(wrong_key)];

            let (client_result, server_result) =
                futures::join!(client.start_session(None), async {
                    let request = server.recv_request().await?;
                    let response = server.handle_request(request)?;
                    server.send_response(response).await
                });
            server_result.unwrap();
            assert_eq!(
                client_result,
                Err(crate::protocol::error::PlabbleProtocolError::FailedToProcessResponse)
            );
            assert_eq!(client.config.data.as_ref().unwrap().session_key, None);
        });
    }

    #[cfg(all(feature = "pqc-lite", feature = "blake-3"))]
    #[test]
    fn complete_session_with_every_implemented_exchange_and_signature() {
        use ml_dsa::{Generate, Keypair, MlDsa44, MlDsa65, SigningKey as MlDsaSigningKey};

        block_on(async {
            let (client_tx, server_rx) = async_channel::unbounded();
            let (server_tx, client_rx) = async_channel::unbounded();
            let mut client = PlabbleConnection::new(client_tx, client_rx);
            let mut server = PlabbleConnection::new(server_tx, server_rx);

            let ed25519_seed = [21u8; 32];
            let ed25519_verification = ed25519_dalek::SigningKey::from_bytes(&ed25519_seed)
                .verifying_key()
                .to_bytes();
            let ed448_seed = [22u8; 57];
            let ed448_signing = ed448_goldilocks::SigningKey::try_from(&ed448_seed[..]).unwrap();
            let ed448_verification = ed448_signing.verifying_key().to_bytes();
            let dsa44 = MlDsaSigningKey::<MlDsa44>::generate();
            let dsa65 = MlDsaSigningKey::<MlDsa65>::generate();

            server.config.data.as_mut().unwrap().signing_keys = vec![
                SigningKey::Ed25519(ed25519_seed),
                SigningKey::Ed448(ed448_seed),
                SigningKey::Dsa44(dsa44.to_seed().into()),
                SigningKey::Dsa65(dsa65.to_seed().into()),
            ];
            client.config.data.as_mut().unwrap().verification_keys = vec![
                VerificationKey::Ed25519(ed25519_verification),
                VerificationKey::Ed448(ed448_verification),
                VerificationKey::Dsa44(dsa44.verifying_key().encode().into()),
                VerificationKey::Dsa65(dsa65.verifying_key().encode().into()),
            ];

            let options = SessionOptions {
                enable_full_encryption: true,
                algorithms: vec![
                    "aes256".into(),
                    "!chacha20".into(),
                    "blake3".into(),
                    "ed448".into(),
                    "mldsa44".into(),
                    "mldsa65".into(),
                    "mlkem512".into(),
                    "mlkem768".into(),
                ],
                ..Default::default()
            };

            let client_session = Box::pin(client.start_session(Some(options)));
            let server_receive = Box::pin(server.recv_request());
            let (request, client_session) =
                match futures::future::select(client_session, server_receive).await {
                    futures::future::Either::Left((result, _)) => {
                        panic!("client stopped before sending the session request: {result:?}")
                    }
                    futures::future::Either::Right((request, client_session)) => {
                        (request.unwrap(), client_session)
                    }
                };
            let response = server.handle_request(request).unwrap();
            server.send_response(response).await.unwrap();
            assert_eq!(client_session.await.unwrap(), None);
            assert_eq!(
                client.config.data.as_ref().unwrap().session_key,
                server.config.data.as_ref().unwrap().session_key
            );
            assert!(client.config.data.as_ref().unwrap().full_encryption);
            assert!(server.config.data.as_ref().unwrap().full_encryption);
        });
    }
}
