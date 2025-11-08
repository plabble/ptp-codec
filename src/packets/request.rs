use binary_codec::{BinaryDeserializer, SerializerConfig};
use serde::{Deserialize, Serialize};

use crate::packets::{
    base::PlabblePacketBase,
    body::{PlabbleRequestBody, RequestSerializationContext},
    header::{request_header::PlabbleRequestHeader, type_and_flags::RequestPacketType},
};

#[derive(Serialize, Debug)]
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
            body: toml::Value, // or `Vec<u8>` if it’s binary
        }

        let raw = RawPacket::deserialize(deserializer)?;
        let body = match raw.header.packet_type {
            RequestPacketType::Certificate { .. } => todo!(),
            RequestPacketType::Session { .. } => PlabbleRequestBody::Session(raw.body.try_into().unwrap()),
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

        Ok(PlabbleRequestPacket { base: raw.base, header: raw.header, body })
    }
}

#[cfg(test)]
mod tests {
    use binary_codec::BinaryDeserializer;

    use crate::packets::{
        base::{PlabblePacketBase, crypto_keys::CryptoKey},
        body::{PlabbleRequestBody, session::SessionRequestBody},
        header::{request_header::PlabbleRequestHeader, type_and_flags::RequestPacketType},
        request::PlabbleRequestPacket,
    };

    #[test]
    pub fn try_serde_serialize() {
        let packet = PlabbleRequestPacket {
            base: PlabblePacketBase {
                version: 0,
                fire_and_forget: false,
                pre_shared_key: false,
                use_encryption: true,
                specify_crypto_settings: false,
                crypto_settings: None,
                psk_id: None,
                psk_salt: None,
                mac: None,
            },
            header: PlabbleRequestHeader::new(RequestPacketType::Session {
                persist_key: true,
                enable_encryption: true,
            }),
            body: PlabbleRequestBody::Session(SessionRequestBody {
                psk_expiration: Some([1, 2, 3, 4]),
                keys: vec![CryptoKey::X25519([0u8; 32])],
            }),
        };

        let data = toml::to_string(&packet).unwrap();
        println!("{}", data);

        let des: PlabbleRequestPacket = toml::from_str(&data).unwrap();
        println!("{:?}", des);
    }
}
