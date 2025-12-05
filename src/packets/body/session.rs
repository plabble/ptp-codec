use binary_codec::{BinarySerializer, DeserializationError, SerializationError, utils::slice};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::packets::base::crypto_keys::CryptoSignature;
use crate::packets::body::request_body::SerializableRequestBody;
use crate::packets::body::response_body::SerializableResponseBody;
use crate::packets::header::type_and_flags::ResponsePacketType;
use crate::packets::{base::crypto_keys::CryptoKey, header::type_and_flags::RequestPacketType};

/// Session request body
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SessionRequestBody {
    /// PSK expiration Plabble timestamp. Filled if request flag persist_key is set.
    pub psk_expiration: Option<[u8; 4]>,

    /// Client-generated salt for key derivation. Filled if request flag with_salt is set.
    pub salt: Option<[u8; 16]>,

    /// Public/encapsulation keys for creating a shared secret with the server
    pub keys: Vec<CryptoKey>,
}

impl SerializableRequestBody for SessionRequestBody {
    fn to_bytes(
        &self,
        context: &mut super::RequestSerializationContext,
    ) -> Result<Vec<u8>, SerializationError> {
        let mut bytes: Vec<u8> = Vec::new();
        if let RequestPacketType::Session { persist_key, with_salt, .. } = context.header.packet_type {
            if persist_key && self.psk_expiration.is_none() {
                return Err(SerializationError::InvalidData(String::from(
                    "psk_expiration should be set if persist_key flag is set",
                )));
            }

            if with_salt && self.salt.is_none() {
                return Err(SerializationError::InvalidData(String::from(
                    "salt should be set if with_salt flag is set",
                )));
            };

            if let Some(expiration_bytes) = self.psk_expiration {
                bytes.extend_from_slice(&expiration_bytes);
                context.config.pos += expiration_bytes.len();
            }

            if let Some(salt_bytes) = self.salt {
                bytes.extend_from_slice(&salt_bytes);
                context.config.pos += salt_bytes.len();
            }
        } else {
            return Err(SerializationError::InvalidData(format!(
                "Header type {:?} did not match body (Session)",
                context.header.packet_type
            )));
        }

        let crypto_settings = context.packet.crypto_settings.clone().unwrap_or_default();
        let key_types = CryptoKey::get_key_exchange_key_types(&crypto_settings, true);
        CryptoKey::verify_keys(key_types, &self.keys)?;

        for key in self.keys.iter() {
            key.write_bytes(&mut bytes, Some(&mut context.config))?;
        }

        Ok(bytes)
    }

