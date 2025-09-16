use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

#[derive(ToBytes, FromBytes, Debug, PartialEq, Serialize, Deserialize)]
struct SessionRequest {
    keys: Vec<Vec<u8>>, // TODO: problem: how to handle this, because it is based on the config
    psk_expiration: Option<u32>
}

// TODO:
// option 1: create traits SerializeToBytes, DeserializeFromBytes in the binary_codec crate
// option 2: Put in final Vec<u8> or Vec<Vec<u8>> if that works, and create a keys() function that handles it
// last option would be the nicest, because it can also validate the keys from TOML.

/*
enum Keys {
    Ed25519([u8; 32]),
    X25519([u8; 32]),
    ...
}
*/

struct SessionResponse {
    // #[toggled_by = "PARENT!!!"]
    psk_id: Option<[u8; 16]>,
    keys: Vec<Vec<u8>>, // TODO: problem: how to handle this, because it is based on the config
    signatures: Vec<Vec<u8>> // TODO: problem: how to handle this, because it is based on the config
}

// TODO:
// Problem: how to handle toggled_by from parent? 

// This problem is in every packet body that has optional fields based on flags.

/*
    #[toggle_key = "my_rt_key"]
    #[toggled_by = "my_rt_key"]

*/