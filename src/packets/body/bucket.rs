use std::collections::HashMap;

use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;

#[serde_as]
#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize)]
// #[variant_by = "TODO"]
pub enum BucketBody {
    Binary(
        #[val_dyn_length]
        #[key_dyn_length]
        HashMap<String, Vec<u8>>,
    ),
    Numeric(
        #[val_dyn_length]
        #[serde_as(as = "HashMap<DisplayFromStr, Base64<UrlSafe, Unpadded>>")]
        HashMap<u16, Vec<u8>>
    ),
}

#[derive(Debug, FromBytes, ToBytes, Serialize, Deserialize)]
// #[variant_by = "TODO"]
pub enum BucketRange {
    Binary(
        #[dyn_length]
        Option<String>, 
        Option<String>
    ),
    Numeric(
        Option<u16>,
        Option<u16>,
    ),
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::body::bucket::{BucketBody, BucketRange};

    #[test]
    fn can_serialize_and_deserialize_bucket_range() {
        let mut hm = HashMap::new();
        hm.insert(123, vec![1,2,3,4]);
        hm.insert(243, vec![1,2]);


        let range = BucketBody::Numeric(hm);
        let bytes = range.to_bytes(None).unwrap();
        println!("{:?}", bytes);

        let parsed = BucketBody::from_bytes(&bytes, None).unwrap();
        println!("{:?}", parsed);

        println!("{:?}", toml::to_string(&range).unwrap());
    }
}