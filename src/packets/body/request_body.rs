use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::packets::{
    body::{
        bucket::{BucketQuery, PutRequestBody},
        session::SessionRequestBody,
    },
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
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
#[no_discriminator]
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