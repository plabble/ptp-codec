use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::crypto::algorithm::VerificationKey;

/// Register a new identity on the server, which can be used for authentication in future sessions.
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct RegisterRequestBody {
    /// Public keys (new generated) for all algorithms specified in `crypto_settings` in the packet base.
    #[multi_enum]
    keys: Vec<VerificationKey>,

    /// Claims to register, key-value, UTF-8. Separated by ;, like `USERNAME=henk;AGE=12`
    claims: String,
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer};

    use crate::packets::request::PlabbleRequestPacket;

    #[test]
    fn can_serialize_and_deserialize_register_request() {
        let key = "f0a22003a3d06ed16f5920a03c1297c742c281bda4895ea7caca682c276bf098";

        let request: PlabbleRequestPacket = toml::from_str(
            r#"
            version = 1
            specify_crypto_settings = true

            [crypto_settings]
            sign_ed25519 = true

            [header]
            packet_type = "Register"

            [body]
            claims = "USERNAME=henk;AGE=24"

            [[body.keys]]
            Ed25519 = "8KIgA6PQbtFvWSCgPBKXx0LCgb2kiV6nyspoLCdr8Jg"
        "#,
        )
        .unwrap();

        let bytes = request.to_bytes(None).unwrap();
        assert_eq!(
            format!("81310a{}{}", key, hex::encode("USERNAME=henk;AGE=24")),
            hex::encode(&bytes)
        );

        let deserialized = PlabbleRequestPacket::from_bytes(&bytes, None).unwrap();
        assert_eq!(request, deserialized);
    }
}
