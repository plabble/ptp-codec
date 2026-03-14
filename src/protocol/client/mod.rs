use binary_codec::{BinaryDeserializer, BinarySerializer};

use crate::{
    packets::{request::PlabbleRequestPacket, response::PlabbleResponsePacket},
    protocol::{PlabbleConnection, error::PlabbleProtocolError},
};

pub mod implementation;
pub mod options;

/// Client-side implementation of [`PlabbleConnection`].
impl PlabbleConnection {
    /// Sends a request packet without waiting for a response.
    ///
    /// If the packet is not fire-and-forget, the internal counter will be incremented.
    pub async fn send_request(
        &mut self,
        packet: PlabbleRequestPacket,
    ) -> Result<(), PlabbleProtocolError> {
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
    pub async fn send_and_recv(
        &mut self,
        packet: PlabbleRequestPacket,
    ) -> Result<PlabbleResponsePacket, PlabbleProtocolError> {
        let counter = self.config.data.as_ref().unwrap().client_counter;
        let (tx, rx) = async_channel::bounded(1);
        self.hooks.insert(counter, tx);
        self.send_request(packet).await?;
        rx.recv()
            .await
            .map_err(|_| PlabbleProtocolError::ReceiverError)
    }

    /// Receives and processes the next response packet.
    ///
    /// If the packet is not fire-and-forget, the internal counter is incremented
    /// and any registered hook for the matching request counter is notified.
    pub async fn recv_response(&mut self) -> Result<PlabbleResponsePacket, PlabbleProtocolError> {
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
                    hook.try_send(packet.clone())
                        .map_err(|_| PlabbleProtocolError::SenderError)?;
                }
            }
        }

        Ok(packet)
    }
}
