use binary_codec::{DeserializationError, SerializationError};
use serde::{Deserialize, Serialize};

use crate::packets::body::{
    ResponseSerializationContext, bucket::BucketBody, session::SessionResponseBody,
};

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
pub enum PlabbleResponseBody {
    Certificate,
    Session(SessionResponseBody),
    Get(BucketBody),
    Stream,
    Post,
    Patch,
    Put,
    Delete,
    Subscribe,
    Unsubscribe,
    Register,
    Identity,
    Proxy,
    Error,
}
