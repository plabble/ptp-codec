use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::{Unpadded};

use settings::EncryptionSettings;

pub mod header;
pub mod settings;

/// Plabble Protocol Packet
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlabblePacketBase {
    /// Plabble Protocol version
    /// 0 = debug
    #[bits = 4]
    version: u8,

    /// If set to true, this packet is sent outside of a session
    /// and no follow-up responses are expected.
    #[serde(default)]
    fire_and_forget: bool,

    /// If set to true, this packet uses a pre-shared key for encryption.
    #[serde(default)]
    pre_shared_key: bool,

    /// If set to true, this packet uses encryption. If false, use a MAC (Message Authentication Code).
    #[serde(default)]
    use_encryption: bool,

    /// If set to true, use custom encryption settings.
    #[serde(default)]
    specify_encryption_settings: bool,

    /// Encryption settings
    #[toggled_by = "specify_encryption_settings"]
    encryption_settings: Option<EncryptionSettings>,

    /// Pre-shared key ID, if using a pre-shared key
    #[toggled_by = "pre_shared_key"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    pre_shared_key_id: Option<[u8; 16]>,

    /// Pre-shared key salt, if using a pre-shared key
    #[toggled_by = "pre_shared_key"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    psk_salt: Option<[u8; 16]>,

    /// Packet payload, encrypted or not depending on the settings above.
    /// It also contains the encrypted part of the header
    #[serde(skip_serializing)]
    payload: Option<Vec<u8>>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_serialize_empty_base_packet() {
        let toml = r#"
        version = 1
        fire_and_forget = true
        use_encryption = true
        "#;

        let packet: PlabblePacketBase = toml::from_str(toml).unwrap();
        let bytes = packet.to_bytes().unwrap();
        let deserialized_packet = PlabblePacketBase::from_bytes(&bytes).unwrap();
        assert_eq!(packet, deserialized_packet);
        assert_eq!(vec![0b0101_0001], bytes);
        assert_eq!(packet.fire_and_forget, true);
        assert_eq!(packet.use_encryption, true);

        // Check some defaults
        assert_eq!(packet.pre_shared_key, false);
        assert_eq!(packet.specify_encryption_settings, false);

        println!("{:?}", packet);
    }
}