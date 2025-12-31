use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::{DisplayFromStr, serde_as};

/// Bucket query structure used for querying bucket data
/// with a specific ID and range.
/// The range can be either numeric or binary, depending on the bucket type.
///
/// # Members
/// - `id`: A 16-byte array representing the unique identifier of the bucket.
/// - `range`: A `BucketRange` enum representing the range of data to query within the bucket.
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct BucketQuery {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    id: [u8; 16],
    range: BucketRange,
}

/// Bucket put request structure used for inserting data into a bucket
/// with a specific ID and body. (for PUT request)
///
/// # Members
/// - `id`: A 16-byte array representing the unique identifier of the bucket.
/// - `body`: A `BucketBody` enum representing the data to be inserted into the bucket.
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct PutRequestBody {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    id: [u8; 16],
    body: BucketBody,
}

/// Bucket body structure used for representing the data within a bucket.
/// The body can be either numeric or binary, depending on the bucket type.
///
/// This is used for writing or reading data from bucket slots
///
/// # Members
/// - `Numeric`: A hashmap where the key is a `u16` representing the slot number,
///   and the value is a vector of bytes representing the data stored in that slot.
/// - `Binary`: A hashmap where the key is a `String` representing the slot identifier,
///   and the value is a vector of bytes representing the data stored in that slot.
#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[no_discriminator]
pub enum BucketBody {
    #[toggled_by = "!binary_keys"]
    Numeric(
        #[val_dyn_length]
        #[serde_as(as = "HashMap<DisplayFromStr, Base64<UrlSafe, Unpadded>>")]
        HashMap<u16, Vec<u8>>,
    ),
    #[toggled_by = "binary_keys"]
    Binary(
        #[val_dyn_length]
        #[key_dyn_length]
        #[serde_as(as = "HashMap<_, Base64<UrlSafe, Unpadded>>")]
        HashMap<String, Vec<u8>>,
    ),
}

/// Bucket range structure used for specifying the range of data
/// to query within a bucket.
/// The range can be either numeric or binary, depending on the bucket type.
///
/// # Members
/// - `Numeric`: A tuple containing two optional `u16` values representing optionally
///  the start and/or end of the numeric range
/// - `Binary`: A tuple containing two optional `String` values representing optionally
///  the start and/or end of the binary range.
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[no_discriminator]
pub enum BucketRange {
    #[toggled_by = "!binary_keys"]
    Numeric(Option<u16>, Option<u16>),

    #[toggled_by = "binary_keys"]
    Binary(#[dyn_length] Option<String>, Option<String>),
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, vec};

    use crate::packets::body::bucket::{BucketBody, PutRequestBody};

    #[test]
    fn can_serialize_and_deserialize_bucket_query() {}
}
