use binary_codec::{BinaryDeserializer, BinarySerializer};

use crate::{
    packets::{
        header::type_and_flags::ResponsePacketType, request::PlabbleRequestPacket,
        response::PlabbleResponsePacket,
    },
    protocol::{PlabbleConnection, error::PlabbleProtocolError},
};

#[cfg(feature = "implementation")]
pub mod implementation;
#[cfg(feature = "implementation")]
pub mod options;

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
        let completes_session = matches!(
            packet.header.packet_type,
            ResponsePacketType::Session { .. }
        );

        let bytes = packet.to_bytes(Some(&mut self.config))?;
        self.tx
            .send(bytes)
            .await
            .map_err(|_| PlabbleProtocolError::SenderError)?;

        self.config.reset();

        if !packet.base.fire_and_forget {
            self.config.data.as_mut().unwrap().increment(false);
        }

        if completes_session {
            self.config
                .data
                .as_mut()
                .unwrap()
                .activate_pending_session();
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
        protocol::PlabbleConnection,
    };

    fn session_request() -> PlabbleRequestPacket {
        PlabbleRequestPacket {
            base: PlabblePacketBase::default(),
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

    fn session_response() -> PlabbleResponsePacket {
        PlabbleResponsePacket {
            base: PlabblePacketBase::default(),
            header: PlabbleResponseHeader::new(
                ResponsePacketType::Session {
                    with_psk: false,
                    with_salt: false,
                },
                Some(0),
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
    fn recv_request_decodes_packet_and_updates_client_counter() {
        block_on(async {
            let (outgoing, _wire) = async_channel::unbounded();
            let (incoming, receiver) = async_channel::unbounded();
            let mut connection = PlabbleConnection::new(outgoing, receiver);
            let expected = session_request();
            incoming
                .send(expected.to_bytes(None).unwrap())
                .await
                .unwrap();

            assert_eq!(connection.recv_request().await.unwrap(), expected);
            assert_eq!(connection.config.data.as_ref().unwrap().client_counter, 1);
        });
    }

    #[test]
    fn session_state_activates_only_after_response_is_sent() {
        block_on(async {
            let (outgoing, wire) = async_channel::unbounded();
            let (_incoming, receiver) = async_channel::unbounded();
            let mut connection = PlabbleConnection::new(outgoing, receiver);
            let context = connection.config.data.as_mut().unwrap();
            context.client_counter = 4;
            context.server_counter = 3;
            context.pending_session_key = Some([9; 64]);
            context.pending_full_encryption = Some(true);

            assert_eq!(context.session_key, None);
            connection.send_response(session_response()).await.unwrap();
            assert!(!wire.recv().await.unwrap().is_empty());

            let context = connection.config.data.as_ref().unwrap();
            assert_eq!(context.session_key, Some([9; 64]));
            assert!(context.full_encryption);
            assert_eq!((context.client_counter, context.server_counter), (0, 0));
        });
    }
}
