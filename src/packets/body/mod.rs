use binary_codec::{DeserializationError, SerializationError, SerializerConfig};

use crate::packets::{base::PlabblePacketBase, header::{request_header::PlabbleRequestHeader, response_header::PlabbleResponseHeader}};

mod session;

pub struct RequestSerializationContext<'a> {
    pub header: &'a PlabbleRequestHeader,
    pub packet: &'a PlabblePacketBase,
    pub config: &'a mut SerializerConfig
}

pub struct ResponseSerializationContext<'a> {
    pub header: &'a PlabbleResponseHeader,
    pub packet: &'a PlabblePacketBase,
    pub config: &'a mut SerializerConfig
}

pub trait SerializableRequestBody {
    fn to_bytes(&self, context: RequestSerializationContext) -> Result<Vec<u8>, SerializationError>;
    fn from_bytes(bytes: &[u8], context: RequestSerializationContext) -> Result<Self, DeserializationError> where Self: Sized;
}

pub trait SerializableResponseBody {
    fn to_bytes(&self, context: ResponseSerializationContext) -> Result<Vec<u8>, SerializationError>;
    fn from_bytes(bytes: &[u8], context: ResponseSerializationContext) -> Result<Self, DeserializationError> where Self: Sized;
}