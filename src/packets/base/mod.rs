use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::{Unpadded};

use settings::EncryptionSettings;

mod flags;
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
    psk_id: Option<[u8; 16]>,

    /// Pre-shared key salt, if using a pre-shared key
    #[toggled_by = "pre_shared_key"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    psk_salt: Option<[u8; 16]>,

    /// Message Authentication Code (MAC)
    #[toggled_by = "!use_encryption"]
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    mac: Option<[u8; 16]>,

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
        assert_eq!(packet.fire_and_forget, true);
        assert_eq!(packet.use_encryption, true);

        // Check some defaults
        assert_eq!(packet.pre_shared_key, false);
        assert_eq!(packet.specify_encryption_settings, false);

        assert_eq!(vec![0b0101_0001], bytes);
    }

    #[test]
    fn can_serialize_packet_with_mac() {
        let toml = r#"
        version = 0
        use_encryption = false
        mac = "AQIDBAUGBwgJEBESExQVFg"
        "#;

        let packet: PlabblePacketBase = toml::from_str(toml).unwrap();
        let bytes = packet.to_bytes().unwrap();
        let deserialized_packet = PlabblePacketBase::from_bytes(&bytes).unwrap();
        assert_eq!(packet, deserialized_packet);

        assert_eq!(vec![0b0000_0000, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16], bytes);
    }

    #[test]
    fn can_serialize_packet_with_full_settings_and_psk() {
        let toml = r#"
        version = 1
        use_encryption = true
        pre_shared_key = true
        specify_encryption_settings = true

        psk_id = "AQIDBAUGBwgJEBESExQVFg"
        psk_salt = "BwAAAAAAAAAAAAAAAAAABw"

        [encryption_settings]
        use_post_quantum = true

        [encryption_settings.post_quantum_settings]
        sign_pqc_dsa_44 = true
        "#;

        let packet: PlabblePacketBase = toml::from_str(toml).unwrap();
        let bytes = packet.to_bytes().unwrap();
        let deserialized_packet = PlabblePacketBase::from_bytes(&bytes).unwrap();
        assert_eq!(packet, deserialized_packet);
        assert_eq!(vec![0b1110_0001, 0b1011_0001, 0b0000_0001, 
            1, 2, 3, 4, 5, 6, 7, 8, 9, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 
            7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7], bytes);
    }
}