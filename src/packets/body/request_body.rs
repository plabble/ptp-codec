use binary_codec::{
    BinaryDeserializer, BinarySerializer, DeserializationError, SerializationError,
    SerializerConfig,
};
use serde::{Deserialize, Serialize};

use crate::packets::{
    body::{
        RequestSerializationContext,
        bucket::{BucketQuery, PutRequestBody},
        session::SessionRequestBody,
    },
    header::type_and_flags::RequestPacketType,
};

/// An enumeration representing the different types of request bodies
/// that can be sent in a Plabble request.
/// Each variant corresponds to a specific request type and may contain
/// associated data relevant to that request.
///
/// # Variants
/// - `Certificate`: Represents a certificate request body.
/// - `Session`: Represents a session request body.
/// - `Get`: Represents a get request body with a bucket query.
/// - `Stream`: Represents a stream request body.
/// - `Post`: Represents a post request body.
/// - `Patch`: Represents a patch request body.
/// - `Put`: Represents a put request body with a put request body.
/// - `Delete`: Represents a delete request body with a bucket query.
/// - `Subscribe`: Represents a subscribe request body with a bucket query.
/// - `Unsubscribe`: Represents an unsubscribe request body with a bucket query.
/// - `Register`: Represents a register request body.
/// - `Identify`: Represents an identify request body.
/// - `Proxy`: Represents a proxy request body.
/// - `Opcode`: Represents an opcode request body.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PlabbleRequestBody {
    Certificate,
    Session(SessionRequestBody),
    Get(BucketQuery),
    Stream,
    Post,
    Patch,
    Put(PutRequestBody),
    Delete(BucketQuery),
    Subscribe(BucketQuery),
    Unsubscribe(BucketQuery),
    Register,
    Identify,
    Proxy,
    Opcode,
}

impl<'a> BinarySerializer<RequestSerializationContext<'a>> for PlabbleRequestBody {
    fn serialize(
        &self,
        config: Option<&mut SerializerConfig<RequestSerializationContext<'a>>>,
    ) -> Result<Vec<u8>, SerializationError> {
        let mut bytes = Vec::new();
        Self::write_bytes(self, &mut bytes, config)?;
        Ok(bytes);
    }

    fn write_bytes(
        &self,
        buffer: &mut Vec<u8>,
        config: Option<&mut SerializerConfig<RequestSerializationContext<'a>>>,
    ) -> Result<(), SerializationError> {
        let context = config.as_ref().unwrap().data.as_ref().unwrap();
        match self {
            PlabbleRequestBody::Certificate => todo!(),
            PlabbleRequestBody::Session(body) => body.serialize(config),
            PlabbleRequestBody::Get(body) => {
                if let RequestPacketType::Get { binary_keys, .. } = context.header.packet_type {
                    context
                        .config
                        .set_variant("bucket_type", if binary_keys { 1 } else { 0 });
                    body.serialize(Some(&mut context.config))
                } else {
                    return Err(SerializationError::InvalidData(format!(
                        "Header type {:?} did not match body (Get)",
                        context.header.packet_type
                    )));
                }
            }
            PlabbleRequestBody::Stream => todo!(),
            PlabbleRequestBody::Post => todo!(),
            PlabbleRequestBody::Patch => todo!(),
            PlabbleRequestBody::Put(body) => {
                if let RequestPacketType::Put { binary_keys, .. } = context.header.packet_type {
                    context
                        .config
                        .set_variant("bucket_type", if binary_keys { 1 } else { 0 });
                    body.serialize(Some(&mut context.config))
                } else {
                    return Err(SerializationError::InvalidData(format!(
                        "Header type {:?} did not match body (Put)",
                        context.header.packet_type
                    )));
                }
            }
            PlabbleRequestBody::Delete(body) => {
                if let RequestPacketType::Delete { binary_keys, .. } = context.header.packet_type {
                    context
                        .config
                        .set_variant("bucket_type", if binary_keys { 1 } else { 0 });
                    body.serialize(Some(&mut context.config))
                } else {
                    return Err(SerializationError::InvalidData(format!(
                        "Header type {:?} did not match body (Delete)",
                        context.header.packet_type
                    )));
                }
            }
            PlabbleRequestBody::Subscribe(body) => {
                if let RequestPacketType::Subscribe { binary_keys, .. } = context.header.packet_type
                {
                    context
                        .config
                        .set_variant("bucket_type", if binary_keys { 1 } else { 0 });
                    body.serialize(Some(&mut context.config))
                } else {
                    return Err(SerializationError::InvalidData(format!(
                        "Header type {:?} did not match body (Subscribe)",
                        context.header.packet_type
                    )));
                }
            }
            PlabbleRequestBody::Unsubscribe(body) => {
                if let RequestPacketType::Unsubscribe { binary_keys, .. } =
                    context.header.packet_type
                {
                    context
                        .config
                        .set_variant("bucket_type", if binary_keys { 1 } else { 0 });
                    body.serialize(Some(&mut context.config))
                } else {
                    return Err(SerializationError::InvalidData(format!(
                        "Header type {:?} did not match body (Unsubscribe)",
                        context.header.packet_type
                    )));
                }
            }
            PlabbleRequestBody::Register => todo!(),
            PlabbleRequestBody::Identify => todo!(),
            PlabbleRequestBody::Proxy => todo!(),
            PlabbleRequestBody::Opcode => todo!(),
        }
    }
}

impl<'a> BinaryDeserializer<RequestSerializationContext<'a>> for PlabbleRequestBody {
    fn deserialize(
        bytes: &[u8],
        config: Option<&mut SerializerConfig<RequestSerializationContext<'a>>>,
    ) -> Result<Self, DeserializationError> {
        let context = config.as_ref().unwrap().data.as_ref().unwrap();
        match context.header.packet_type {
            RequestPacketType::Certificate {
                full_chain,
                challenge,
                query_mode,
            } => todo!(),
            RequestPacketType::Session {
                persist_key,
                enable_encryption,
                with_salt,
                request_salt,
            } => todo!(),
            RequestPacketType::Get {
                binary_keys,
                subscribe,
                range_mode_until,
            } => todo!(),
            RequestPacketType::Stream {
                binary_keys,
                subscribe,
                range_mode_until,
                stream_append,
            } => todo!(),
            RequestPacketType::Post {
                binary_keys,
                subscribe,
                range_mode_until,
                do_not_persist,
            } => todo!(),
            RequestPacketType::Patch => todo!(),
            RequestPacketType::Put {
                binary_keys,
                subscribe,
                assert_keys,
                append,
            } => todo!(),
            RequestPacketType::Delete {
                binary_keys,
                range_mode_until,
            } => todo!(),
            RequestPacketType::Subscribe {
                binary_keys,
                range_mode_until,
            } => todo!(),
            RequestPacketType::Unsubscribe {
                binary_keys,
                range_mode_until,
            } => todo!(),
            RequestPacketType::Register => todo!(),
            RequestPacketType::Identify => todo!(),
            RequestPacketType::Proxy {
                init_session,
                keep_connection,
                select_random_hops,
            } => todo!(),
            RequestPacketType::_Reserved13 => todo!(),
            RequestPacketType::Opcode {
                allow_bucket_operations,
                allow_eval,
            } => todo!(),
            RequestPacketType::_Reserved15 => todo!(),
        }
    }
}
