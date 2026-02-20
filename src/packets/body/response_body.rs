use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::{
    crypto::certificate::Certificate,
    packets::body::{
        bucket::BucketBody, certificate::CertificateResponseBody, custom::CustomBody, error::PlabbleError, opcode::OpCodeResponseBody, proxy::ProxyResponseBody, session::SessionResponseBody, stream::StreamResponseBody
    },
};

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
/// - `Custom`: Represents a custom response body (for sub-protocols).
/// - `Opcode`: Represents an opcode response body.
/// - `Error`: Represents an error response body.
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
#[no_discriminator]
#[repr(u8)]
pub enum PlabbleResponseBody {
    Certificate(CertificateResponseBody) = 0,
    Session(SessionResponseBody) = 1,
    Get(#[variant_by = "binary_keys"] BucketBody) = 2,
    Stream(StreamResponseBody) = 3,
    Post = 4,
    Patch = 5,
    Put = 6,
    Delete = 7,
    Subscribe = 8,
    Unsubscribe = 9,
    Register(Certificate) = 10,
    Identity = 11,
    Proxy(#[variant_by = "init_session"] ProxyResponseBody) = 12,
    Custom(CustomBody) = 13,
    Opcode(OpCodeResponseBody) = 14,
    Error(PlabbleError) = 15,
}
