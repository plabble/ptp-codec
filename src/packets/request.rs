use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::packets::{base::PlabblePacketBase, header::request_header::PlabbleRequestHeader};

#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlabbleRequestPacket {
    #[serde(flatten)]
    base: PlabblePacketBase,

    header: PlabbleRequestHeader,
}