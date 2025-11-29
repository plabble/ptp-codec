use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::{DisplayFromStr, serde_as};

#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct BucketQuery {
    id: [u8; 16],
    range: BucketRange,
}

#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
pub struct PutRequestBody {
    id: [u8; 16],
    body: BucketBody,
}

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

#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize, PartialEq)]
#[variant_by = "bucket_type"]
#[no_discriminator]
pub enum BucketRange {
    Numeric(Option<u16>, Option<u16>),
    Binary(#[dyn_length] Option<String>, Option<String>),
}

#[cfg(test)]
mod tests {}
