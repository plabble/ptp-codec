use ::base64::Engine;
use ::base64::prelude::BASE64_URL_SAFE_NO_PAD;
use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::crypto::hash_128;

/// Bucket Identifier
#[serde_as]
#[derive(Debug, Serialize, ToBytes, FromBytes, PartialEq)]
#[serde(transparent)]
pub struct BucketId {
    #[serde_as(as = "Base64<UrlSafe, Unpadded>")]
    pub data: [u8; 16],
}

impl BucketId {
    /// Parse BucketId from string
    /// Accepts a 16-byte base64-urlencoded string, or a UTF-8 string prefixed with magic prefix `#` (hash with Blake2b) or `@` (hash with Blake3)
    pub fn parse(repr: &str) -> Option<Self> {
        match repr.chars().next()? {
            '#' => Some(Self {
                data: hash_128(false, vec![&repr.as_bytes()[1..]]),
            }),
            '@' => {
                let hash = hash_128(true, vec![&repr.as_bytes()[1..]]);
                Some(Self { data: hash })
            }
            _ => {
                let decode = BASE64_URL_SAFE_NO_PAD.decode(repr).ok()?;
                Some(Self {
                    data: decode.try_into().ok()?,
                })
            }
        }
    }
}

impl<'de> Deserialize<'de> for BucketId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = String::deserialize(deserializer)?;
        Ok(Self::parse(&str).expect("Not a valid base64 url-encoded string"))
    }
}

#[cfg(test)]
mod tests {
    use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};

    use crate::core::BucketId;

    #[test]
    fn can_create_hashed_bucket_id_with_blake2b() {
        let id = BucketId::parse("#test").unwrap();
        assert_eq!(
            id.data,
            BASE64_URL_SAFE_NO_PAD
                .decode("RKiZXdULZlegN6eDkwRTWw")
                .unwrap()[..]
        )
    }

    #[cfg(feature = "blake-3")]
    #[test]
    fn can_create_hashed_bucket_id_with_blake3() {
        let id = BucketId::parse("@test").unwrap();
        assert_eq!(
            id.data,
            BASE64_URL_SAFE_NO_PAD
                .decode("SHjKBCXHOfpCf37aIP6EXw")
                .unwrap()[..]
        )
    }
}
