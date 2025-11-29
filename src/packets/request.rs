use serde::{Deserialize, Serialize};

use crate::packets::{
    base::PlabblePacketBase,
    body::PlabbleRequestBody,
    header::{request_header::PlabbleRequestHeader, type_and_flags::RequestPacketType},
};

#[derive(Serialize, Debug, PartialEq)]
pub struct PlabbleRequestPacket {
    #[serde(flatten)]
    base: PlabblePacketBase,

    header: PlabbleRequestHeader,

    body: PlabbleRequestBody,
}

impl<'de> Deserialize<'de> for PlabbleRequestPacket {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Debug)]
        struct RawPacket {
            #[serde(flatten)]
            base: PlabblePacketBase,
            header: PlabbleRequestHeader,
            // We'll temporarily store the body as untyped data
            body: serde_value::Value, // or `Vec<u8>` if it’s binary
        }

        let raw = RawPacket::deserialize(deserializer)?;
        let body = match raw.header.packet_type {
            RequestPacketType::Certificate { .. } => todo!(),
            RequestPacketType::Session { .. } => {
                PlabbleRequestBody::Session(raw.body.deserialize_into().unwrap())
            }
            RequestPacketType::Get { .. } => todo!(),
            RequestPacketType::Stream { .. } => todo!(),
            RequestPacketType::Post { .. } => todo!(),
            RequestPacketType::Patch => todo!(),
            RequestPacketType::Put { .. } => todo!(),
            RequestPacketType::Delete { .. } => todo!(),
            RequestPacketType::Subscribe { .. } => todo!(),
            RequestPacketType::Unsubscribe { .. } => todo!(),
            RequestPacketType::Register => todo!(),
            RequestPacketType::Identify => todo!(),
            RequestPacketType::Proxy { .. } => todo!(),
            RequestPacketType::_Reserved13 => todo!(),
            RequestPacketType::_Reserved14 => todo!(),
            RequestPacketType::_Reserved15 => todo!(),
        };

        Ok(PlabbleRequestPacket {
            base: raw.base,
            header: raw.header,
            body,
        })
    }
}

// Tests for the request and response packets are in the type-specific files
