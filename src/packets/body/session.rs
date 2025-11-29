use binary_codec::{BinarySerializer, DeserializationError, SerializationError, utils::slice};
use serde::{Deserialize, Serialize};
use serde_with::base64::{Base64, UrlSafe};
use serde_with::formats::Unpadded;
use serde_with::serde_as;

use crate::packets::base::crypto_keys::CryptoSignature;
use crate::packets::body::SerializableResponseBody;
use crate::packets::header::type_and_flags::ResponsePacketType;
use crate::packets::{
    base::crypto_keys::CryptoKey, body::SerializableRequestBody,
    header::type_and_flags::RequestPacketType,
};

/// Session request body
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SessionRequestBody {
    /// PSK expiration Plabble timestamp. Filled if request flag persist_key is set.
    pub psk_expiration: Option<[u8; 4]>,

    /// Public/encapsulation keys for creating a shared secret with the server
    pub keys: Vec<CryptoKey>,
}

impl SerializableRequestBody for SessionRequestBody {
    fn to_bytes(
        &self,
        context: &mut super::RequestSerializationContext,
    ) -> Result<Vec<u8>, SerializationError> {
        let mut bytes: Vec<u8> = Vec::new();
        if let RequestPacketType::Session { persist_key, .. } = context.header.packet_type {
            if persist_key && self.psk_expiration.is_none() {
                return Err(SerializationError::InvalidData(String::from(
                    "psk_expiration should be set if persist_key flag is set",
                )));
            }

            if let Some(expiration_bytes) = self.psk_expiration {
                bytes.extend_from_slice(&expiration_bytes);
                context.config.pos += expiration_bytes.len();
            }
        } else {
            return Err(SerializationError::InvalidData(String::from(
                "Header type did not match body",
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
        if let RequestPacketType::Session { persist_key, .. } = context.header.packet_type {
            let psk_expiration = if persist_key {
                Some(
                    slice(&mut context.config, bytes, 4, true)?
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
                keys,
            })
        } else {
            Err(DeserializationError::InvalidData(String::from(
                "Header type did not match body",
            )))
        }
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SessionResponseBody {
    #[serde_as(as = "Option<Base64<UrlSafe, Unpadded>>")]
    psk_id: Option<[u8; 12]>,

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
        if let ResponsePacketType::Session { with_psk } = context.header.packet_type {
            if with_psk && self.psk_id.is_none() {
                return Err(SerializationError::InvalidData(String::from(
                    "psk_id should be set if with_psk flag is set",
                )));
            }

            if let Some(psk_id_bytes) = self.psk_id {
                bytes.extend_from_slice(&psk_id_bytes);
                context.config.pos += psk_id_bytes.len();
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
        if let ResponsePacketType::Session { with_psk } = context.header.packet_type {
            let psk_id = if with_psk {
                Some(
                    slice(&mut context.config, bytes, 12, true)?
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

    use binary_codec::SerializerConfig;

    use crate::packets::{
        base::PlabblePacketBase,
        body::{
            RequestSerializationContext, ResponseSerializationContext, SerializableRequestBody,
            SerializableResponseBody,
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

        let expected = vec![
            204u8, 153, 102, 196, 195, 106, 130, 19, 194, 155, 102, 50, 90, 107, 199, 85, 113, 58,
            45, 136, 157, 88, 239, 255, 169, 216, 102, 252, 175, 191, 211, 59, 76, 154, 115, 40,
            150, 3, 77, 224, 111, 187, 23, 131, 143, 13, 64, 79, 175, 135, 246, 174, 47, 255, 82,
            106, 19, 61, 11, 148, 230, 35, 213, 159, 37, 220, 16, 200, 249, 151, 56, 149, 237, 249,
            116, 164, 210, 108, 27, 49, 225, 73, 193, 32, 243, 254, 79, 115, 175, 12, 95, 175, 5,
            1, 137, 225, 24, 219, 181, 39, 193, 247, 48, 88, 95, 140, 201, 33, 177, 239, 69, 73,
            32, 63, 118, 129, 134, 252, 37, 168, 140, 129, 252, 67, 239, 173, 45, 221, 11, 132, 22,
            5, 182, 227, 56, 77, 19, 83, 167, 151, 184, 37, 81, 127, 161, 243, 200, 82, 114, 78,
            254, 60, 23, 41, 212, 150, 57, 211, 54, 127, 166, 166, 100, 252, 201, 21, 93, 46, 171,
            216, 249, 172, 130, 22, 212, 115, 246, 238, 136, 158, 140, 155, 67, 221, 86, 61, 159,
            81, 14, 133, 145, 143, 200, 118, 24, 45, 83, 91, 8, 199, 242, 84, 230, 121, 119, 134,
            34, 99, 204, 138, 171, 226, 17, 41, 116, 79, 212, 84, 13, 52, 181, 201, 123, 128, 130,
            14, 100, 21, 51, 27, 91, 178, 113, 88, 152, 237, 61, 65, 153, 124, 118, 194, 242, 241,
            105, 250, 196, 188, 34, 250, 156, 67, 67, 132, 140, 151, 11, 109, 24, 124, 4, 154, 27,
            114, 34, 115, 242, 199, 69, 177, 181, 154, 166, 113, 31, 225, 134, 98, 92, 100, 129, 8,
            114, 168, 119, 128, 18, 211, 39, 254, 173, 240, 24, 3, 87, 153, 187, 105, 12, 224, 177,
            17, 6, 125, 30, 98, 214, 55, 37, 46, 221, 231, 220, 122, 100, 162, 75, 120, 247, 198,
            253, 65, 139, 95, 171, 126, 116, 57, 150, 229, 169, 238, 199, 81, 216, 180, 3, 197,
            246, 230, 182, 155, 202, 110, 162, 253, 27, 127, 252, 99, 50, 89, 35, 102, 116, 255,
            231, 54, 113, 43, 10, 215, 209, 206, 36, 168, 10, 42, 141, 27, 101, 175, 110, 189, 139,
            94, 243, 103, 237, 99, 22, 106, 104, 114, 176, 237, 33, 187, 51, 59, 163, 122, 195,
            180, 100, 199, 245, 148, 27, 150, 36, 29, 72, 54, 37, 167, 132, 227, 101, 40, 145, 211,
            105, 162, 0, 49, 104, 247, 200, 59, 245, 88, 86, 140, 185, 170, 157, 142, 72, 94, 147,
            143, 148, 77, 30, 36, 142, 7, 33, 152, 193, 9, 169, 203, 45, 69, 41, 10, 63, 180, 226,
            163, 198, 20, 98, 172, 206, 116, 5, 65, 129, 250, 199, 251, 24, 228, 115, 178, 165,
            226, 124, 231, 162, 58, 169, 179, 6, 106, 145, 37, 79, 215, 128, 26, 204, 252, 153,
            136, 249, 32, 94, 79, 15, 124, 175, 239, 15, 210, 158, 88, 85, 201, 236, 140, 170, 126,
            99, 138, 10, 119, 46, 77, 1, 110, 62, 185, 207, 208, 136, 193, 224, 36, 233, 1, 21,
            217, 17, 170, 251, 70, 91, 246, 186, 9, 154, 192, 236, 244, 164, 242, 31, 217, 37, 142,
            29, 8, 124, 152, 30, 33, 216, 127, 29, 163, 237, 7, 61, 166, 147, 138, 242, 123, 94, 0,
            147, 246, 170, 122, 21, 53, 196, 100, 254, 70, 71, 82, 15, 14, 227, 57, 184, 243, 117,
            33, 188, 100, 255, 187, 194, 56, 74, 1, 74, 124, 31, 46, 57, 175, 109, 215, 46, 236,
            212, 40, 115, 66, 192, 215, 244, 204, 216, 208, 201, 42, 182, 122, 135, 37, 244, 192,
            72, 78, 109, 215, 83, 92, 161, 243, 243, 48, 71, 60, 133, 30, 137, 218, 250, 0, 62, 26,
            141, 117, 148, 247, 250, 239, 240, 145, 191, 66, 244, 33, 73, 130, 147, 130, 18, 12,
            56, 195, 77, 202, 119, 103, 49, 59, 72, 141, 71, 13, 231, 156, 222, 25, 142, 8, 154,
            216, 73, 90, 249, 41, 168, 232, 26, 94, 33, 27, 167, 156, 234, 21, 255, 174, 75, 220,
            180, 201, 153, 173, 231, 159, 134, 7, 83, 129, 62, 128, 26, 64, 82, 231, 248, 92, 47,
            58, 62, 111, 224, 253, 12, 36, 172, 112, 186, 75, 30, 89, 28, 79, 250, 60, 141, 181,
            128, 104, 229, 14, 34, 223, 134, 93, 137, 208, 25, 91, 222, 109, 96, 34, 107, 220, 136,
            34, 127, 181, 251, 213, 171, 248, 226, 36, 247, 185, 220, 130, 19, 157, 29, 158, 112,
            139, 248, 229, 253, 237, 151, 68, 88, 225, 85, 179, 181, 210, 253, 191, 144, 150, 198,
            160, 144, 217, 226, 86, 23, 96, 81, 201, 254, 26, 39, 105, 61, 35, 162, 69, 108, 131,
            118, 174, 13, 189, 186, 19, 203, 29, 24, 189, 207, 199, 204, 101, 180, 119, 127, 222,
            198, 126, 223, 16, 88, 236, 133, 234, 122, 227, 194, 105, 75, 115, 112, 193, 68, 38,
            83, 53, 43, 154, 116, 187, 69, 164, 140, 11, 75, 215, 125, 35, 50, 45, 70, 10, 128, 11,
            157, 140, 253, 247, 168, 15, 129, 116, 252, 129, 137, 39, 50, 213, 48, 104, 33, 99,
            106, 14, 239, 61, 110, 226, 183, 233, 42, 5, 184, 178, 243, 179, 233, 18, 140, 189, 92,
            191, 147, 208, 190, 114, 241, 30, 54, 233, 124, 152, 240, 60, 189, 174, 33, 52, 144,
            115, 221, 91, 55, 75, 68, 97, 182, 215, 184, 85, 2, 151, 55, 35, 157, 1, 175, 88, 92,
            29, 177, 182, 24, 227, 4, 117, 222, 124, 230, 195, 90, 194, 158, 60, 60, 188, 104, 133,
            151, 36, 128, 86, 102, 191, 30, 206, 17, 206, 36, 173, 211, 0, 199, 49, 222, 126, 170,
            119, 151, 18, 117, 174, 250, 88, 228, 162, 78, 180, 201, 62, 230, 53, 138, 128, 87,
            220, 216, 43, 152, 136, 158, 152, 8, 100, 200, 131, 61, 47, 203, 203, 78, 171, 2, 224,
            49, 219, 188, 133, 26, 135, 18, 246, 239, 218, 54, 100, 127, 166, 237, 249, 179, 130,
            181, 255, 187, 239, 151, 19, 227, 221, 52, 246, 210, 158, 255, 211, 5, 150, 101, 47,
            185, 196, 161, 91, 56, 194, 240, 114, 192, 27, 156, 46, 32, 220, 196, 92, 160, 188,
            172, 205, 158, 174, 227, 205, 230, 81, 107, 79, 203, 117, 239, 180, 105, 167, 145, 116,
            212, 29, 122, 197, 115, 24, 152, 42, 81, 77, 229, 140, 94, 0, 136, 89, 16, 122, 194,
            12, 251, 214, 24, 223, 254, 74, 94, 67, 206, 241, 165, 63, 89, 100, 213, 95, 176, 197,
            51, 147, 209, 134, 30, 74, 0, 244, 80, 61, 209, 103, 128, 118, 125, 66, 197, 198, 32,
            179, 14, 116, 50, 112, 170, 127, 97, 84, 249, 212, 102, 116, 225, 2, 55, 208, 184, 27,
            207, 156, 157, 155, 129, 55, 186, 174, 202, 13, 206, 66, 52, 233, 28, 16, 22, 247, 224,
            203, 39, 234, 26, 18, 83, 56, 58, 194, 242, 43, 202, 35, 88, 97, 115, 24, 39, 93, 114,
            157, 108, 78, 97, 15, 213, 146, 207, 109, 111, 253, 246, 126, 17, 37, 95, 209, 249, 6,
            186, 108, 128, 150, 13, 115, 20, 237, 0, 194, 37, 130, 39, 218, 122, 72, 110, 60, 173,
            188, 125, 231, 233, 183, 116, 215, 186, 253, 242, 170, 107, 166, 72, 231, 218, 215,
            224, 229, 88, 215, 180, 193, 252, 132, 57, 248, 181, 179, 105, 1, 84, 169, 84, 248,
            234, 150, 16, 46, 168, 42, 239, 100, 153, 34, 99, 94, 122, 241, 236, 119, 252, 134, 34,
            209, 120, 132, 175, 148, 89, 6, 100, 38, 0, 117, 211, 244, 233, 94, 25, 169, 158, 230,
            133, 191, 135, 75, 163, 171, 134, 87, 230, 218, 26, 63, 18, 156, 61, 73, 204, 14, 35,
            101, 213, 83, 44, 98, 45, 199, 196, 87, 16, 96, 191, 8, 103, 140, 27, 17, 197, 98, 235,
            170, 43, 241, 163, 151, 63, 43, 139, 174, 152, 110, 242, 11, 254, 201, 118, 1, 115,
            104, 41, 118, 33, 180, 224, 78, 49, 232, 47, 255, 10, 123, 182, 253, 151, 118, 114,
            230, 79, 227, 89, 116, 216, 204, 189, 166, 155, 178, 149, 129, 161, 34, 213, 152, 192,
            74, 191, 220, 9, 238, 135, 169, 242, 105, 204, 56, 15, 34, 59, 17, 35, 23, 141, 119,
            250, 35, 226, 185, 68, 233, 33, 55, 137, 32, 202, 183, 236, 167, 226, 201, 73, 168,
            121, 57, 2, 216, 131, 177, 254, 198, 147, 32, 138, 92, 79, 31, 235, 227, 183, 5, 156,
            228, 210, 219, 82, 249, 182, 84, 170, 223, 131, 149, 70, 60, 169, 45, 232, 167, 207,
            62, 61, 215, 224, 242, 208, 142, 173, 68, 194, 194, 146, 177, 147, 28, 229, 189, 100,
            166, 172, 175, 74, 223, 1, 188, 50, 246, 158, 67, 67, 117, 98, 125, 255, 127, 186, 219,
            239, 224, 83, 84, 202, 238, 201, 18, 20, 7, 230, 240, 26, 135, 110, 85, 14, 218, 172,
            40, 149, 247, 115, 113, 233, 218, 173, 97, 69, 90, 76, 62, 148, 12, 112, 130, 85, 36,
            78, 99, 235, 66, 145, 128, 155, 9, 135, 183, 142, 160, 86, 38, 255, 151, 191, 46, 92,
            246, 228, 182, 99, 104, 191, 236, 98, 168, 6, 76, 64, 202, 242, 68, 73, 122, 183, 61,
            210, 57, 92, 159, 176, 89, 111, 2, 157, 211, 73, 10, 52, 56, 120, 172, 77, 150, 217,
            102, 16, 252, 43, 9, 196, 229, 42, 215, 211, 176, 139, 151, 33, 165, 149, 250, 32, 158,
            212, 255, 127, 53, 180, 113, 93, 120, 145, 78, 138, 64, 14, 204, 72, 20, 47, 109, 217,
            60, 132, 14, 210, 89, 71, 166, 119, 28, 226, 246, 242, 144, 221, 63, 152, 112, 31, 124,
            62, 66, 40, 99, 55, 245, 207, 228, 155, 101, 242, 143, 121, 26, 241, 83, 206, 0, 69,
            151, 60, 190, 79, 176, 20, 61, 90, 181, 10, 245, 25, 202, 26, 219, 38, 223, 70, 162, 3,
            21, 153, 55, 4, 40, 220, 33, 222, 148, 37, 55, 56, 240, 47, 4, 143, 219, 155, 131, 107,
            222, 149, 230, 203, 31, 41, 101, 46, 85, 70, 118, 0, 95, 0, 243, 33, 247, 155, 255,
            127, 130, 203, 232, 143, 71, 127, 206, 40, 176, 215, 82, 108, 82, 26, 125, 234, 247,
            193, 83, 134, 184, 153, 125, 112, 245, 34, 41, 19, 54, 39, 139, 216, 88, 201, 27, 210,
            31, 68, 174, 58, 215, 54, 172, 119, 186, 98, 146, 85, 163, 93, 103, 184, 70, 47, 122,
            66, 227, 232, 84, 168, 103, 144, 13, 59, 252, 214, 80, 139, 20, 129, 87, 105, 81, 69,
            91, 3, 174, 186, 94, 169, 78, 148, 141, 174, 11, 58, 26, 180, 7, 219, 241, 85, 93, 180,
            63, 240, 17, 244, 14, 98, 38, 180, 145, 63, 105, 226, 114, 229, 63, 32, 208, 214, 78,
            141, 47, 224, 253, 47, 39, 201, 32, 112, 188, 226, 95, 54, 250, 112, 182, 90, 23, 171,
            80, 138, 59, 131, 111, 124, 182, 30, 110, 187, 90, 88, 199, 68, 14, 8, 38, 5, 152, 240,
            112, 141, 29, 157, 35, 87, 46, 255, 196, 27, 59, 109, 27, 204, 111, 195, 246, 109, 15,
            14, 178, 240, 36, 44, 153, 77, 252, 140, 193, 6, 227, 16, 164, 250, 73, 142, 247, 114,
            68, 128, 107, 20, 255, 81, 91, 37, 206, 24, 235, 132, 163, 34, 233, 249, 235, 137, 59,
            204, 216, 128, 48, 167, 165, 148, 230, 152, 192, 104, 162, 130, 130, 98, 240, 198, 108,
            247, 85, 32, 173, 162, 14, 54, 212, 138, 160, 106, 150, 32, 197, 206, 142, 159, 27,
            107, 130, 93, 199, 0, 129, 197, 19, 109, 121, 220, 176, 137, 248, 21, 130, 18, 145,
            105, 119, 81, 49, 197, 28, 208, 227, 150, 50, 32, 26, 219, 41, 195, 210, 246, 101, 152,
            148, 150, 172, 213, 186, 153, 167, 160, 40, 165, 63, 157, 10, 10, 157, 87, 147, 109,
            158, 82, 62, 152, 153, 159, 150, 167, 41, 93, 172, 140, 31, 76, 152, 92, 204, 17, 207,
            147, 124, 194, 86, 81, 47, 181, 231, 83, 186, 59, 99, 233, 50, 30, 114, 38, 27, 90, 84,
            243, 15, 57, 39, 192, 34, 203, 31, 164, 56, 15, 208, 190, 18, 29, 66, 143, 186, 252,
            137, 132, 169, 26, 172, 147, 204, 241, 132, 35, 196, 163, 61, 89, 249, 180, 209, 182,
            129, 239, 214, 34, 101, 172, 112, 45, 7, 142, 128, 170, 210, 40, 31, 140, 207, 143, 57,
            107, 181, 72, 248, 253, 93, 48, 59, 215, 122, 132, 72, 148, 158, 35, 180, 137, 179, 23,
            5, 156, 95, 0, 80, 49, 136, 152, 110, 244, 195, 217, 82, 151, 227, 173, 198, 15, 47,
            163, 81, 16, 90, 241, 137, 222, 113, 227, 190, 17, 112, 146, 255, 20, 135, 254, 132,
            15, 118, 140, 172, 166, 245, 149, 36, 155, 254, 169, 193, 163, 34, 185, 213, 169, 29,
            31, 176, 162, 79, 132, 8, 122, 135, 34, 85, 48, 244, 180, 43, 194, 188, 248, 181, 206,
            245, 58, 97, 84, 164, 189, 49, 156, 5, 116, 192, 102, 252, 207, 238, 0, 112, 107, 193,
            97, 212, 179, 23, 207, 56, 6, 141, 107, 84, 40, 32, 207, 235, 201, 136, 169, 37, 200,
            60, 110, 68, 205, 95, 63, 252, 25, 37, 243, 217, 163, 108, 70, 36, 75, 128, 130, 149,
            147, 1, 20, 181, 194, 198, 17, 238, 252, 14, 252, 148, 34, 32, 75, 104, 164, 13, 158,
            19, 40, 169, 148, 52, 75, 56, 218, 109, 114, 106, 196, 106, 192, 144, 97, 47, 52, 120,
            3, 22, 34, 52, 11, 189, 23, 42, 84, 16, 17, 37, 173, 188, 158, 182, 26, 12, 168, 23,
            235, 105, 153, 50, 224, 147, 105, 209, 89, 61, 57, 144, 249, 4, 211, 227, 198, 133,
            216, 154, 104, 79, 164, 176, 74, 45, 113, 138, 202, 68, 180, 174, 155, 44, 59, 59, 92,
            241, 148, 133, 246, 2, 223, 172, 150, 216, 248, 95, 213, 76, 109, 69, 221, 231, 6, 201,
            159, 76, 167, 13, 133, 15, 70, 115, 200, 95, 138, 223, 9, 93, 81, 119, 208, 73, 241,
            225, 88, 92, 34, 246, 208, 114, 81, 61, 108, 210, 13, 134, 236, 225, 56, 98, 190, 209,
            200, 191, 162, 183, 117, 22, 233, 252, 67, 207, 43, 21, 11, 195, 8, 185, 23, 89, 251,
            187, 100, 223, 115, 193, 21, 108, 254, 211, 46, 212, 6, 86, 97, 70, 210, 9, 5, 70, 75,
            252, 69, 251, 248, 48, 241, 69, 243, 184, 207, 116, 116, 83, 198, 132, 90, 13, 202,
            247, 59, 59, 4, 187, 155, 213, 99, 248, 203, 203, 92, 25, 6, 140, 152, 35, 190, 117,
            19, 161, 33, 133, 108, 138, 116, 75, 93, 119, 62, 157, 190, 179, 134, 86, 95, 80, 175,
            121, 45, 160, 130, 46, 2, 38, 235, 93, 155, 141, 213, 80, 233, 125, 76, 161, 94, 155,
            1, 81, 109, 108, 64, 196, 212, 151, 235, 44, 36, 93, 71, 205, 23, 189, 232, 95, 133,
            154, 207, 19, 220, 157, 55, 160, 219, 110, 171, 201, 220, 7, 1, 233, 161, 1, 236, 60,
            41, 131, 77, 132, 89, 10, 33, 255, 3, 136, 170, 155, 34, 200, 153, 129, 186, 192, 132,
            164, 254, 20, 60, 159, 100, 128, 169, 227, 92, 179, 95, 99, 46, 194, 104, 18, 183, 169,
            242, 28, 23, 63, 68, 215, 236, 165, 118, 81, 6, 28, 203, 129, 43, 5, 221, 229, 44, 225,
            6, 247, 231, 144, 16, 8, 109, 196, 208, 43, 143, 120, 11, 204, 102, 126, 151, 238, 68,
            164, 79, 125, 66, 237, 29, 103, 47, 75, 97, 194, 221, 216, 16, 3, 126, 17, 44, 188,
            176, 92, 113, 211, 162, 96, 83, 168, 241, 224, 178, 61, 86, 147, 235, 32, 39, 61, 94,
            124, 179, 214, 246, 76, 171, 82, 145, 36, 243, 119, 239, 60, 205, 201, 114, 85, 86,
            136, 237, 44, 52, 250, 183, 172, 35, 128, 158, 69, 115, 13, 44, 143, 2, 25, 77, 142,
            145, 161, 25, 146, 245, 24, 142, 141, 130, 158, 201, 64, 52, 162, 168, 150, 65, 166,
            236, 48, 161, 46, 55, 252, 173, 22, 95, 75, 247, 18, 51, 145, 41, 47, 207, 9, 252, 252,
            21, 221, 63, 11, 178, 232, 79, 15, 183, 164, 226, 213, 229, 225, 216, 162, 142, 214,
            85, 237, 238, 23, 141, 182, 193, 196, 12, 122, 95, 87, 124, 11, 23, 148, 92, 96, 211,
            38, 3, 137, 250, 178, 251, 179, 6, 105, 53, 108, 117, 135, 89, 255, 0, 4, 73, 131, 148,
            98, 117, 55, 179, 138, 10, 73, 6, 110, 10, 154, 240, 204, 248, 226, 122, 174, 86, 184,
            98, 223, 194, 229, 227, 41, 232, 120, 44, 225, 82, 150, 70, 25, 248, 27, 135, 231, 244,
            55, 122, 57, 174, 205, 251, 241, 233, 2, 18, 253, 209, 73, 200, 165, 243, 191, 149,
            219, 173, 205, 138, 125, 31, 127, 27, 27, 126, 215, 162, 157, 98, 98, 220, 242, 175,
            239, 55, 207, 247, 195, 185, 190, 216, 99, 234, 65, 112, 135, 10, 96, 147, 123, 37,
            166, 235, 230, 113, 145, 207, 90, 114, 21, 253, 34, 221, 15, 52, 208, 245, 141, 150,
            67, 153, 174, 58, 128, 100, 189, 40, 202, 255, 147, 146, 145, 47, 246, 35, 3, 254, 98,
            21, 198, 210, 253, 191, 25, 8, 217, 64, 168, 219, 38, 77, 241, 61, 187, 73, 91, 9, 125,
            217, 36, 130, 61, 208, 228, 197, 67, 5, 52, 129, 151, 110, 90, 110, 32, 157, 95, 142,
            24, 92, 121, 198, 27, 153, 139, 190, 138, 204, 46, 69, 103, 242, 101, 216, 48, 128, 34,
            47, 13, 70, 211, 214, 176, 153, 87, 65, 207, 61, 250, 246, 203, 133, 150, 26, 163, 126,
            144, 197, 54, 64, 127, 197, 173, 65, 223, 224, 237, 70, 209, 136, 120, 53, 192, 44, 37,
            186, 114, 224, 255, 24, 152, 97, 149, 206, 12, 208, 25, 196, 159, 110, 73, 204, 194,
            70, 168, 166, 65, 88, 159, 64, 52, 155, 106, 189, 199, 77, 75, 42, 244, 223, 90, 2,
            207, 54, 183, 116, 50, 66, 90, 216, 81, 26, 214, 173, 212, 45, 144, 28, 146, 75, 96,
            173, 204, 22, 75, 41, 210, 124, 177, 185, 219, 30, 216, 4, 77, 60, 185, 106, 111, 216,
            11, 179, 40, 8, 189, 169, 69, 106, 143, 218, 13, 249, 44, 222, 43, 78, 3, 133, 108, 58,
            232, 64, 228, 231, 173, 147, 163, 92, 182, 182, 0, 23, 36, 154, 36, 21, 203, 47, 29,
            100, 11, 28, 9, 126, 67, 115, 8, 86, 73, 22, 250, 179, 117, 24, 45, 210, 230, 93, 19,
            217, 88, 206, 59, 151, 174, 9, 128, 206, 254, 90, 202, 200, 249, 43, 56, 37, 230, 249,
            205, 35, 37, 213, 179, 89, 244, 124, 45, 239, 83, 120, 79, 215, 94, 207, 219, 51, 193,
            137, 169, 105, 155, 218, 128, 64, 104, 240, 241, 116, 152, 107, 70, 32, 215, 187, 188,
            112, 18, 241, 84, 179, 152, 107, 153, 183, 14, 152, 221, 102, 13, 6, 180, 109, 165,
            210, 171, 233, 55, 117, 9, 83, 146, 90, 11, 125, 77, 75, 236, 49, 24, 141, 145, 41,
            202,
        ];

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
