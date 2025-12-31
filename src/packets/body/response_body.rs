use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::packets::body::{bucket::BucketBody, error::PlabbleError, session::SessionResponseBody};

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
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
#[no_discriminator]
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
    Error(PlabbleError),
}
