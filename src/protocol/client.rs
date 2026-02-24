use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

use crate::{packets::{context::PlabbleConnectionContext, request::PlabbleRequestPacket, response::PlabbleResponsePacket}, protocol::{PlabbleConnection, error::PlabbleProtocolError}};


impl PlabbleConnection {
    pub fn new(endpoint: &str) -> Self {
        let (req_sender, req_receiver) = async_channel::unbounded();
        let (res_sender, res_receiver) = async_channel::unbounded();
        Self {
            config: SerializerConfig::new(Some(PlabbleConnectionContext::new())),
            req_sender,
            req_receiver,
            res_sender,
            res_receiver,
        }
    }

    pub fn send_request(&mut self, packet: PlabbleRequestPacket) -> Result<(), PlabbleProtocolError> {
        let bytes = packet.to_bytes(Some(&mut self.config))?;
        self.config.reset();
        Ok(())
    }

    pub fn handle_response(&mut self, bytes: &[u8]) -> Result<(), PlabbleProtocolError> {
        let packet = PlabbleResponsePacket::from_bytes(bytes, Some(&mut self.config))?;
        self.config.reset();
        Ok(())
    }

    // TODO: start session
    
}