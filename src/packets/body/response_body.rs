use binary_codec::{DeserializationError, SerializationError};
use serde::{Deserialize, Serialize};

use crate::packets::body::{
    ResponseSerializationContext, bucket::BucketBody, session::SessionResponseBody,
};

/// A trait for serializing and deserializing response bodies.
/// Implementors of this trait can convert their data to and from byte arrays
/// using the provided serialization context.
///
/// # Methods
/// - `to_bytes`: Serializes the response body into a byte vector.
/// - `from_bytes`: Deserializes a byte slice into the response body.
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

/// An enumeration representing the different types of response bodies
/// that can be sent in a Plabble response.
/// Each variant corresponds to a specific response type and may contain
/// associated data relevant to that response.
///
/// # Variants
/// - `Certificate`: Represents a certificate response body.
/// - `Session`: Represents a session response body.
/// - `Get`: Represents a get response body with a bucket body.
/// - `Stream`: Represents a stream response body.
/// - `Post`: Represents a post response body.
/// - `Patch`: Represents a patch response body.
/// - `Put`: Represents a put response body.
/// - `Delete`: Represents a delete response body.
/// - `Subscribe`: Represents a subscribe response body.
/// - `Unsubscribe`: Represents an unsubscribe response body.
/// - `Register`: Represents a register response body.
/// - `Identity`: Represents an identity response body.
/// - `Proxy`: Represents a proxy response body.
/// - `Opcode`: Represents an opcode response body.
/// - `Error`: Represents an error response body.
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
    Opcode,
    Error,
}