    fn from_bytes(
        bytes: &[u8],
        context: &mut super::RequestSerializationContext,
    ) -> Result<Self, DeserializationError>
    where
        Self: Sized,
    {
        if let RequestPacketType::Session { persist_key, with_salt, .. } = context.header.packet_type {
            let psk_expiration = if persist_key {
                Some(
                    slice(&mut context.config, bytes, 4, true)?
                        .try_into()
                        .unwrap(),
                )
            } else {
                None
            };

            let salt = if with_salt {
                Some(
                    slice(&mut context.config, bytes, 16, true)?
                        .try_into()
                        .unwrap(),
                )
            } else {
                None
            };

            // TODO: get_crypto_settings method
            let crypto_settings = context.packet.crypto_settings.clone().unwrap_or_default();
            let key_types = CryptoKey::get_key_exchange_key_types(&crypto_settings, true);
            let keys = CryptoKey::read_keys(bytes, key_types, &mut context.config)?;

            Ok(Self {
                psk_expiration,
                salt,
                keys,
            })
        } else {
            return Err(DeserializationError::InvalidData(format!(
                "Header type {:?} did not match body (Session)",
                context.header.packet_type
            )));
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SessionResponseBody {
    /// Pre-shared key identifier. Filled if request flag with_psk is set.
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    psk_id: Option<[u8; 12]>,

    /// Server-generated salt for key derivation. Filled if request flag with_salt is set.
    salt: Option<[u8; 16]>,

    /// Public keys or encapsulated secret for creating a shared secret
    keys: Vec<CryptoKey>,

    /// Signatures of the request
    signatures: Vec<CryptoSignature>,
}

impl SerializableResponseBody for SessionResponseBody {
    fn to_bytes(
        &self,
        context: &mut super::ResponseSerializationContext,
    ) -> Result<Vec<u8>, SerializationError> {
        let mut bytes: Vec<u8> = Vec::new();
        if let ResponsePacketType::Session { with_psk, with_salt } = context.header.packet_type {
            if with_psk && self.psk_id.is_none() {
                return Err(SerializationError::InvalidData(String::from(
                    "psk_id should be set if with_psk flag is set",
                )));
            }

            if with_salt && self.salt.is_none() {
                return Err(SerializationError::InvalidData(String::from(
                    "salt should be set if with_salt flag is set",
                )));
            };

            if let Some(psk_id_bytes) = self.psk_id {
                bytes.extend_from_slice(&psk_id_bytes);
                context.config.pos += psk_id_bytes.len();
            }

            if let Some(salt_bytes) = self.salt {
                bytes.extend_from_slice(&salt_bytes);
                context.config.pos += salt_bytes.len();
            }
        } else {
            return Err(SerializationError::InvalidData(String::from(
                "Header type did not match body",
            )));
        }

        let crypto_settings = context.packet.crypto_settings.clone().unwrap_or_default();

        // Check if key and signature types match the crypto settings
        let key_types = CryptoKey::get_key_exchange_key_types(&crypto_settings, false);
        CryptoKey::verify_keys(key_types, &self.keys)?;
        let signature_types = CryptoKey::get_signature_types(&crypto_settings);
        CryptoKey::verify_signatures(signature_types, &self.signatures)?;

        for key in self.keys.iter() {
            key.write_bytes(&mut bytes, Some(&mut context.config))?;
        }

        for signature in self.signatures.iter() {
            signature.write_bytes(&mut bytes, Some(&mut context.config))?;
        }

        Ok(bytes)
    }

    fn from_bytes(
        bytes: &[u8],
        context: &mut super::ResponseSerializationContext,
    ) -> Result<Self, DeserializationError>
    where
        Self: Sized,
    {
        if let ResponsePacketType::Session { with_psk, with_salt } = context.header.packet_type {
            let psk_id = if with_psk {
                Some(
                    slice(&mut context.config, bytes, 12, true)?
                        .try_into()
                        .unwrap(),
                )
            } else {
                None
            };

            let salt = if with_salt {
                Some(
                    slice(&mut context.config, bytes, 16, true)?
                        .try_into()
                        .unwrap(),
                )
            } else {
                None
            };

            // TODO: get_crypto_settings method
            let crypto_settings = context.packet.crypto_settings.clone().unwrap_or_default();
            let key_types = CryptoKey::get_key_exchange_key_types(&crypto_settings, false);
            let signature_types = CryptoKey::get_signature_types(&crypto_settings);

            let keys = CryptoKey::read_keys(bytes, key_types, &mut context.config)?;
            let signatures =
                CryptoKey::read_signatures(bytes, signature_types, &mut context.config)?;

            Ok(Self {
                psk_id,
                salt,
                keys,
                signatures,
            })
        } else {
            Err(DeserializationError::InvalidData(String::from(
                "Header type did not match body",
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::iter::repeat;

    use base64::{Engine, prelude::BASE64_STANDARD};
    use binary_codec::SerializerConfig;

    use crate::packets::{
        base::PlabblePacketBase,
        body::{
            RequestSerializationContext, ResponseSerializationContext,
            request_body::SerializableRequestBody,
            response_body::SerializableResponseBody,
            session::{SessionRequestBody, SessionResponseBody},
        },
        header::{request_header::PlabbleRequestHeader, response_header::PlabbleResponseHeader},
        request::PlabbleRequestPacket,
    };

    #[test]
    fn can_serialize_and_deserialize_session_request() {
        let (base, header) = get_context();
        let mut context = RequestSerializationContext {
            header: &header,
            packet: &base,
            config: SerializerConfig::new(),
        };

        // [body]
        let body: SessionRequestBody = toml::from_str(
            format!(
                r#"
        psk_expiration = [1,2,3,4]

        [[keys]]
        X25519 = "si6IcNvysw_Ex8D9Z1Q0LFi1vNrvfA3lAhfwy2_Hw24"
        
        [[keys]]
        Kem512 = "{}"
        "#,
                repeat('A').take(1067).collect::<String>()
            )
            .as_str(),
        )
        .unwrap();

        let bytes = body.to_bytes(&mut context).unwrap();
        assert!(matches!(
            bytes[..],
            [
                1,
                2,
                3,
                4,
                178,
                46,
                136,
                112,
                219,
                242,
                179,
                15,
                196,
                199,
                192,
                253,
                103,
                84,
                52,
                44,
                88,
                181,
                188,
                218,
                239,
                124,
                13,
                229,
                2,
                23,
                240,
                203,
                111,
                199,
                195,
                110,
                0,
                ..,
                0
            ]
        ));

        let mut context = RequestSerializationContext {
            header: &header,
            packet: &base,
            config: SerializerConfig::new(),
        };

        let deserialized = SessionRequestBody::from_bytes(&bytes, &mut context).unwrap();
        assert_eq!(Some([1, 2, 3, 4]), deserialized.psk_expiration);
        assert_eq!(body, deserialized);
    }

    #[test]
    fn can_serialize_and_deserialize_session_response() {
        let base = get_base();
        let header: PlabbleResponseHeader = toml::from_str(
            r#"
        packet_type = "Session"
        with_psk = true
        "#,
        )
        .unwrap();
        let body: SessionResponseBody = toml::from_str(r#"
        psk_id = "zJlmxMNqghPCm2Yy"

        [[keys]]
        X25519 = "WmvHVXE6LYidWO__qdhm_K-_0ztMmnMolgNN4G-7F4M"

        [[keys]]
        Kem512Cipher = "jw1AT6-H9q4v_1JqEz0LlOYj1Z8l3BDI-Zc4le35dKTSbBsx4UnBIPP-T3OvDF-vBQGJ4RjbtSfB9zBYX4zJIbHvRUkgP3aBhvwlqIyB_EPvrS3dC4QWBbbjOE0TU6eXuCVRf6HzyFJyTv48FynUljnTNn-mpmT8yRVdLqvY-ayCFtRz9u6InoybQ91WPZ9RDoWRj8h2GC1TWwjH8lTmeXeGImPMiqviESl0T9RUDTS1yXuAgg5kFTMbW7JxWJjtPUGZfHbC8vFp-sS8IvqcQ0OEjJcLbRh8BJobciJz8sdFsbWapnEf4YZiXGSBCHKod4AS0yf-rfAYA1eZu2kM4LERBn0eYtY3JS7d59x6ZKJLePfG_UGLX6t-dDmW5anux1HYtAPF9ua2m8puov0bf_xjMlkjZnT_5zZxKwrX0c4kqAoqjRtlr269i17zZ-1jFmpocrDtIbszO6N6w7Rkx_WUG5YkHUg2JaeE42UokdNpogAxaPfIO_VYVoy5qp2OSF6Tj5RNHiSOByGYwQmpyy1FKQo_tOKjxhRirM50BUGB-sf7GORzsqXifOeiOqmzBmqRJU_XgBrM_JmI-SBeTw98r-8P0p5YVcnsjKp-Y4oKdy5NAW4-uc_QiMHgJOkBFdkRqvtGW_a6CZrA7PSk8h_ZJY4dCHyYHiHYfx2j7Qc9ppOK8nteAJP2qnoVNcRk_kZHUg8O4zm483UhvGT_u8I4SgFKfB8uOa9t1y7s1ChzQsDX9MzY0MkqtnqHJfTASE5t11NcofPzMEc8hR6J2voAPhqNdZT3-u_wkb9C9CFJgpOCEgw4w03Kd2cxO0iNRw3nnN4Zjgia2Ela-Smo6BpeIRunnOoV_65L3LTJma3nn4YHU4E-gBpAUuf4XC86Pm_g_QwkrHC6Sx5ZHE_6PI21gGjlDiLfhl2J0Blb3m1gImvciCJ_tfvVq_jiJPe53IITnR2ecIv45f3tl0RY4VWztdL9v5CWxqCQ2eJWF2BRyf4aJ2k9I6JFbIN2rg29"

        [[signatures]]
        Ed25519 = "uhPLHRi9z8fMZbR3f97Gft8QWOyF6nrjwmlLc3DBRCZTNSuadLtFpIwLS9d9IzItRgqAC52M_feoD4F0_IGJJw"
        
        [[signatures]]
        Dsa44 = "MtUwaCFjag7vPW7it-kqBbiy87PpEoy9XL-T0L5y8R426XyY8Dy9riE0kHPdWzdLRGG217hVApc3I50Br1hcHbG2GOMEdd585sNawp48PLxohZckgFZmvx7OEc4krdMAxzHefqp3lxJ1rvpY5KJOtMk-5jWKgFfc2CuYiJ6YCGTIgz0vy8tOqwLgMdu8hRqHEvbv2jZkf6bt-bOCtf-775cT49009tKe_9MFlmUvucShWzjC8HLAG5wuINzEXKC8rM2eruPN5lFrT8t177Rpp5F01B16xXMYmCpRTeWMXgCIWRB6wgz71hjf_kpeQ87xpT9ZZNVfsMUzk9GGHkoA9FA90WeAdn1CxcYgsw50MnCqf2FU-dRmdOECN9C4G8-cnZuBN7quyg3OQjTpHBAW9-DLJ-oaElM4OsLyK8ojWGFzGCddcp1sTmEP1ZLPbW_99n4RJV_R-Qa6bICWDXMU7QDCJYIn2npIbjytvH3n6bd017r98qprpkjn2tfg5VjXtMH8hDn4tbNpAVSpVPjqlhAuqCrvZJkiY1568ex3_IYi0XiEr5RZBmQmAHXT9OleGame5oW_h0ujq4ZX5toaPxKcPUnMDiNl1VMsYi3HxFcQYL8IZ4wbEcVi66or8aOXPyuLrphu8gv-yXYBc2gpdiG04E4x6C__Cnu2_Zd2cuZP41l02My9ppuylYGhItWYwEq_3Anuh6nyacw4DyI7ESMXjXf6I-K5ROkhN4kgyrfsp-LJSah5OQLYg7H-xpMgilxPH-vjtwWc5NLbUvm2VKrfg5VGPKkt6KfPPj3X4PLQjq1EwsKSsZMc5b1kpqyvSt8BvDL2nkNDdWJ9_3-62-_gU1TK7skSFAfm8BqHblUO2qwolfdzcenarWFFWkw-lAxwglUkTmPrQpGAmwmHt46gVib_l78uXPbktmNov-xiqAZMQMryREl6tz3SOVyfsFlvAp3TSQo0OHisTZbZZhD8KwnE5SrX07CLlyGllfogntT_fzW0cV14kU6KQA7MSBQvbdk8hA7SWUemdxzi9vKQ3T-YcB98PkIoYzf1z-SbZfKPeRrxU84ARZc8vk-wFD1atQr1Gcoa2ybfRqIDFZk3BCjcId6UJTc48C8Ej9ubg2velebLHyllLlVGdgBfAPMh95v_f4LL6I9Hf84osNdSbFIafer3wVOGuJl9cPUiKRM2J4vYWMkb0h9ErjrXNqx3umKSVaNdZ7hGL3pC4-hUqGeQDTv81lCLFIFXaVFFWwOuul6pTpSNrgs6GrQH2_FVXbQ_8BH0DmImtJE_aeJy5T8g0NZOjS_g_S8nySBwvOJfNvpwtloXq1CKO4NvfLYebrtaWMdEDggmBZjwcI0dnSNXLv_EGzttG8xvw_ZtDw6y8CQsmU38jMEG4xCk-kmO93JEgGsU_1FbJc4Y64SjIun564k7zNiAMKellOaYwGiigoJi8MZs91UgraIONtSKoGqWIMXOjp8ba4JdxwCBxRNtedywifgVghKRaXdRMcUc0OOWMiAa2ynD0vZlmJSWrNW6maegKKU_nQoKnVeTbZ5SPpiZn5anKV2sjB9MmFzMEc-TfMJWUS-151O6O2PpMh5yJhtaVPMPOSfAIssfpDgP0L4SHUKPuvyJhKkarJPM8YQjxKM9Wfm00baB79YiZaxwLQeOgKrSKB-Mz485a7VI-P1dMDvXeoRIlJ4jtImzFwWcXwBQMYiYbvTD2VKX463GDy-jURBa8YneceO-EXCS_xSH_oQPdoyspvWVJJv-qcGjIrnVqR0fsKJPhAh6hyJVMPS0K8K8-LXO9TphVKS9MZwFdMBm_M_uAHBrwWHUsxfPOAaNa1QoIM_ryYipJcg8bkTNXz_8GSXz2aNsRiRLgIKVkwEUtcLGEe78DvyUIiBLaKQNnhMoqZQ0SzjabXJqxGrAkGEvNHgDFiI0C70XKlQQESWtvJ62GgyoF-tpmTLgk2nRWT05kPkE0-PGhdiaaE-ksEotcYrKRLSumyw7O1zxlIX2At-sltj4X9VMbUXd5wbJn0ynDYUPRnPIX4rfCV1Rd9BJ8eFYXCL20HJRPWzSDYbs4ThivtHIv6K3dRbp_EPPKxULwwi5F1n7u2Tfc8EVbP7TLtQGVmFG0gkFRkv8Rfv4MPFF87jPdHRTxoRaDcr3OzsEu5vVY_jLy1wZBoyYI751E6EhhWyKdEtddz6dvrOGVl9Qr3ktoIIuAibrXZuN1VDpfUyhXpsBUW1sQMTUl-ssJF1HzRe96F-Fms8T3J03oNtuq8ncBwHpoQHsPCmDTYRZCiH_A4iqmyLImYG6wISk_hQ8n2SAqeNcs19jLsJoErep8hwXP0TX7KV2UQYcy4ErBd3lLOEG9-eQEAhtxNArj3gLzGZ-l-5EpE99Qu0dZy9LYcLd2BADfhEsvLBccdOiYFOo8eCyPVaT6yAnPV58s9b2TKtSkSTzd-88zclyVVaI7Sw0-resI4CeRXMNLI8CGU2OkaEZkvUYjo2CnslANKKolkGm7DChLjf8rRZfS_cSM5EpL88J_PwV3T8LsuhPD7ek4tXl4diijtZV7e4XjbbBxAx6X1d8CxeUXGDTJgOJ-rL7swZpNWx1h1n_AARJg5RidTezigpJBm4KmvDM-OJ6rla4Yt_C5eMp6Hgs4VKWRhn4G4fn9Dd6Oa7N-_HpAhL90UnIpfO_ldutzYp9H38bG37Xop1iYtzyr-83z_fDub7YY-pBcIcKYJN7Jabr5nGRz1pyFf0i3Q800PWNlkOZrjqAZL0oyv-TkpEv9iMD_mIVxtL9vxkI2UCo2yZN8T27SVsJfdkkgj3Q5MVDBTSBl25abiCdX44YXHnGG5mLvorMLkVn8mXYMIAiLw1G09awmVdBzz369suFlhqjfpDFNkB_xa1B3-DtRtGIeDXALCW6cuD_GJhhlc4M0BnEn25JzMJGqKZBWJ9ANJtqvcdNSyr031oCzza3dDJCWthRGtat1C2QHJJLYK3MFksp0nyxudse2ARNPLlqb9gLsygIvalFao_aDfks3itOA4VsOuhA5Oetk6NctrYAFySaJBXLLx1kCxwJfkNzCFZJFvqzdRgt0uZdE9lYzjuXrgmAzv5aysj5Kzgl5vnNIyXVs1n0fC3vU3hP117P2zPBialpm9qAQGjw8XSYa0Yg17u8cBLxVLOYa5m3DpjdZg0GtG2l0qvpN3UJU5JaC31NS-wxGI2RKco"
        "#).unwrap();

        println!("{:?}", body);

        let expected = BASE64_STANDARD.decode("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").unwrap();

        let mut context = ResponseSerializationContext {
            header: &header,
            packet: &base,
            config: SerializerConfig::new(),
        };

        let bytes = body.to_bytes(&mut context).unwrap();
        assert_eq!(expected, bytes);

        let mut context = ResponseSerializationContext {
            header: &header,
            packet: &base,
            config: SerializerConfig::new(),
        };

        let deserialized = SessionResponseBody::from_bytes(&bytes, &mut context).unwrap();
        assert_eq!(body, deserialized);
    }

    #[test]
    fn can_serde_toml_session_request_packet() {
        let toml = r#"
        version = 1
        use_encryption = false
        mac = "6k0bdANb3Q2TkCIioHE71A"

        [header]
        packet_type = "Session"
        persist_key = true
        
        [body]
        psk_expiration = [1,2,3,4]

        [[body.keys]]
        X25519 = "si6IcNvysw_Ex8D9Z1Q0LFi1vNrvfA3lAhfwy2_Hw24"
        "#;

        let request: PlabbleRequestPacket = toml::from_str(toml).unwrap();
        let serialized = toml::to_string(&request).unwrap();
        let deserialized: PlabbleRequestPacket = toml::from_str(&serialized).unwrap();
        assert_eq!(request, deserialized);
    }

    fn get_base() -> PlabblePacketBase {
        toml::from_str(
            r#"
        version = 0
        use_encryption = true
        specify_crypto_settings = true
        crypto_settings.use_post_quantum = true
        
        [crypto_settings.post_quantum_settings]
        sign_pqc_dsa_44 = true
        key_exchange_pqc_kem_512 = true
        "#,
        )
        .unwrap()
    }

    fn get_context() -> (PlabblePacketBase, PlabbleRequestHeader) {
        let base = get_base();

        // [header]
        let header: PlabbleRequestHeader = toml::from_str(
            r#"
        packet_type = "Session"
        persist_key = true
        "#,
        )
        .unwrap();

        (base, header)
    }
}
