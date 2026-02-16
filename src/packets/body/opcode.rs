use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::formats::Lowercase;
use serde_with::hex::Hex;
use serde_with::serde_as;

use crate::scripting::opcode_script::OpcodeScript;

/// Execute OPCODE script on server
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct OpCodeRequestBody {
    /// The script to run
    script: OpcodeScript,
}

/// OPCODE script response from server
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct OpCodeResponseBody {
    /// The result of the script execution, if any
    #[serde_as(as = "Option<Hex<Lowercase>>")]
    result: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::{
        packets::{
            base::PlabblePacketBase,
            body::{opcode::OpCodeRequestBody, request_body::PlabbleRequestBody},
            header::{request_header::PlabbleRequestHeader, type_and_flags::RequestPacketType},
            request::PlabbleRequestPacket,
            response::PlabbleResponsePacket,
        },
        scripting::opcode_script::{Opcode, OpcodeScript},
    };

    #[test]
    fn can_serialize_and_deserialize_opcode_request() {
        let script = OpcodeScript::new(vec![
            Opcode::PUSHINT(5),
            Opcode::PUSHINT(2),
            Opcode::PUSHINT(3),
            Opcode::ADD,
            Opcode::EQ,
            Opcode::PUSH2([1, 2]),
        ]);

        let body = OpCodeRequestBody { script };
        let packet = PlabbleRequestPacket {
            base: PlabblePacketBase::default(),
            header: PlabbleRequestHeader::new(
                RequestPacketType::Opcode {
                    allow_bucket_operations: false,
                    allow_eval: false,
                },
                None,
            ),
            body: PlabbleRequestBody::Opcode(body),
        };

        let request: PlabbleRequestPacket = toml::from_str(r#"
            version = 1

            [header]
            packet_type = "Opcode"
            allow_bucket_operations = false
            allow_eval = false

            [body.script]
            instructions = [{ PUSHINT = 5 }, { PUSHINT = 2 }, { PUSHINT = 3 }, "ADD", "EQ", { PUSH2 = "0102" }]
        "#).unwrap();

        assert_eq!(request, packet);

        let serialized = packet.to_bytes(None).unwrap();
        assert_eq!("010e080a080408060a36030102", hex::encode(&serialized));

        let deserialized = PlabbleRequestPacket::from_bytes(&serialized, None).unwrap();
        assert_eq!(deserialized, packet);
    }

    #[test]
    fn can_serialize_and_deserialize_opcode_response() {
        let packet: PlabbleResponsePacket = toml::from_str(
            r#"
            version = 1
            
            [header]
            packet_type = "Opcode"
            request_counter = 7

            [body]
            result = "0102030405"
        "#,
        )
        .unwrap();

        let serialized = packet.to_bytes(None).unwrap();
        assert_eq!("010e00070102030405", hex::encode(&serialized));

        let deserialized = PlabbleResponsePacket::from_bytes(&serialized, None).unwrap();
        assert_eq!(packet, deserialized);
    }
}
