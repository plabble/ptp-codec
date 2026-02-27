use std::collections::HashMap;

use async_channel::{Receiver, Sender};
use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};
use chrono::{DateTime, Utc};

use crate::{
    core::PlabbleDateTime, crypto::{KeyExchange, KeyExchangeAlgorithm}, packets::{
        base::PlabblePacketBase, body::{request_body::PlabbleRequestBody, response_body::PlabbleResponseBody, session::SessionRequestBody}, context::PlabbleConnectionContext, header::{request_header::PlabbleRequestHeader, type_and_flags::{RequestPacketType, ResponsePacketType}}, request::PlabbleRequestPacket, response::PlabbleResponsePacket
    }, protocol::{PlabbleConnection, error::PlabbleProtocolError}
};

/**
 * Implemention of PlabbleConnection for client
 */
impl PlabbleConnection {
    /**
     * Create new PlabbleConnection to given binary sender and receiver
     */
    pub fn new(tx: Sender<Vec<u8>>, rx: Receiver<Vec<u8>>) -> Self {
        Self {
            config: SerializerConfig::new(Some(PlabbleConnectionContext::new())),
            tx,
            rx,
            hooks: HashMap::new(),
        }
    }

    /**
     * Send a request packet without waiting for response. 
     * If the packet is not fire-and-forget, the internal counter will be incremented.
     */
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

    /**
     * Send and wait for response with the same counter.
     */
    pub async fn send_and_recv(&mut self, packet: PlabbleRequestPacket) -> Result<PlabbleResponsePacket, PlabbleProtocolError> {
        let counter = self.config.data.as_ref().unwrap().client_counter;
        let (tx, rx) = async_channel::bounded(1);
        self.hooks.insert(counter, tx);
        self.send(packet).await?;
        rx.recv().await.map_err(|_| PlabbleProtocolError::ReceiverError)
    }

    /**
     * Receive a response packet.
     */
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

    pub async fn start_session(&mut self, enable_full_encryption: bool, psk_expiration: Option<DateTime<Utc>>) -> Result<Option<[u8; 12]>, PlabbleProtocolError> {
        // TODO: support multiple algorithms and keys
        let mut kx = KeyExchange::new(KeyExchangeAlgorithm::X25519);

        // TODO: support client salt and server salt
        let client_salt = None;
        let req = PlabbleRequestPacket {
            base: PlabblePacketBase::default(),
            header: PlabbleRequestHeader::new(RequestPacketType::Session { 
                persist_key: psk_expiration.is_some(), 
                enable_encryption: enable_full_encryption, 
                with_salt: false, 
                request_salt: false 
            }, None),
            body: PlabbleRequestBody::Session(SessionRequestBody {
                psk_expiration: psk_expiration.map(|d|PlabbleDateTime(d)),
                salt: None,
                keys: vec![kx.make_request().ok_or(PlabbleProtocolError::SenderError)?]
            })
        };

        let res = self.send_and_recv(req).await?;
        if let ResponsePacketType::Session { with_psk, .. } = res.header.packet_type {
            if let PlabbleResponseBody::Session(body) = res.body {
                // TODO: multiple algorithms and keys
                let key = body.keys.first().ok_or(PlabbleProtocolError::UnexpectedResponse)?;
                let ss = kx.process_response(&key).ok_or(PlabbleProtocolError::FailedToProcessResponse)?;

                let context = self.config.data.as_mut().unwrap();
                // TODO: support blake3
                context.create_session_key(false, client_salt, body.salt, vec![ss]);

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
