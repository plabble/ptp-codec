use binary_codec::{BinaryDeserializer, BinarySerializer};

use crate::{
    packets::{request::PlabbleRequestPacket, response::PlabbleResponsePacket},
    protocol::{PlabbleConnection, error::PlabbleProtocolError},
};

pub mod node;

/// Server-side implementation of [`PlabbleConnection`].
impl PlabbleConnection {
    /// Sends a response packet
    ///
    /// If the packet is not fire-and-forget, the internal counter will be incremented.
    pub async fn send_response(
        &mut self,
        packet: PlabbleResponsePacket,
    ) -> Result<(), PlabbleProtocolError> {
        let bytes = packet.to_bytes(Some(&mut self.config))?;
        self.tx
            .send(bytes)
            .await
            .map_err(|_| PlabbleProtocolError::SenderError)?;
        self.config.reset();
        if !packet.base.fire_and_forget {
            self.config.data.as_mut().unwrap().increment(false);
        }
        Ok(())
    }

    /// Receives and processes the next request packet.
    ///
    /// If the packet is not fire-and-forget, the internal counter is incremented
    pub async fn recv_request(&mut self) -> Result<PlabbleRequestPacket, PlabbleProtocolError> {
        let bytes = self
            .rx
            .recv()
            .await
            .map_err(|_| PlabbleProtocolError::ReceiverError)?;

        let packet = PlabbleRequestPacket::from_bytes(&bytes, Some(&mut self.config))?;
        self.config.reset();
        if !packet.base.fire_and_forget {
            self.config.data.as_mut().unwrap().increment(true);
        }

        Ok(packet)
    }
}
