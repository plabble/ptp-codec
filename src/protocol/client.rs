use std::collections::HashMap;

use async_channel::{Receiver, Sender};
use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

use crate::{
    packets::{
        context::PlabbleConnectionContext, request::PlabbleRequestPacket,
        response::PlabbleResponsePacket,
    },
    protocol::{PlabbleConnection, error::PlabbleProtocolError},
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
}
