use std::collections::HashMap;

use async_channel::{Receiver, Sender};
use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

use crate::{
    core::PlabbleDateTime, crypto::KeyExchange, packets::{
        base::{PlabblePacketBase, settings::CryptoSettings}, body::{request_body::PlabbleRequestBody, response_body::PlabbleResponseBody, session::SessionRequestBody}, context::PlabbleConnectionContext, header::{request_header::PlabbleRequestHeader, type_and_flags::{RequestPacketType, ResponsePacketType}}, request::PlabbleRequestPacket, response::PlabbleResponsePacket
    }, protocol::{PlabbleConnection, client::options::{SessionOptions, get_key_exchange_algorithms, set_crypto_settings}, error::PlabbleProtocolError}
};

pub mod options;

/// Client-side implementation of [`PlabbleConnection`].
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

    /// Sends a request packet without waiting for a response.
    ///
    /// If the packet is not fire-and-forget, the internal counter will be incremented.
    pub async fn send(&mut self, packet: PlabbleRequestPacket) -> Result<(), PlabbleProtocolError> {
        let bytes = packet.to_bytes(Some(&mut self.config))?;
        self.tx
            .send(bytes)
            .await
            .map_err(|_| PlabbleProtocolError::SenderError)?;
        self.config.reset();
        if !packet.base.fire_and_forget {
            self.config.data.as_mut().unwrap().increment(true);
        }
        Ok(())
    }

    /// Sends a request packet and waits for a response with the matching counter.
    pub async fn send_and_recv(&mut self, packet: PlabbleRequestPacket) -> Result<PlabbleResponsePacket, PlabbleProtocolError> {
        let counter = self.config.data.as_ref().unwrap().client_counter;
        let (tx, rx) = async_channel::bounded(1);
        self.hooks.insert(counter, tx);
        self.send(packet).await?;
        rx.recv().await.map_err(|_| PlabbleProtocolError::ReceiverError)
    }

    /// Receives and processes the next response packet.
    ///
    /// If the packet is not fire-and-forget, the internal counter is incremented
    /// and any registered hook for the matching request counter is notified.
    pub async fn recv(&mut self) -> Result<PlabbleResponsePacket, PlabbleProtocolError> {
        let bytes = self
            .rx
            .recv()
            .await
            .map_err(|_| PlabbleProtocolError::ReceiverError)?;

        let packet = PlabbleResponsePacket::from_bytes(&bytes, Some(&mut self.config))?;
        self.config.reset();
        if !packet.base.fire_and_forget {
            self.config.data.as_mut().unwrap().increment(false);
            let counter = packet.header.request_counter.expect("Expected counter");
            if let Some(hook) = self.hooks.get(&counter) {
                if hook.is_closed() || hook.is_full() {
                    self.hooks.remove(&counter);
                } else {
                    hook.try_send(packet.clone()).map_err(|_| PlabbleProtocolError::SenderError)?;
                }
            }
        }

        Ok(packet)
    }

    /// Start a new session with the given options. Returns the PSK ID if a pre-shared key is created.
    pub async fn start_session(&mut self, options: SessionOptions) -> Result<Option<[u8; 12]>, PlabbleProtocolError> {
        let mut settings = CryptoSettings::default();
        set_crypto_settings(&mut settings, options.algorithms);
        
        let mut key_exchanges: Vec<KeyExchange> = get_key_exchange_algorithms(&settings)
            .into_iter().map(|alg| KeyExchange::new(alg)).collect();
        
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

        let req = PlabbleRequestPacket {
            base,
            header: PlabbleRequestHeader::new(RequestPacketType::Session { 
                persist_key: options.stored_key_lifetime.is_some(), 
                enable_encryption: options.enable_full_encryption, 
                with_salt: options.client_salt, 
                request_salt: options.server_salt,
            }, None),
            body: PlabbleRequestBody::Session(SessionRequestBody {
                psk_expiration: options.stored_key_lifetime.map(|d| PlabbleDateTime::from_now(d)),
                salt: client_salt,
                keys: key_exchanges.iter_mut()
                    .map(|kx| kx.make_request().expect("Unsupported algorithm")).collect(),
            })
        };

        let res = self.send_and_recv(req).await?;
        if let ResponsePacketType::Session { with_psk, .. } = res.header.packet_type {
            if let PlabbleResponseBody::Session(body) = res.body {
                let shared_secrets = body.keys.into_iter().enumerate()
                    .map(|(idx, key)| 
                        key_exchanges.get(idx).and_then(|kx|kx.process_response(&key))
                    .ok_or(PlabbleProtocolError::FailedToProcessResponse))
                    .collect::<Result<Vec<_>, _>>()?;
                    
                let context = self.config.data.as_mut().unwrap();
                context.create_session_key(settings.use_blake3, client_salt, body.salt, shared_secrets);

                if with_psk {
                    let psk_id = body.psk_id.expect("Expected PSK ID");
                    // TODO: store PSK with ID and expiration
                    return Ok(Some(psk_id));
                } else {
                    return Ok(None);
                }
            }
        }

        Err(PlabbleProtocolError::UnexpectedResponse)
    }
}
