use binary_codec::{DeserializationError, SerializationError, SerializerConfig};
use serde::{Deserialize, Serialize};

use crate::packets::{
    base::PlabblePacketBase,
    body::session::{SessionRequestBody, SessionResponseBody},
    header::{request_header::PlabbleRequestHeader, response_header::PlabbleResponseHeader},
};

pub mod session;

#[derive(Clone)]
pub struct RequestSerializationContext<'a> {
    pub header: &'a PlabbleRequestHeader,
    pub packet: &'a PlabblePacketBase,
    pub config: SerializerConfig,
}

#[derive(Clone)]
pub struct ResponseSerializationContext<'a> {
    pub header: &'a PlabbleResponseHeader,
    pub packet: &'a PlabblePacketBase,
    pub config: SerializerConfig,
}

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

pub trait SerializableResponseBody {
    fn to_bytes(
        &self,
        context: &mut ResponseSerializationContext,
    ) -> Result<Vec<u8>, SerializationError>;
    fn from_bytes(
        bytes: &[u8],
        context: &mut ResponseSerializationContext,
    ) -> Result<Self, DeserializationError>
    where
        Self: Sized;
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PlabbleRequestBody {
    Session(SessionRequestBody),
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PlabbleResponseBody {
    Session(SessionResponseBody),
}
