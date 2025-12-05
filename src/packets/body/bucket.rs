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
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct BucketQuery {
    id: [u8; 16],
    range: BucketRange,
}

/// Bucket put request structure used for inserting data into a bucket
/// with a specific ID and body. (for PUT request)
/// 
/// # Members
/// - `id`: A 16-byte array representing the unique identifier of the bucket.
/// - `body`: A `BucketBody` enum representing the data to be inserted into the bucket.
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct PutRequestBody {
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
#[variant_by = "bucket_type"]
#[no_discriminator]
pub enum BucketBody {
    Numeric(
        #[val_dyn_length]
        #[serde_as(as = "HashMap<DisplayFromStr, Base64<UrlSafe, Unpadded>>")]
        HashMap<u16, Vec<u8>>,
    ),
    Binary(
        #[val_dyn_length]
        #[key_dyn_length]
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
#[variant_by = "bucket_type"]
#[no_discriminator]
pub enum BucketRange {
    Numeric(Option<u16>, Option<u16>),
    Binary(#[dyn_length] Option<String>, Option<String>),
}

#[cfg(test)]
mod tests {}
