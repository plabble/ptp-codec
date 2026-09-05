use binary_codec::{BinaryDeserializer, BinarySerializer};

use crate::{
    packets::{request::PlabbleRequestPacket, response::PlabbleResponsePacket},
    protocol::{PlabbleConnection, error::PlabbleProtocolError},
};

#[cfg(feature = "implementation")]
pub mod implementation;
#[cfg(feature = "implementation")]
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
        if packet.base.fire_and_forget {
            return Err(PlabbleProtocolError::UnexpectedRequest);
        }
        let counter = self.config.data.as_ref().unwrap().client_counter;
        self.send_request(packet).await?;
        let response = self.recv_response().await?;
        if response.header.request_counter != Some(counter) {
            return Err(PlabbleProtocolError::UnexpectedResponse);
        }
        Ok(response)
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
            if let Some(hook) = self.hooks.remove(&counter)
                && !hook.is_closed()
            {
                hook.try_send(packet.clone())
                    .map_err(|_| PlabbleProtocolError::SenderError)?;
            }
        }

        Ok(packet)
    }
}

#[cfg(test)]
mod tests {
    use binary_codec::BinarySerializer;
    use futures::executor::block_on;

    use crate::{
        crypto::algorithm::{CryptoSignature, KeyExhangeRequest, KeyExhangeResponse},
        packets::{
            base::PlabblePacketBase,
            body::{
                request_body::PlabbleRequestBody,
                response_body::PlabbleResponseBody,
                session::{SessionRequestBody, SessionResponseBody},
            },
            header::{
                request_header::PlabbleRequestHeader,
                response_header::PlabbleResponseHeader,
                type_and_flags::{RequestPacketType, ResponsePacketType},
            },
            request::PlabbleRequestPacket,
            response::PlabbleResponsePacket,
        },
        protocol::{PlabbleConnection, error::PlabbleProtocolError},
    };

    fn session_request(fire_and_forget: bool) -> PlabbleRequestPacket {
        PlabbleRequestPacket {
            base: PlabblePacketBase {
                fire_and_forget,
                ..Default::default()
            },
            header: PlabbleRequestHeader::new(
                RequestPacketType::Session {
                    persist_key: false,
                    enable_encryption: false,
                    with_salt: false,
                    request_salt: false,
                },
                None,
            ),
            body: PlabbleRequestBody::Session(SessionRequestBody {
                psk_expiration: None,
                salt: None,
                keys: vec![KeyExhangeRequest::X25519([1; 32])],
            }),
        }
    }

    fn session_response(counter: u16) -> PlabbleResponsePacket {
        PlabbleResponsePacket {
            base: PlabblePacketBase::default(),
            header: PlabbleResponseHeader::new(
                ResponsePacketType::Session {
                    with_psk: false,
                    with_salt: false,
                },
                Some(counter),
            ),
            body: PlabbleResponseBody::Session(SessionResponseBody {
                psk_id: None,
                salt: None,
                keys: vec![KeyExhangeResponse::X25519([2; 32])],
                signatures: vec![CryptoSignature::Ed25519([3; 64])],
            }),
        }
    }

    #[test]
    fn send_request_updates_counter_except_for_fire_and_forget() {
        block_on(async {
            let (outgoing, wire) = async_channel::unbounded();
            let (_incoming, receiver) = async_channel::unbounded();
            let mut connection = PlabbleConnection::new(outgoing, receiver);

            connection
                .send_request(session_request(false))
                .await
                .unwrap();
            assert!(!wire.recv().await.unwrap().is_empty());
            assert_eq!(connection.config.data.as_ref().unwrap().client_counter, 1);

            connection
                .send_request(session_request(true))
                .await
                .unwrap();
            assert!(!wire.recv().await.unwrap().is_empty());
            assert_eq!(connection.config.data.as_ref().unwrap().client_counter, 1);
        });
    }

    #[test]
    fn send_and_recv_receives_directly_and_checks_the_counter() {
        block_on(async {
            let (outgoing, wire) = async_channel::unbounded();
            let (incoming, receiver) = async_channel::unbounded();
            let mut connection = PlabbleConnection::new(outgoing, receiver);
            incoming
                .send(session_response(0).to_bytes(None).unwrap())
                .await
                .unwrap();

            let response = connection
                .send_and_recv(session_request(false))
                .await
                .unwrap();
            assert_eq!(response.header.request_counter, Some(0));
            assert!(!wire.recv().await.unwrap().is_empty());
            assert_eq!(connection.config.data.as_ref().unwrap().server_counter, 1);

            incoming
                .send(session_response(99).to_bytes(None).unwrap())
                .await
                .unwrap();
            assert_eq!(
                connection.send_and_recv(session_request(false)).await,
                Err(PlabbleProtocolError::UnexpectedResponse)
            );
        });
    }

    #[test]
    fn send_and_recv_rejects_fire_and_forget_without_sending() {
        block_on(async {
            let (outgoing, wire) = async_channel::unbounded();
            let (_incoming, receiver) = async_channel::unbounded();
            let mut connection = PlabbleConnection::new(outgoing, receiver);

            assert_eq!(
                connection.send_and_recv(session_request(true)).await,
                Err(PlabbleProtocolError::UnexpectedRequest)
            );
            assert!(wire.is_empty());
        });
    }
}
