use binary_codec::{BinarySerializer, DeserializationError, SerializationError};
use serde::{Deserialize, Serialize};

use crate::packets::{
    body::{
        RequestSerializationContext,
        bucket::{BucketQuery, PutRequestBody},
        session::SessionRequestBody,
    },
    header::type_and_flags::RequestPacketType,
};

pub trait SerializableRequestBody {
    fn to_bytes(
        &self,
        context: &mut RequestSerializationContext,
    ) -> Result<Vec<u8>, SerializationError>;
    fn from_bytes(
        bytes: &[u8],
        context: &mut RequestSerializationContext,
    ) -> Result<Self, DeserializationError>
    where
        Self: Sized;
}

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
}

impl SerializableRequestBody for PlabbleRequestBody {
    fn to_bytes(
        &self,
        context: &mut RequestSerializationContext,
    ) -> Result<Vec<u8>, SerializationError> {
        match self {
            PlabbleRequestBody::Certificate => todo!(),
            PlabbleRequestBody::Session(body) => body.to_bytes(context),
            PlabbleRequestBody::Get(body) => {
                if let RequestPacketType::Get { binary_keys, .. } = context.header.packet_type {
                    context
                        .config
                        .set_variant("bucket_type", if binary_keys { 1 } else { 0 });
                    body.to_bytes(Some(&mut context.config))
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
                    body.to_bytes(Some(&mut context.config))
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
                    body.to_bytes(Some(&mut context.config))
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
                    body.to_bytes(Some(&mut context.config))
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
                    body.to_bytes(Some(&mut context.config))
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
        }
    }

    fn from_bytes(
        bytes: &[u8],
        context: &mut RequestSerializationContext,
    ) -> Result<Self, DeserializationError>
    where
        Self: Sized,
    {
        match context.header.packet_type {
            RequestPacketType::Certificate {
                full_chain,
                challenge,
                query_mode,
            } => todo!(),
            RequestPacketType::Session {
                persist_key,
                enable_encryption,
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
                with_keys,
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
            RequestPacketType::_Reserved14 => todo!(),
            RequestPacketType::_Reserved15 => todo!(),
        }
    }
}
