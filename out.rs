#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
pub mod packets {
    pub mod base {
        use binary_codec::{FromBytes, ToBytes};
        use serde::{Deserialize, Serialize};
        use serde_with::serde_as;
        use serde_with::base64::{Base64, UrlSafe};
        use serde_with::formats::Unpadded;
        use settings::CryptoSettings;
        pub mod settings {
            use crate::default_true;
            use binary_codec::{FromBytes, ToBytes};
            use serde::{Deserialize, Serialize};
            pub struct CryptoSettings {
                /// If true, encrypt with ChaCha20 (Poly1305).
                /// This is the default if no encryption settings are specified.
                #[serde(default = "default_true")]
                pub encrypt_with_cha_cha20: bool,
                /// If true, encrypt with AES-CTR and AES-GCM.
                #[serde(default)]
                pub encrypt_with_aes: bool,
                /// Use 32-byte hashes instead of 16-byte ones.
                #[serde(default)]
                pub larger_hashes: bool,
                /// Use Blake3 for hashing, MAC and key derivation instead of Blake2.
                #[serde(default)]
                pub use_blake3: bool,
                /// Sign with Ed25519 (default), 32 B keys, signature 64 B.
                #[serde(default = "default_true")]
                pub sign_ed25519: bool,
                /// Key exchange with X25519 (default), 32 B keys.
                #[serde(default = "default_true")]
                pub key_exchange_x25519: bool,
                /// Reserved for future use
                #[serde(default)]
                flag_64: bool,
                /// Use post-quantum cryptography (e.g., Kyber etc.)
                /// This adds the Post-Quantum settings
                #[serde(default)]
                #[toggles("pqc")]
                pub use_post_quantum: bool,
                /// Post-Quantum settings
                #[toggled_by = "pqc"]
                pub post_quantum_settings: Option<PostQuantumSettings>,
            }
            impl binary_codec::BinaryDeserializer for CryptoSettings {
                fn from_bytes(
                    bytes: &[u8],
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Self, binary_codec::DeserializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = bytes;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let encrypt_with_cha_cha20 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let encrypt_with_aes = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let larger_hashes = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let use_blake3 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let sign_ed25519 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let key_exchange_x25519 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let flag_64 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let use_post_quantum = _p_val;
                    _p_config.set_toggle("pqc", _p_val);
                    let mut __option_0: Option<PostQuantumSettings> = None;
                    if _p_config.get_toggle("pqc").unwrap_or(false) {
                        let _p_val = binary_codec::variable::read_object(
                            _p_bytes,
                            None,
                            _p_config,
                        )?;
                        __option_0 = Some(_p_val);
                    }
                    let _p_val = __option_0;
                    let post_quantum_settings = _p_val;
                    Ok(Self {
                        encrypt_with_cha_cha20,
                        encrypt_with_aes,
                        larger_hashes,
                        use_blake3,
                        sign_ed25519,
                        key_exchange_x25519,
                        flag_64,
                        use_post_quantum,
                        post_quantum_settings,
                    })
                }
            }
            impl binary_codec::BinarySerializer for CryptoSettings {
                fn to_bytes(
                    &self,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Vec<u8>, binary_codec::SerializationError> {
                    let mut bytes = Vec::new();
                    Self::write_bytes(self, &mut bytes, config)?;
                    Ok(bytes)
                }
                fn write_bytes(
                    &self,
                    buffer: &mut Vec<u8>,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<(), binary_codec::SerializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = buffer;
                    let _p_val = &self.encrypt_with_cha_cha20;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.encrypt_with_aes;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.larger_hashes;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.use_blake3;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.sign_ed25519;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.key_exchange_x25519;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.flag_64;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.use_post_quantum;
                    _p_config.set_toggle("pqc", *_p_val);
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.post_quantum_settings;
                    if _p_config.get_toggle("pqc").unwrap_or(false) {
                        let _p_val = _p_val
                            .as_ref()
                            .expect(
                                "Expected Some value, because toggled_by field is true",
                            );
                        binary_codec::variable::write_object(
                            _p_val,
                            None,
                            _p_bytes,
                            _p_config,
                        )?;
                    }
                    Ok(())
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CryptoSettings {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "CryptoSettings",
                            false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "encrypt_with_cha_cha20",
                            &self.encrypt_with_cha_cha20,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "encrypt_with_aes",
                            &self.encrypt_with_aes,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "larger_hashes",
                            &self.larger_hashes,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "use_blake3",
                            &self.use_blake3,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "sign_ed25519",
                            &self.sign_ed25519,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "key_exchange_x25519",
                            &self.key_exchange_x25519,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "flag_64",
                            &self.flag_64,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "use_post_quantum",
                            &self.use_post_quantum,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "post_quantum_settings",
                            &self.post_quantum_settings,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CryptoSettings {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __field8,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field0),
                                    1u64 => _serde::__private225::Ok(__Field::__field1),
                                    2u64 => _serde::__private225::Ok(__Field::__field2),
                                    3u64 => _serde::__private225::Ok(__Field::__field3),
                                    4u64 => _serde::__private225::Ok(__Field::__field4),
                                    5u64 => _serde::__private225::Ok(__Field::__field5),
                                    6u64 => _serde::__private225::Ok(__Field::__field6),
                                    7u64 => _serde::__private225::Ok(__Field::__field7),
                                    8u64 => _serde::__private225::Ok(__Field::__field8),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "encrypt_with_cha_cha20" => {
                                        _serde::__private225::Ok(__Field::__field0)
                                    }
                                    "encrypt_with_aes" => {
                                        _serde::__private225::Ok(__Field::__field1)
                                    }
                                    "larger_hashes" => {
                                        _serde::__private225::Ok(__Field::__field2)
                                    }
                                    "use_blake3" => _serde::__private225::Ok(__Field::__field3),
                                    "sign_ed25519" => {
                                        _serde::__private225::Ok(__Field::__field4)
                                    }
                                    "key_exchange_x25519" => {
                                        _serde::__private225::Ok(__Field::__field5)
                                    }
                                    "flag_64" => _serde::__private225::Ok(__Field::__field6),
                                    "use_post_quantum" => {
                                        _serde::__private225::Ok(__Field::__field7)
                                    }
                                    "post_quantum_settings" => {
                                        _serde::__private225::Ok(__Field::__field8)
                                    }
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"encrypt_with_cha_cha20" => {
                                        _serde::__private225::Ok(__Field::__field0)
                                    }
                                    b"encrypt_with_aes" => {
                                        _serde::__private225::Ok(__Field::__field1)
                                    }
                                    b"larger_hashes" => {
                                        _serde::__private225::Ok(__Field::__field2)
                                    }
                                    b"use_blake3" => _serde::__private225::Ok(__Field::__field3),
                                    b"sign_ed25519" => {
                                        _serde::__private225::Ok(__Field::__field4)
                                    }
                                    b"key_exchange_x25519" => {
                                        _serde::__private225::Ok(__Field::__field5)
                                    }
                                    b"flag_64" => _serde::__private225::Ok(__Field::__field6),
                                    b"use_post_quantum" => {
                                        _serde::__private225::Ok(__Field::__field7)
                                    }
                                    b"post_quantum_settings" => {
                                        _serde::__private225::Ok(__Field::__field8)
                                    }
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private225::PhantomData<CryptoSettings>,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CryptoSettings;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "struct CryptoSettings",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => default_true(),
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field3 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field4 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => default_true(),
                                };
                                let __field5 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => default_true(),
                                };
                                let __field6 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field7 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field8 = match _serde::de::SeqAccess::next_element::<
                                    Option<PostQuantumSettings>,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                8usize,
                                                &"struct CryptoSettings with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private225::Ok(CryptoSettings {
                                    encrypt_with_cha_cha20: __field0,
                                    encrypt_with_aes: __field1,
                                    larger_hashes: __field2,
                                    use_blake3: __field3,
                                    sign_ed25519: __field4,
                                    key_exchange_x25519: __field5,
                                    flag_64: __field6,
                                    use_post_quantum: __field7,
                                    post_quantum_settings: __field8,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field2: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field3: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field4: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field5: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field6: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field7: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field8: _serde::__private225::Option<
                                    Option<PostQuantumSettings>,
                                > = _serde::__private225::None;
                                while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private225::Option::is_some(&__field0) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "encrypt_with_cha_cha20",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private225::Option::is_some(&__field1) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "encrypt_with_aes",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private225::Option::is_some(&__field2) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "larger_hashes",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private225::Option::is_some(&__field3) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "use_blake3",
                                                    ),
                                                );
                                            }
                                            __field3 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::__private225::Option::is_some(&__field4) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "sign_ed25519",
                                                    ),
                                                );
                                            }
                                            __field4 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field5 => {
                                            if _serde::__private225::Option::is_some(&__field5) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "key_exchange_x25519",
                                                    ),
                                                );
                                            }
                                            __field5 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field6 => {
                                            if _serde::__private225::Option::is_some(&__field6) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "flag_64",
                                                    ),
                                                );
                                            }
                                            __field6 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field7 => {
                                            if _serde::__private225::Option::is_some(&__field7) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "use_post_quantum",
                                                    ),
                                                );
                                            }
                                            __field7 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field8 => {
                                            if _serde::__private225::Option::is_some(&__field8) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "post_quantum_settings",
                                                    ),
                                                );
                                            }
                                            __field8 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<PostQuantumSettings>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private225::Some(__field0) => __field0,
                                    _serde::__private225::None => default_true(),
                                };
                                let __field1 = match __field1 {
                                    _serde::__private225::Some(__field1) => __field1,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private225::Some(__field2) => __field2,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private225::Some(__field3) => __field3,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::__private225::Some(__field4) => __field4,
                                    _serde::__private225::None => default_true(),
                                };
                                let __field5 = match __field5 {
                                    _serde::__private225::Some(__field5) => __field5,
                                    _serde::__private225::None => default_true(),
                                };
                                let __field6 = match __field6 {
                                    _serde::__private225::Some(__field6) => __field6,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field7 = match __field7 {
                                    _serde::__private225::Some(__field7) => __field7,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field8 = match __field8 {
                                    _serde::__private225::Some(__field8) => __field8,
                                    _serde::__private225::None => {
                                        _serde::__private225::de::missing_field(
                                            "post_quantum_settings",
                                        )?
                                    }
                                };
                                _serde::__private225::Ok(CryptoSettings {
                                    encrypt_with_cha_cha20: __field0,
                                    encrypt_with_aes: __field1,
                                    larger_hashes: __field2,
                                    use_blake3: __field3,
                                    sign_ed25519: __field4,
                                    key_exchange_x25519: __field5,
                                    flag_64: __field6,
                                    use_post_quantum: __field7,
                                    post_quantum_settings: __field8,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "encrypt_with_cha_cha20",
                            "encrypt_with_aes",
                            "larger_hashes",
                            "use_blake3",
                            "sign_ed25519",
                            "key_exchange_x25519",
                            "flag_64",
                            "use_post_quantum",
                            "post_quantum_settings",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "CryptoSettings",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<CryptoSettings>,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for CryptoSettings {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for CryptoSettings {
                #[inline]
                fn eq(&self, other: &CryptoSettings) -> bool {
                    self.encrypt_with_cha_cha20 == other.encrypt_with_cha_cha20
                        && self.encrypt_with_aes == other.encrypt_with_aes
                        && self.larger_hashes == other.larger_hashes
                        && self.use_blake3 == other.use_blake3
                        && self.sign_ed25519 == other.sign_ed25519
                        && self.key_exchange_x25519 == other.key_exchange_x25519
                        && self.flag_64 == other.flag_64
                        && self.use_post_quantum == other.use_post_quantum
                        && self.post_quantum_settings == other.post_quantum_settings
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for CryptoSettings {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "encrypt_with_cha_cha20",
                        "encrypt_with_aes",
                        "larger_hashes",
                        "use_blake3",
                        "sign_ed25519",
                        "key_exchange_x25519",
                        "flag_64",
                        "use_post_quantum",
                        "post_quantum_settings",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.encrypt_with_cha_cha20,
                        &self.encrypt_with_aes,
                        &self.larger_hashes,
                        &self.use_blake3,
                        &self.sign_ed25519,
                        &self.key_exchange_x25519,
                        &self.flag_64,
                        &self.use_post_quantum,
                        &&self.post_quantum_settings,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "CryptoSettings",
                        names,
                        values,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for CryptoSettings {
                #[inline]
                fn clone(&self) -> CryptoSettings {
                    CryptoSettings {
                        encrypt_with_cha_cha20: ::core::clone::Clone::clone(
                            &self.encrypt_with_cha_cha20,
                        ),
                        encrypt_with_aes: ::core::clone::Clone::clone(
                            &self.encrypt_with_aes,
                        ),
                        larger_hashes: ::core::clone::Clone::clone(&self.larger_hashes),
                        use_blake3: ::core::clone::Clone::clone(&self.use_blake3),
                        sign_ed25519: ::core::clone::Clone::clone(&self.sign_ed25519),
                        key_exchange_x25519: ::core::clone::Clone::clone(
                            &self.key_exchange_x25519,
                        ),
                        flag_64: ::core::clone::Clone::clone(&self.flag_64),
                        use_post_quantum: ::core::clone::Clone::clone(
                            &self.use_post_quantum,
                        ),
                        post_quantum_settings: ::core::clone::Clone::clone(
                            &self.post_quantum_settings,
                        ),
                    }
                }
            }
            pub struct PostQuantumSettings {
                /// Sign with ML-DSA-44, public key size 1312 B, signature 2420 B.
                /// Super fast, NIST level 1 security.
                #[serde(default)]
                pub sign_pqc_dsa_44: bool,
                /// Sign with ML-DSA-65, public key size 1952 B, signature 3309 B.
                /// Super fast, NIST level 3 security.
                #[serde(default)]
                pub sign_pqc_dsa_65: bool,
                /// Sign with Falcon-1024, public key size 1793 B, signature 1462 B.
                /// 3x slower than ML-DSA, NIST level 5 security.
                #[serde(default)]
                pub sign_pqc_falcon: bool,
                /// Sign with SLH-DSA-SHA128s, public key size 32 B, signature 7856 B.
                /// Very slow, but might be more secure because its based on hash functions only.
                /// NIST level 1 security.
                #[serde(default)]
                pub sign_pqc_slh_dsa: bool,
                /// Use ML-KEM-512 for key exchange, public key size 800 B, ciphertext size 768 B
                #[serde(default)]
                pub key_exchange_pqc_kem_512: bool,
                /// Use ML-KEM-768 for key exchange, public key size 1184 B, ciphertext size 1088 B
                #[serde(default)]
                pub key_exchange_pqc_kem_768: bool,
                /// Reserved for future use
                #[serde(default)]
                flag_64: bool,
                /// Reserved for future use
                #[serde(default)]
                flag_128: bool,
            }
            impl binary_codec::BinaryDeserializer for PostQuantumSettings {
                fn from_bytes(
                    bytes: &[u8],
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Self, binary_codec::DeserializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = bytes;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let sign_pqc_dsa_44 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let sign_pqc_dsa_65 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let sign_pqc_falcon = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let sign_pqc_slh_dsa = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let key_exchange_pqc_kem_512 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let key_exchange_pqc_kem_768 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let flag_64 = _p_val;
                    let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                    let flag_128 = _p_val;
                    Ok(Self {
                        sign_pqc_dsa_44,
                        sign_pqc_dsa_65,
                        sign_pqc_falcon,
                        sign_pqc_slh_dsa,
                        key_exchange_pqc_kem_512,
                        key_exchange_pqc_kem_768,
                        flag_64,
                        flag_128,
                    })
                }
            }
            impl binary_codec::BinarySerializer for PostQuantumSettings {
                fn to_bytes(
                    &self,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Vec<u8>, binary_codec::SerializationError> {
                    let mut bytes = Vec::new();
                    Self::write_bytes(self, &mut bytes, config)?;
                    Ok(bytes)
                }
                fn write_bytes(
                    &self,
                    buffer: &mut Vec<u8>,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<(), binary_codec::SerializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = buffer;
                    let _p_val = &self.sign_pqc_dsa_44;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.sign_pqc_dsa_65;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.sign_pqc_falcon;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.sign_pqc_slh_dsa;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.key_exchange_pqc_kem_512;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.key_exchange_pqc_kem_768;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.flag_64;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.flag_128;
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    Ok(())
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for PostQuantumSettings {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "PostQuantumSettings",
                            false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "sign_pqc_dsa_44",
                            &self.sign_pqc_dsa_44,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "sign_pqc_dsa_65",
                            &self.sign_pqc_dsa_65,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "sign_pqc_falcon",
                            &self.sign_pqc_falcon,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "sign_pqc_slh_dsa",
                            &self.sign_pqc_slh_dsa,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "key_exchange_pqc_kem_512",
                            &self.key_exchange_pqc_kem_512,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "key_exchange_pqc_kem_768",
                            &self.key_exchange_pqc_kem_768,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "flag_64",
                            &self.flag_64,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "flag_128",
                            &self.flag_128,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for PostQuantumSettings {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field0),
                                    1u64 => _serde::__private225::Ok(__Field::__field1),
                                    2u64 => _serde::__private225::Ok(__Field::__field2),
                                    3u64 => _serde::__private225::Ok(__Field::__field3),
                                    4u64 => _serde::__private225::Ok(__Field::__field4),
                                    5u64 => _serde::__private225::Ok(__Field::__field5),
                                    6u64 => _serde::__private225::Ok(__Field::__field6),
                                    7u64 => _serde::__private225::Ok(__Field::__field7),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "sign_pqc_dsa_44" => {
                                        _serde::__private225::Ok(__Field::__field0)
                                    }
                                    "sign_pqc_dsa_65" => {
                                        _serde::__private225::Ok(__Field::__field1)
                                    }
                                    "sign_pqc_falcon" => {
                                        _serde::__private225::Ok(__Field::__field2)
                                    }
                                    "sign_pqc_slh_dsa" => {
                                        _serde::__private225::Ok(__Field::__field3)
                                    }
                                    "key_exchange_pqc_kem_512" => {
                                        _serde::__private225::Ok(__Field::__field4)
                                    }
                                    "key_exchange_pqc_kem_768" => {
                                        _serde::__private225::Ok(__Field::__field5)
                                    }
                                    "flag_64" => _serde::__private225::Ok(__Field::__field6),
                                    "flag_128" => _serde::__private225::Ok(__Field::__field7),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"sign_pqc_dsa_44" => {
                                        _serde::__private225::Ok(__Field::__field0)
                                    }
                                    b"sign_pqc_dsa_65" => {
                                        _serde::__private225::Ok(__Field::__field1)
                                    }
                                    b"sign_pqc_falcon" => {
                                        _serde::__private225::Ok(__Field::__field2)
                                    }
                                    b"sign_pqc_slh_dsa" => {
                                        _serde::__private225::Ok(__Field::__field3)
                                    }
                                    b"key_exchange_pqc_kem_512" => {
                                        _serde::__private225::Ok(__Field::__field4)
                                    }
                                    b"key_exchange_pqc_kem_768" => {
                                        _serde::__private225::Ok(__Field::__field5)
                                    }
                                    b"flag_64" => _serde::__private225::Ok(__Field::__field6),
                                    b"flag_128" => _serde::__private225::Ok(__Field::__field7),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private225::PhantomData<
                                PostQuantumSettings,
                            >,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = PostQuantumSettings;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "struct PostQuantumSettings",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field3 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field4 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field5 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field6 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field7 = match _serde::de::SeqAccess::next_element::<
                                    bool,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                _serde::__private225::Ok(PostQuantumSettings {
                                    sign_pqc_dsa_44: __field0,
                                    sign_pqc_dsa_65: __field1,
                                    sign_pqc_falcon: __field2,
                                    sign_pqc_slh_dsa: __field3,
                                    key_exchange_pqc_kem_512: __field4,
                                    key_exchange_pqc_kem_768: __field5,
                                    flag_64: __field6,
                                    flag_128: __field7,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field2: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field3: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field4: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field5: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field6: _serde::__private225::Option<bool> = _serde::__private225::None;
                                let mut __field7: _serde::__private225::Option<bool> = _serde::__private225::None;
                                while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private225::Option::is_some(&__field0) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "sign_pqc_dsa_44",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private225::Option::is_some(&__field1) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "sign_pqc_dsa_65",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private225::Option::is_some(&__field2) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "sign_pqc_falcon",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private225::Option::is_some(&__field3) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "sign_pqc_slh_dsa",
                                                    ),
                                                );
                                            }
                                            __field3 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::__private225::Option::is_some(&__field4) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "key_exchange_pqc_kem_512",
                                                    ),
                                                );
                                            }
                                            __field4 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field5 => {
                                            if _serde::__private225::Option::is_some(&__field5) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "key_exchange_pqc_kem_768",
                                                    ),
                                                );
                                            }
                                            __field5 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field6 => {
                                            if _serde::__private225::Option::is_some(&__field6) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "flag_64",
                                                    ),
                                                );
                                            }
                                            __field6 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field7 => {
                                            if _serde::__private225::Option::is_some(&__field7) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "flag_128",
                                                    ),
                                                );
                                            }
                                            __field7 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private225::Some(__field0) => __field0,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private225::Some(__field1) => __field1,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private225::Some(__field2) => __field2,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private225::Some(__field3) => __field3,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::__private225::Some(__field4) => __field4,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field5 = match __field5 {
                                    _serde::__private225::Some(__field5) => __field5,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field6 = match __field6 {
                                    _serde::__private225::Some(__field6) => __field6,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field7 = match __field7 {
                                    _serde::__private225::Some(__field7) => __field7,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                _serde::__private225::Ok(PostQuantumSettings {
                                    sign_pqc_dsa_44: __field0,
                                    sign_pqc_dsa_65: __field1,
                                    sign_pqc_falcon: __field2,
                                    sign_pqc_slh_dsa: __field3,
                                    key_exchange_pqc_kem_512: __field4,
                                    key_exchange_pqc_kem_768: __field5,
                                    flag_64: __field6,
                                    flag_128: __field7,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "sign_pqc_dsa_44",
                            "sign_pqc_dsa_65",
                            "sign_pqc_falcon",
                            "sign_pqc_slh_dsa",
                            "key_exchange_pqc_kem_512",
                            "key_exchange_pqc_kem_768",
                            "flag_64",
                            "flag_128",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "PostQuantumSettings",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<
                                    PostQuantumSettings,
                                >,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for PostQuantumSettings {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for PostQuantumSettings {
                #[inline]
                fn eq(&self, other: &PostQuantumSettings) -> bool {
                    self.sign_pqc_dsa_44 == other.sign_pqc_dsa_44
                        && self.sign_pqc_dsa_65 == other.sign_pqc_dsa_65
                        && self.sign_pqc_falcon == other.sign_pqc_falcon
                        && self.sign_pqc_slh_dsa == other.sign_pqc_slh_dsa
                        && self.key_exchange_pqc_kem_512
                            == other.key_exchange_pqc_kem_512
                        && self.key_exchange_pqc_kem_768
                            == other.key_exchange_pqc_kem_768
                        && self.flag_64 == other.flag_64
                        && self.flag_128 == other.flag_128
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PostQuantumSettings {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "sign_pqc_dsa_44",
                        "sign_pqc_dsa_65",
                        "sign_pqc_falcon",
                        "sign_pqc_slh_dsa",
                        "key_exchange_pqc_kem_512",
                        "key_exchange_pqc_kem_768",
                        "flag_64",
                        "flag_128",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.sign_pqc_dsa_44,
                        &self.sign_pqc_dsa_65,
                        &self.sign_pqc_falcon,
                        &self.sign_pqc_slh_dsa,
                        &self.key_exchange_pqc_kem_512,
                        &self.key_exchange_pqc_kem_768,
                        &self.flag_64,
                        &&self.flag_128,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "PostQuantumSettings",
                        names,
                        values,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for PostQuantumSettings {
                #[inline]
                fn clone(&self) -> PostQuantumSettings {
                    PostQuantumSettings {
                        sign_pqc_dsa_44: ::core::clone::Clone::clone(
                            &self.sign_pqc_dsa_44,
                        ),
                        sign_pqc_dsa_65: ::core::clone::Clone::clone(
                            &self.sign_pqc_dsa_65,
                        ),
                        sign_pqc_falcon: ::core::clone::Clone::clone(
                            &self.sign_pqc_falcon,
                        ),
                        sign_pqc_slh_dsa: ::core::clone::Clone::clone(
                            &self.sign_pqc_slh_dsa,
                        ),
                        key_exchange_pqc_kem_512: ::core::clone::Clone::clone(
                            &self.key_exchange_pqc_kem_512,
                        ),
                        key_exchange_pqc_kem_768: ::core::clone::Clone::clone(
                            &self.key_exchange_pqc_kem_768,
                        ),
                        flag_64: ::core::clone::Clone::clone(&self.flag_64),
                        flag_128: ::core::clone::Clone::clone(&self.flag_128),
                    }
                }
            }
            impl Default for CryptoSettings {
                fn default() -> Self {
                    Self {
                        encrypt_with_cha_cha20: true,
                        encrypt_with_aes: false,
                        larger_hashes: false,
                        use_blake3: false,
                        sign_ed25519: true,
                        key_exchange_x25519: true,
                        flag_64: false,
                        use_post_quantum: false,
                        post_quantum_settings: None,
                    }
                }
            }
        }
        pub mod crypto_keys {
            use binary_codec::{
                DeserializationError, SerializationError, ToBytes, utils::slice,
            };
            use serde::{Deserialize, Serialize};
            use serde_with::serde_as;
            use serde_with::base64::{Base64, UrlSafe};
            use serde_with::formats::Unpadded;
            use crate::packets::base::settings::CryptoSettings;
            #[no_discriminator]
            pub enum CryptoKey {
                ChaCha20(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 32],
                ),
                Aes(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 32],
                ),
                Ed25519(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 32],
                ),
                X25519(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 32],
                ),
                Dsa44(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 1312],
                ),
                Dsa65(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 1952],
                ),
                Falcon(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 1793],
                ),
                SlhDsaSha128s(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 32],
                ),
                Kem512(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 800],
                ),
                Kem512Secret(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 768],
                ),
                Kem768(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 1184],
                ),
                Kem768Secret(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 1088],
                ),
            }
            impl binary_codec::BinarySerializer for CryptoKey {
                fn to_bytes(
                    &self,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Vec<u8>, binary_codec::SerializationError> {
                    let mut bytes = Vec::new();
                    Self::write_bytes(self, &mut bytes, config)?;
                    Ok(bytes)
                }
                fn write_bytes(
                    &self,
                    buffer: &mut Vec<u8>,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<(), binary_codec::SerializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = buffer;
                    match self {
                        Self::ChaCha20(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Aes(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Ed25519(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::X25519(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Dsa44(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Dsa65(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Falcon(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::SlhDsaSha128s(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Kem512(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Kem512Secret(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Kem768(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Kem768Secret(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            impl CryptoKey {
                fn get_discriminator(&self) -> u8 {
                    match self {
                        Self::ChaCha20(..) => 0u8,
                        Self::Aes(..) => 1u8,
                        Self::Ed25519(..) => 2u8,
                        Self::X25519(..) => 3u8,
                        Self::Dsa44(..) => 4u8,
                        Self::Dsa65(..) => 5u8,
                        Self::Falcon(..) => 6u8,
                        Self::SlhDsaSha128s(..) => 7u8,
                        Self::Kem512(..) => 8u8,
                        Self::Kem512Secret(..) => 9u8,
                        Self::Kem768(..) => 10u8,
                        Self::Kem768Secret(..) => 11u8,
                    }
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CryptoKey {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            CryptoKey::ChaCha20(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    0u32,
                                    "ChaCha20",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 32],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::Aes(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    1u32,
                                    "Aes",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 32],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::Ed25519(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    2u32,
                                    "Ed25519",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 32],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::X25519(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    3u32,
                                    "X25519",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 32],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::Dsa44(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    4u32,
                                    "Dsa44",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 1312],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::Dsa65(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    5u32,
                                    "Dsa65",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 1952],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::Falcon(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    6u32,
                                    "Falcon",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 1793],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::SlhDsaSha128s(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    7u32,
                                    "SlhDsaSha128s",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 32],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::Kem512(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    8u32,
                                    "Kem512",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 800],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::Kem512Secret(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    9u32,
                                    "Kem512Secret",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 768],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::Kem768(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    10u32,
                                    "Kem768",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 1184],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                            CryptoKey::Kem768Secret(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoKey",
                                    11u32,
                                    "Kem768Secret",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 1088],),
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<CryptoKey>,
                                        }
                                    },
                                )
                            }
                        }
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CryptoKey {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __field8,
                            __field9,
                            __field10,
                            __field11,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field0),
                                    1u64 => _serde::__private225::Ok(__Field::__field1),
                                    2u64 => _serde::__private225::Ok(__Field::__field2),
                                    3u64 => _serde::__private225::Ok(__Field::__field3),
                                    4u64 => _serde::__private225::Ok(__Field::__field4),
                                    5u64 => _serde::__private225::Ok(__Field::__field5),
                                    6u64 => _serde::__private225::Ok(__Field::__field6),
                                    7u64 => _serde::__private225::Ok(__Field::__field7),
                                    8u64 => _serde::__private225::Ok(__Field::__field8),
                                    9u64 => _serde::__private225::Ok(__Field::__field9),
                                    10u64 => _serde::__private225::Ok(__Field::__field10),
                                    11u64 => _serde::__private225::Ok(__Field::__field11),
                                    _ => {
                                        _serde::__private225::Err(
                                            _serde::de::Error::invalid_value(
                                                _serde::de::Unexpected::Unsigned(__value),
                                                &"variant index 0 <= i < 12",
                                            ),
                                        )
                                    }
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "ChaCha20" => _serde::__private225::Ok(__Field::__field0),
                                    "Aes" => _serde::__private225::Ok(__Field::__field1),
                                    "Ed25519" => _serde::__private225::Ok(__Field::__field2),
                                    "X25519" => _serde::__private225::Ok(__Field::__field3),
                                    "Dsa44" => _serde::__private225::Ok(__Field::__field4),
                                    "Dsa65" => _serde::__private225::Ok(__Field::__field5),
                                    "Falcon" => _serde::__private225::Ok(__Field::__field6),
                                    "SlhDsaSha128s" => {
                                        _serde::__private225::Ok(__Field::__field7)
                                    }
                                    "Kem512" => _serde::__private225::Ok(__Field::__field8),
                                    "Kem512Secret" => {
                                        _serde::__private225::Ok(__Field::__field9)
                                    }
                                    "Kem768" => _serde::__private225::Ok(__Field::__field10),
                                    "Kem768Secret" => {
                                        _serde::__private225::Ok(__Field::__field11)
                                    }
                                    _ => {
                                        _serde::__private225::Err(
                                            _serde::de::Error::unknown_variant(__value, VARIANTS),
                                        )
                                    }
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"ChaCha20" => _serde::__private225::Ok(__Field::__field0),
                                    b"Aes" => _serde::__private225::Ok(__Field::__field1),
                                    b"Ed25519" => _serde::__private225::Ok(__Field::__field2),
                                    b"X25519" => _serde::__private225::Ok(__Field::__field3),
                                    b"Dsa44" => _serde::__private225::Ok(__Field::__field4),
                                    b"Dsa65" => _serde::__private225::Ok(__Field::__field5),
                                    b"Falcon" => _serde::__private225::Ok(__Field::__field6),
                                    b"SlhDsaSha128s" => {
                                        _serde::__private225::Ok(__Field::__field7)
                                    }
                                    b"Kem512" => _serde::__private225::Ok(__Field::__field8),
                                    b"Kem512Secret" => {
                                        _serde::__private225::Ok(__Field::__field9)
                                    }
                                    b"Kem768" => _serde::__private225::Ok(__Field::__field10),
                                    b"Kem768Secret" => {
                                        _serde::__private225::Ok(__Field::__field11)
                                    }
                                    _ => {
                                        let __value = &_serde::__private225::from_utf8_lossy(
                                            __value,
                                        );
                                        _serde::__private225::Err(
                                            _serde::de::Error::unknown_variant(__value, VARIANTS),
                                        )
                                    }
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private225::PhantomData<CryptoKey>,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CryptoKey;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "enum CryptoKey",
                                )
                            }
                            fn visit_enum<__A>(
                                self,
                                __data: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::EnumAccess<'de>,
                            {
                                match _serde::de::EnumAccess::variant(__data)? {
                                    (__Field::__field0, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 32],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::ChaCha20(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field1, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 32],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::Aes(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field2, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 32],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::Ed25519(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field3, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 32],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::X25519(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field4, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 1312],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::Dsa44(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field5, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 1952],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::Dsa65(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field6, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 1793],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::Falcon(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field7, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 32],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::SlhDsaSha128s(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field8, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 800],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::Kem512(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field9, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 768],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::Kem512Secret(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field10, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 1184],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::Kem768(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field11, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 1088],
                                            phantom: _serde::__private225::PhantomData<CryptoKey>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoKey::Kem768Secret(__wrapper.value),
                                        )
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        const VARIANTS: &'static [&'static str] = &[
                            "ChaCha20",
                            "Aes",
                            "Ed25519",
                            "X25519",
                            "Dsa44",
                            "Dsa65",
                            "Falcon",
                            "SlhDsaSha128s",
                            "Kem512",
                            "Kem512Secret",
                            "Kem768",
                            "Kem768Secret",
                        ];
                        _serde::Deserializer::deserialize_enum(
                            __deserializer,
                            "CryptoKey",
                            VARIANTS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<CryptoKey>,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            #[no_discriminator]
            pub enum CryptoSignature {
                Ed25519(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 64],
                ),
                Dsa44(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 2420],
                ),
                Dsa65(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 3309],
                ),
                Falcon(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 1462],
                ),
                SlhDsaSha128s(
                    #[serde_as(r#as = "Base64<UrlSafe, Unpadded>")]
                    #[serde(
                        with = ":: serde_with :: As :: < Base64 < UrlSafe, Unpadded > >"
                    )]
                    [u8; 7856],
                ),
            }
            impl binary_codec::BinarySerializer for CryptoSignature {
                fn to_bytes(
                    &self,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Vec<u8>, binary_codec::SerializationError> {
                    let mut bytes = Vec::new();
                    Self::write_bytes(self, &mut bytes, config)?;
                    Ok(bytes)
                }
                fn write_bytes(
                    &self,
                    buffer: &mut Vec<u8>,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<(), binary_codec::SerializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = buffer;
                    match self {
                        Self::Ed25519(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Dsa44(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Dsa65(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::Falcon(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                        Self::SlhDsaSha128s(f0) => {
                            let _p_val = f0;
                            for _p_val in _p_val {
                                binary_codec::fixed_int::FixedInt::write(
                                    *_p_val,
                                    _p_bytes,
                                    _p_config,
                                )?;
                            }
                        }
                    }
                    Ok(())
                }
            }
            impl CryptoSignature {
                fn get_discriminator(&self) -> u8 {
                    match self {
                        Self::Ed25519(..) => 0u8,
                        Self::Dsa44(..) => 1u8,
                        Self::Dsa65(..) => 2u8,
                        Self::Falcon(..) => 3u8,
                        Self::SlhDsaSha128s(..) => 4u8,
                    }
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for CryptoSignature {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            CryptoSignature::Ed25519(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoSignature",
                                    0u32,
                                    "Ed25519",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 64],),
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<
                                                CryptoSignature,
                                            >,
                                        }
                                    },
                                )
                            }
                            CryptoSignature::Dsa44(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoSignature",
                                    1u32,
                                    "Dsa44",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 2420],),
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<
                                                CryptoSignature,
                                            >,
                                        }
                                    },
                                )
                            }
                            CryptoSignature::Dsa65(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoSignature",
                                    2u32,
                                    "Dsa65",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 3309],),
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<
                                                CryptoSignature,
                                            >,
                                        }
                                    },
                                )
                            }
                            CryptoSignature::Falcon(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoSignature",
                                    3u32,
                                    "Falcon",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 1462],),
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<
                                                CryptoSignature,
                                            >,
                                        }
                                    },
                                )
                            }
                            CryptoSignature::SlhDsaSha128s(ref __field0) => {
                                _serde::Serializer::serialize_newtype_variant(
                                    __serializer,
                                    "CryptoSignature",
                                    4u32,
                                    "SlhDsaSha128s",
                                    &{
                                        #[doc(hidden)]
                                        struct __SerializeWith<'__a> {
                                            values: (&'__a [u8; 7856],),
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                        }
                                        #[automatically_derived]
                                        impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                            fn serialize<__S>(
                                                &self,
                                                __s: __S,
                                            ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                            where
                                                __S: _serde::Serializer,
                                            {
                                                ::serde_with::As::<
                                                    Base64<UrlSafe, Unpadded>,
                                                >::serialize(self.values.0, __s)
                                            }
                                        }
                                        __SerializeWith {
                                            values: (__field0,),
                                            phantom: _serde::__private225::PhantomData::<
                                                CryptoSignature,
                                            >,
                                        }
                                    },
                                )
                            }
                        }
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for CryptoSignature {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field0),
                                    1u64 => _serde::__private225::Ok(__Field::__field1),
                                    2u64 => _serde::__private225::Ok(__Field::__field2),
                                    3u64 => _serde::__private225::Ok(__Field::__field3),
                                    4u64 => _serde::__private225::Ok(__Field::__field4),
                                    _ => {
                                        _serde::__private225::Err(
                                            _serde::de::Error::invalid_value(
                                                _serde::de::Unexpected::Unsigned(__value),
                                                &"variant index 0 <= i < 5",
                                            ),
                                        )
                                    }
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "Ed25519" => _serde::__private225::Ok(__Field::__field0),
                                    "Dsa44" => _serde::__private225::Ok(__Field::__field1),
                                    "Dsa65" => _serde::__private225::Ok(__Field::__field2),
                                    "Falcon" => _serde::__private225::Ok(__Field::__field3),
                                    "SlhDsaSha128s" => {
                                        _serde::__private225::Ok(__Field::__field4)
                                    }
                                    _ => {
                                        _serde::__private225::Err(
                                            _serde::de::Error::unknown_variant(__value, VARIANTS),
                                        )
                                    }
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"Ed25519" => _serde::__private225::Ok(__Field::__field0),
                                    b"Dsa44" => _serde::__private225::Ok(__Field::__field1),
                                    b"Dsa65" => _serde::__private225::Ok(__Field::__field2),
                                    b"Falcon" => _serde::__private225::Ok(__Field::__field3),
                                    b"SlhDsaSha128s" => {
                                        _serde::__private225::Ok(__Field::__field4)
                                    }
                                    _ => {
                                        let __value = &_serde::__private225::from_utf8_lossy(
                                            __value,
                                        );
                                        _serde::__private225::Err(
                                            _serde::de::Error::unknown_variant(__value, VARIANTS),
                                        )
                                    }
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private225::PhantomData<CryptoSignature>,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = CryptoSignature;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "enum CryptoSignature",
                                )
                            }
                            fn visit_enum<__A>(
                                self,
                                __data: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::EnumAccess<'de>,
                            {
                                match _serde::de::EnumAccess::variant(__data)? {
                                    (__Field::__field0, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 64],
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoSignature::Ed25519(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field1, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 2420],
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoSignature::Dsa44(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field2, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 3309],
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoSignature::Dsa65(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field3, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 1462],
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoSignature::Falcon(__wrapper.value),
                                        )
                                    }
                                    (__Field::__field4, __variant) => {
                                        #[doc(hidden)]
                                        struct __DeserializeWith<'de> {
                                            value: [u8; 7856],
                                            phantom: _serde::__private225::PhantomData<CryptoSignature>,
                                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                                        }
                                        #[automatically_derived]
                                        impl<'de> _serde::Deserialize<'de>
                                        for __DeserializeWith<'de> {
                                            fn deserialize<__D>(
                                                __deserializer: __D,
                                            ) -> _serde::__private225::Result<Self, __D::Error>
                                            where
                                                __D: _serde::Deserializer<'de>,
                                            {
                                                _serde::__private225::Ok(__DeserializeWith {
                                                    value: ::serde_with::As::<
                                                        Base64<UrlSafe, Unpadded>,
                                                    >::deserialize(__deserializer)?,
                                                    phantom: _serde::__private225::PhantomData,
                                                    lifetime: _serde::__private225::PhantomData,
                                                })
                                            }
                                        }
                                        _serde::__private225::Result::map(
                                            _serde::de::VariantAccess::newtype_variant::<
                                                __DeserializeWith<'de>,
                                            >(__variant),
                                            |__wrapper| CryptoSignature::SlhDsaSha128s(__wrapper.value),
                                        )
                                    }
                                }
                            }
                        }
                        #[doc(hidden)]
                        const VARIANTS: &'static [&'static str] = &[
                            "Ed25519",
                            "Dsa44",
                            "Dsa65",
                            "Falcon",
                            "SlhDsaSha128s",
                        ];
                        _serde::Deserializer::deserialize_enum(
                            __deserializer,
                            "CryptoSignature",
                            VARIANTS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<
                                    CryptoSignature,
                                >,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            #[repr(u8)]
            pub enum Algorithm {
                ChaCha20,
                Aes,
                Ed25519,
                X25519,
                Dsa44,
                Dsa65,
                Falcon,
                SlhDsaSha128s,
                Kem512Key,
                Kem512Secret,
                Kem768Key,
                Kem768Secret,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Algorithm {
                #[inline]
                fn clone(&self) -> Algorithm {
                    *self
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for Algorithm {}
            #[automatically_derived]
            impl ::core::fmt::Debug for Algorithm {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            Algorithm::ChaCha20 => "ChaCha20",
                            Algorithm::Aes => "Aes",
                            Algorithm::Ed25519 => "Ed25519",
                            Algorithm::X25519 => "X25519",
                            Algorithm::Dsa44 => "Dsa44",
                            Algorithm::Dsa65 => "Dsa65",
                            Algorithm::Falcon => "Falcon",
                            Algorithm::SlhDsaSha128s => "SlhDsaSha128s",
                            Algorithm::Kem512Key => "Kem512Key",
                            Algorithm::Kem512Secret => "Kem512Secret",
                            Algorithm::Kem768Key => "Kem768Key",
                            Algorithm::Kem768Secret => "Kem768Secret",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for Algorithm {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for Algorithm {
                #[inline]
                fn eq(&self, other: &Algorithm) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::cmp::Eq for Algorithm {
                #[inline]
                #[doc(hidden)]
                #[coverage(off)]
                fn assert_receiver_is_total_eq(&self) -> () {}
            }
            impl CryptoKey {
                pub fn read_keys(
                    bytes: &[u8],
                    key_types: Vec<Algorithm>,
                    config: &mut binary_codec::SerializerConfig,
                ) -> Result<Vec<CryptoKey>, DeserializationError> {
                    let mut keys = Vec::new();
                    for key_type in key_types {
                        keys.push(
                            match key_type {
                                Algorithm::X25519 => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::X25519)
                                }
                                Algorithm::ChaCha20 => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::ChaCha20)
                                }
                                Algorithm::Aes => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::Aes)
                                }
                                Algorithm::Ed25519 => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::Ed25519)
                                }
                                Algorithm::Dsa44 => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::Dsa44)
                                }
                                Algorithm::Dsa65 => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::Dsa65)
                                }
                                Algorithm::Falcon => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::Falcon)
                                }
                                Algorithm::SlhDsaSha128s => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::SlhDsaSha128s)
                                }
                                Algorithm::Kem512Key => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::Kem512)
                                }
                                Algorithm::Kem768Key => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::Kem768)
                                }
                                Algorithm::Kem512Secret => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::Kem512Secret)
                                }
                                Algorithm::Kem768Secret => {
                                    Self::read_fixed_n(config, bytes, CryptoKey::Kem768Secret)
                                }
                            }?,
                        );
                    }
                    Ok(keys)
                }
                pub fn read_signatures(
                    bytes: &[u8],
                    signature_types: Vec<Algorithm>,
                    config: &mut binary_codec::SerializerConfig,
                ) -> Result<Vec<CryptoSignature>, DeserializationError> {
                    let mut signatures = Vec::new();
                    for signature_type in signature_types {
                        signatures
                            .push(
                                match signature_type {
                                    Algorithm::Ed25519 => {
                                        Self::read_fixed_n(config, bytes, CryptoSignature::Ed25519)
                                    }
                                    Algorithm::Dsa44 => {
                                        Self::read_fixed_n(config, bytes, CryptoSignature::Dsa44)
                                    }
                                    Algorithm::Dsa65 => {
                                        Self::read_fixed_n(config, bytes, CryptoSignature::Dsa65)
                                    }
                                    Algorithm::Falcon => {
                                        Self::read_fixed_n(config, bytes, CryptoSignature::Falcon)
                                    }
                                    Algorithm::SlhDsaSha128s => {
                                        Self::read_fixed_n(
                                            config,
                                            bytes,
                                            CryptoSignature::SlhDsaSha128s,
                                        )
                                    }
                                    other => {
                                        Err(
                                            DeserializationError::InvalidData(
                                                ::alloc::__export::must_use({
                                                    ::alloc::fmt::format(
                                                        format_args!("{0:?} is not a signature algorithm", other),
                                                    )
                                                }),
                                            ),
                                        )
                                    }
                                }?,
                            );
                    }
                    Ok(signatures)
                }
                pub fn verify_keys(
                    expected: Vec<Algorithm>,
                    actual: &Vec<CryptoKey>,
                ) -> Result<(), SerializationError> {
                    for (i, key) in actual.iter().enumerate() {
                        let variant = match key {
                            CryptoKey::X25519(_) => Algorithm::X25519,
                            CryptoKey::Kem512(_) => Algorithm::Kem512Key,
                            CryptoKey::Kem768(_) => Algorithm::Kem768Key,
                            CryptoKey::ChaCha20(_) => Algorithm::ChaCha20,
                            CryptoKey::Aes(_) => Algorithm::Aes,
                            CryptoKey::Ed25519(_) => Algorithm::Ed25519,
                            CryptoKey::Dsa44(_) => Algorithm::Dsa44,
                            CryptoKey::Dsa65(_) => Algorithm::Dsa65,
                            CryptoKey::Falcon(_) => Algorithm::Falcon,
                            CryptoKey::SlhDsaSha128s(_) => Algorithm::SlhDsaSha128s,
                            CryptoKey::Kem512Secret(_) => Algorithm::Kem512Secret,
                            CryptoKey::Kem768Secret(_) => Algorithm::Kem768Secret,
                        };
                        if expected.get(i).cloned() != Some(variant) {
                            return Err(
                                SerializationError::InvalidData(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Unexpected algorithm at position {0}: expected type {1:?}, got {2:?}",
                                                i,
                                                expected.get(i),
                                                variant,
                                            ),
                                        )
                                    }),
                                ),
                            );
                        }
                    }
                    Ok(())
                }
                pub fn get_key_exchange_key_types(
                    settings: &CryptoSettings,
                    is_request: bool,
                ) -> Vec<Algorithm> {
                    let mut expected = Vec::new();
                    if settings.key_exchange_x25519 {
                        expected.push(Algorithm::X25519);
                    }
                    if settings.use_post_quantum
                        && let Some(settings) = &settings.post_quantum_settings
                    {
                        if settings.key_exchange_pqc_kem_512 {
                            expected
                                .push(
                                    if is_request {
                                        Algorithm::Kem512Key
                                    } else {
                                        Algorithm::Kem512Secret
                                    },
                                );
                        }
                        if settings.key_exchange_pqc_kem_768 {
                            expected
                                .push(
                                    if is_request {
                                        Algorithm::Kem768Key
                                    } else {
                                        Algorithm::Kem768Secret
                                    },
                                );
                        }
                    }
                    expected
                }
                fn read_fixed_n<const N: usize, F, C>(
                    config: &mut binary_codec::SerializerConfig,
                    bytes: &[u8],
                    constructor: F,
                ) -> Result<C, DeserializationError>
                where
                    F: FnOnce([u8; N]) -> C,
                {
                    let data: [u8; N] = slice(config, bytes, N, true)?
                        .try_into()
                        .unwrap();
                    Ok(constructor(data))
                }
            }
        }
        /// Plabble Protocol Packet
        pub struct PlabblePacketBase {
            /// Plabble Protocol version
            /// 0 = debug
            #[bits = 4]
            pub version: u8,
            /// If set to true, this packet is sent outside of a session
            /// and no follow-up responses are expected.
            #[serde(default)]
            pub fire_and_forget: bool,
            /// If set to true, this packet uses a pre-shared key for encryption.
            #[serde(default)]
            #[toggles("pre_shared_key")]
            pub pre_shared_key: bool,
            /// If set to true, this packet uses encryption. If false, use a MAC (Message Authentication Code).
            #[serde(default)]
            #[toggles("encryption")]
            pub use_encryption: bool,
            /// If set to true, use custom encryption settings.
            #[serde(default)]
            #[toggles("crypto_settings")]
            pub specify_crypto_settings: bool,
            /// Encryption settings
            #[toggled_by = "crypto_settings"]
            pub crypto_settings: Option<CryptoSettings>,
            /// Pre-shared key ID, if using a pre-shared key
            #[serde_as(r#as = "Option<Base64<UrlSafe, Unpadded>>")]
            #[toggled_by = "pre_shared_key"]
            #[serde(default)]
            #[serde(
                with = ":: serde_with :: As :: < Option < Base64 < UrlSafe, Unpadded > > >"
            )]
            pub psk_id: Option<[u8; 16]>,
            /// Pre-shared key salt, if using a pre-shared key
            #[serde_as(r#as = "Option<Base64<UrlSafe, Unpadded>>")]
            #[toggled_by = "pre_shared_key"]
            #[serde(default)]
            #[serde(
                with = ":: serde_with :: As :: < Option < Base64 < UrlSafe, Unpadded > > >"
            )]
            pub psk_salt: Option<[u8; 16]>,
            /// Message Authentication Code (MAC)
            #[serde_as(r#as = "Option<Base64<UrlSafe, Unpadded>>")]
            #[toggled_by = "!encryption"]
            #[serde(default)]
            #[serde(
                with = ":: serde_with :: As :: < Option < Base64 < UrlSafe, Unpadded > > >"
            )]
            pub mac: Option<[u8; 16]>,
        }
        impl binary_codec::BinaryDeserializer for PlabblePacketBase {
            fn from_bytes(
                bytes: &[u8],
                config: Option<&mut binary_codec::SerializerConfig>,
            ) -> Result<Self, binary_codec::DeserializationError> {
                let mut _new_config = binary_codec::SerializerConfig::new();
                let _p_config = config.unwrap_or(&mut _new_config);
                let _p_bytes = bytes;
                let _p_val = binary_codec::dynamics::read_small_dynamic_unsigned(
                    _p_bytes,
                    _p_config,
                    4u8,
                )?;
                let version = _p_val;
                let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                let fire_and_forget = _p_val;
                let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                let pre_shared_key = _p_val;
                _p_config.set_toggle("pre_shared_key", _p_val);
                let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                let use_encryption = _p_val;
                _p_config.set_toggle("encryption", _p_val);
                let _p_val = binary_codec::dynamics::read_bool(_p_bytes, _p_config)?;
                let specify_crypto_settings = _p_val;
                _p_config.set_toggle("crypto_settings", _p_val);
                let mut __option_0: Option<CryptoSettings> = None;
                if _p_config.get_toggle("crypto_settings").unwrap_or(false) {
                    let _p_val = binary_codec::variable::read_object(
                        _p_bytes,
                        None,
                        _p_config,
                    )?;
                    __option_0 = Some(_p_val);
                }
                let _p_val = __option_0;
                let crypto_settings = _p_val;
                let mut __option_0: Option<[u8; 16]> = None;
                if _p_config.get_toggle("pre_shared_key").unwrap_or(false) {
                    let mut __val_1 = Vec::<u8>::with_capacity(16usize);
                    for _ in 0..16usize {
                        let _p_val = binary_codec::fixed_int::FixedInt::read(
                            _p_bytes,
                            _p_config,
                        )?;
                        __val_1.push(_p_val);
                    }
                    let _p_val = TryInto::<[u8; 16usize]>::try_into(__val_1)
                        .expect("Failed to convert Vec to array");
                    __option_0 = Some(_p_val);
                }
                let _p_val = __option_0;
                let psk_id = _p_val;
                let mut __option_0: Option<[u8; 16]> = None;
                if _p_config.get_toggle("pre_shared_key").unwrap_or(false) {
                    let mut __val_1 = Vec::<u8>::with_capacity(16usize);
                    for _ in 0..16usize {
                        let _p_val = binary_codec::fixed_int::FixedInt::read(
                            _p_bytes,
                            _p_config,
                        )?;
                        __val_1.push(_p_val);
                    }
                    let _p_val = TryInto::<[u8; 16usize]>::try_into(__val_1)
                        .expect("Failed to convert Vec to array");
                    __option_0 = Some(_p_val);
                }
                let _p_val = __option_0;
                let psk_salt = _p_val;
                let mut __option_0: Option<[u8; 16]> = None;
                if _p_config.get_toggle("!encryption").unwrap_or(false) {
                    let mut __val_1 = Vec::<u8>::with_capacity(16usize);
                    for _ in 0..16usize {
                        let _p_val = binary_codec::fixed_int::FixedInt::read(
                            _p_bytes,
                            _p_config,
                        )?;
                        __val_1.push(_p_val);
                    }
                    let _p_val = TryInto::<[u8; 16usize]>::try_into(__val_1)
                        .expect("Failed to convert Vec to array");
                    __option_0 = Some(_p_val);
                }
                let _p_val = __option_0;
                let mac = _p_val;
                Ok(Self {
                    version,
                    fire_and_forget,
                    pre_shared_key,
                    use_encryption,
                    specify_crypto_settings,
                    crypto_settings,
                    psk_id,
                    psk_salt,
                    mac,
                })
            }
        }
        impl binary_codec::BinarySerializer for PlabblePacketBase {
            fn to_bytes(
                &self,
                config: Option<&mut binary_codec::SerializerConfig>,
            ) -> Result<Vec<u8>, binary_codec::SerializationError> {
                let mut bytes = Vec::new();
                Self::write_bytes(self, &mut bytes, config)?;
                Ok(bytes)
            }
            fn write_bytes(
                &self,
                buffer: &mut Vec<u8>,
                config: Option<&mut binary_codec::SerializerConfig>,
            ) -> Result<(), binary_codec::SerializationError> {
                let mut _new_config = binary_codec::SerializerConfig::new();
                let _p_config = config.unwrap_or(&mut _new_config);
                let _p_bytes = buffer;
                let _p_val = &self.version;
                binary_codec::dynamics::write_small_dynamic_unsigned(
                    *_p_val,
                    _p_bytes,
                    _p_config,
                    4u8,
                )?;
                let _p_val = &self.fire_and_forget;
                binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                let _p_val = &self.pre_shared_key;
                _p_config.set_toggle("pre_shared_key", *_p_val);
                binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                let _p_val = &self.use_encryption;
                _p_config.set_toggle("encryption", *_p_val);
                binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                let _p_val = &self.specify_crypto_settings;
                _p_config.set_toggle("crypto_settings", *_p_val);
                binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                let _p_val = &self.crypto_settings;
                if _p_config.get_toggle("crypto_settings").unwrap_or(false) {
                    let _p_val = _p_val
                        .as_ref()
                        .expect("Expected Some value, because toggled_by field is true");
                    binary_codec::variable::write_object(
                        _p_val,
                        None,
                        _p_bytes,
                        _p_config,
                    )?;
                }
                let _p_val = &self.psk_id;
                if _p_config.get_toggle("pre_shared_key").unwrap_or(false) {
                    let _p_val = _p_val
                        .as_ref()
                        .expect("Expected Some value, because toggled_by field is true");
                    for _p_val in _p_val {
                        binary_codec::fixed_int::FixedInt::write(
                            *_p_val,
                            _p_bytes,
                            _p_config,
                        )?;
                    }
                }
                let _p_val = &self.psk_salt;
                if _p_config.get_toggle("pre_shared_key").unwrap_or(false) {
                    let _p_val = _p_val
                        .as_ref()
                        .expect("Expected Some value, because toggled_by field is true");
                    for _p_val in _p_val {
                        binary_codec::fixed_int::FixedInt::write(
                            *_p_val,
                            _p_bytes,
                            _p_config,
                        )?;
                    }
                }
                let _p_val = &self.mac;
                if _p_config.get_toggle("!encryption").unwrap_or(false) {
                    let _p_val = _p_val
                        .as_ref()
                        .expect("Expected Some value, because toggled_by field is true");
                    for _p_val in _p_val {
                        binary_codec::fixed_int::FixedInt::write(
                            *_p_val,
                            _p_bytes,
                            _p_config,
                        )?;
                    }
                }
                Ok(())
            }
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for PlabblePacketBase {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "PlabblePacketBase",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "version",
                        &self.version,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "fire_and_forget",
                        &self.fire_and_forget,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "pre_shared_key",
                        &self.pre_shared_key,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "use_encryption",
                        &self.use_encryption,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "specify_crypto_settings",
                        &self.specify_crypto_settings,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "crypto_settings",
                        &self.crypto_settings,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "psk_id",
                        &{
                            #[doc(hidden)]
                            struct __SerializeWith<'__a> {
                                values: (&'__a Option<[u8; 16]>,),
                                phantom: _serde::__private225::PhantomData<
                                    PlabblePacketBase,
                                >,
                            }
                            #[automatically_derived]
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    ::serde_with::As::<
                                        Option<Base64<UrlSafe, Unpadded>>,
                                    >::serialize(self.values.0, __s)
                                }
                            }
                            __SerializeWith {
                                values: (&self.psk_id,),
                                phantom: _serde::__private225::PhantomData::<
                                    PlabblePacketBase,
                                >,
                            }
                        },
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "psk_salt",
                        &{
                            #[doc(hidden)]
                            struct __SerializeWith<'__a> {
                                values: (&'__a Option<[u8; 16]>,),
                                phantom: _serde::__private225::PhantomData<
                                    PlabblePacketBase,
                                >,
                            }
                            #[automatically_derived]
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    ::serde_with::As::<
                                        Option<Base64<UrlSafe, Unpadded>>,
                                    >::serialize(self.values.0, __s)
                                }
                            }
                            __SerializeWith {
                                values: (&self.psk_salt,),
                                phantom: _serde::__private225::PhantomData::<
                                    PlabblePacketBase,
                                >,
                            }
                        },
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "mac",
                        &{
                            #[doc(hidden)]
                            struct __SerializeWith<'__a> {
                                values: (&'__a Option<[u8; 16]>,),
                                phantom: _serde::__private225::PhantomData<
                                    PlabblePacketBase,
                                >,
                            }
                            #[automatically_derived]
                            impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                fn serialize<__S>(
                                    &self,
                                    __s: __S,
                                ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                where
                                    __S: _serde::Serializer,
                                {
                                    ::serde_with::As::<
                                        Option<Base64<UrlSafe, Unpadded>>,
                                    >::serialize(self.values.0, __s)
                                }
                            }
                            __SerializeWith {
                                values: (&self.mac,),
                                phantom: _serde::__private225::PhantomData::<
                                    PlabblePacketBase,
                                >,
                            }
                        },
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for PlabblePacketBase {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private225::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private225::Formatter,
                        ) -> _serde::__private225::fmt::Result {
                            _serde::__private225::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private225::Ok(__Field::__field0),
                                1u64 => _serde::__private225::Ok(__Field::__field1),
                                2u64 => _serde::__private225::Ok(__Field::__field2),
                                3u64 => _serde::__private225::Ok(__Field::__field3),
                                4u64 => _serde::__private225::Ok(__Field::__field4),
                                5u64 => _serde::__private225::Ok(__Field::__field5),
                                6u64 => _serde::__private225::Ok(__Field::__field6),
                                7u64 => _serde::__private225::Ok(__Field::__field7),
                                8u64 => _serde::__private225::Ok(__Field::__field8),
                                _ => _serde::__private225::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "version" => _serde::__private225::Ok(__Field::__field0),
                                "fire_and_forget" => {
                                    _serde::__private225::Ok(__Field::__field1)
                                }
                                "pre_shared_key" => {
                                    _serde::__private225::Ok(__Field::__field2)
                                }
                                "use_encryption" => {
                                    _serde::__private225::Ok(__Field::__field3)
                                }
                                "specify_crypto_settings" => {
                                    _serde::__private225::Ok(__Field::__field4)
                                }
                                "crypto_settings" => {
                                    _serde::__private225::Ok(__Field::__field5)
                                }
                                "psk_id" => _serde::__private225::Ok(__Field::__field6),
                                "psk_salt" => _serde::__private225::Ok(__Field::__field7),
                                "mac" => _serde::__private225::Ok(__Field::__field8),
                                _ => _serde::__private225::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"version" => _serde::__private225::Ok(__Field::__field0),
                                b"fire_and_forget" => {
                                    _serde::__private225::Ok(__Field::__field1)
                                }
                                b"pre_shared_key" => {
                                    _serde::__private225::Ok(__Field::__field2)
                                }
                                b"use_encryption" => {
                                    _serde::__private225::Ok(__Field::__field3)
                                }
                                b"specify_crypto_settings" => {
                                    _serde::__private225::Ok(__Field::__field4)
                                }
                                b"crypto_settings" => {
                                    _serde::__private225::Ok(__Field::__field5)
                                }
                                b"psk_id" => _serde::__private225::Ok(__Field::__field6),
                                b"psk_salt" => _serde::__private225::Ok(__Field::__field7),
                                b"mac" => _serde::__private225::Ok(__Field::__field8),
                                _ => _serde::__private225::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private225::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private225::PhantomData<PlabblePacketBase>,
                        lifetime: _serde::__private225::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PlabblePacketBase;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private225::Formatter,
                        ) -> _serde::__private225::fmt::Result {
                            _serde::__private225::Formatter::write_str(
                                __formatter,
                                "struct PlabblePacketBase",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private225::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                u8,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    return _serde::__private225::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct PlabblePacketBase with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                bool,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                Option<CryptoSettings>,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    return _serde::__private225::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct PlabblePacketBase with 9 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match {
                                #[doc(hidden)]
                                struct __DeserializeWith<'de> {
                                    value: Option<[u8; 16]>,
                                    phantom: _serde::__private225::PhantomData<
                                        PlabblePacketBase,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de>
                                for __DeserializeWith<'de> {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::__private225::Ok(__DeserializeWith {
                                            value: ::serde_with::As::<
                                                Option<Base64<UrlSafe, Unpadded>>,
                                            >::deserialize(__deserializer)?,
                                            phantom: _serde::__private225::PhantomData,
                                            lifetime: _serde::__private225::PhantomData,
                                        })
                                    }
                                }
                                _serde::__private225::Option::map(
                                    _serde::de::SeqAccess::next_element::<
                                        __DeserializeWith<'de>,
                                    >(&mut __seq)?,
                                    |__wrap| __wrap.value,
                                )
                            } {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field7 = match {
                                #[doc(hidden)]
                                struct __DeserializeWith<'de> {
                                    value: Option<[u8; 16]>,
                                    phantom: _serde::__private225::PhantomData<
                                        PlabblePacketBase,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de>
                                for __DeserializeWith<'de> {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::__private225::Ok(__DeserializeWith {
                                            value: ::serde_with::As::<
                                                Option<Base64<UrlSafe, Unpadded>>,
                                            >::deserialize(__deserializer)?,
                                            phantom: _serde::__private225::PhantomData,
                                            lifetime: _serde::__private225::PhantomData,
                                        })
                                    }
                                }
                                _serde::__private225::Option::map(
                                    _serde::de::SeqAccess::next_element::<
                                        __DeserializeWith<'de>,
                                    >(&mut __seq)?,
                                    |__wrap| __wrap.value,
                                )
                            } {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field8 = match {
                                #[doc(hidden)]
                                struct __DeserializeWith<'de> {
                                    value: Option<[u8; 16]>,
                                    phantom: _serde::__private225::PhantomData<
                                        PlabblePacketBase,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de>
                                for __DeserializeWith<'de> {
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::__private225::Ok(__DeserializeWith {
                                            value: ::serde_with::As::<
                                                Option<Base64<UrlSafe, Unpadded>>,
                                            >::deserialize(__deserializer)?,
                                            phantom: _serde::__private225::PhantomData,
                                            lifetime: _serde::__private225::PhantomData,
                                        })
                                    }
                                }
                                _serde::__private225::Option::map(
                                    _serde::de::SeqAccess::next_element::<
                                        __DeserializeWith<'de>,
                                    >(&mut __seq)?,
                                    |__wrap| __wrap.value,
                                )
                            } {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            _serde::__private225::Ok(PlabblePacketBase {
                                version: __field0,
                                fire_and_forget: __field1,
                                pre_shared_key: __field2,
                                use_encryption: __field3,
                                specify_crypto_settings: __field4,
                                crypto_settings: __field5,
                                psk_id: __field6,
                                psk_salt: __field7,
                                mac: __field8,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private225::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private225::Option<u8> = _serde::__private225::None;
                            let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                            let mut __field2: _serde::__private225::Option<bool> = _serde::__private225::None;
                            let mut __field3: _serde::__private225::Option<bool> = _serde::__private225::None;
                            let mut __field4: _serde::__private225::Option<bool> = _serde::__private225::None;
                            let mut __field5: _serde::__private225::Option<
                                Option<CryptoSettings>,
                            > = _serde::__private225::None;
                            let mut __field6: _serde::__private225::Option<
                                Option<[u8; 16]>,
                            > = _serde::__private225::None;
                            let mut __field7: _serde::__private225::Option<
                                Option<[u8; 16]>,
                            > = _serde::__private225::None;
                            let mut __field8: _serde::__private225::Option<
                                Option<[u8; 16]>,
                            > = _serde::__private225::None;
                            while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private225::Option::is_some(&__field0) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "version",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private225::Option::is_some(&__field1) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "fire_and_forget",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private225::Option::is_some(&__field2) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "pre_shared_key",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private225::Option::is_some(&__field3) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "use_encryption",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private225::Option::is_some(&__field4) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "specify_crypto_settings",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private225::Option::is_some(&__field5) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "crypto_settings",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<CryptoSettings>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private225::Option::is_some(&__field6) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("psk_id"),
                                            );
                                        }
                                        __field6 = _serde::__private225::Some({
                                            #[doc(hidden)]
                                            struct __DeserializeWith<'de> {
                                                value: Option<[u8; 16]>,
                                                phantom: _serde::__private225::PhantomData<
                                                    PlabblePacketBase,
                                                >,
                                                lifetime: _serde::__private225::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::Deserialize<'de>
                                            for __DeserializeWith<'de> {
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private225::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::__private225::Ok(__DeserializeWith {
                                                        value: ::serde_with::As::<
                                                            Option<Base64<UrlSafe, Unpadded>>,
                                                        >::deserialize(__deserializer)?,
                                                        phantom: _serde::__private225::PhantomData,
                                                        lifetime: _serde::__private225::PhantomData,
                                                    })
                                                }
                                            }
                                            match _serde::de::MapAccess::next_value::<
                                                __DeserializeWith<'de>,
                                            >(&mut __map) {
                                                _serde::__private225::Ok(__wrapper) => __wrapper.value,
                                                _serde::__private225::Err(__err) => {
                                                    return _serde::__private225::Err(__err);
                                                }
                                            }
                                        });
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private225::Option::is_some(&__field7) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "psk_salt",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private225::Some({
                                            #[doc(hidden)]
                                            struct __DeserializeWith<'de> {
                                                value: Option<[u8; 16]>,
                                                phantom: _serde::__private225::PhantomData<
                                                    PlabblePacketBase,
                                                >,
                                                lifetime: _serde::__private225::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::Deserialize<'de>
                                            for __DeserializeWith<'de> {
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private225::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::__private225::Ok(__DeserializeWith {
                                                        value: ::serde_with::As::<
                                                            Option<Base64<UrlSafe, Unpadded>>,
                                                        >::deserialize(__deserializer)?,
                                                        phantom: _serde::__private225::PhantomData,
                                                        lifetime: _serde::__private225::PhantomData,
                                                    })
                                                }
                                            }
                                            match _serde::de::MapAccess::next_value::<
                                                __DeserializeWith<'de>,
                                            >(&mut __map) {
                                                _serde::__private225::Ok(__wrapper) => __wrapper.value,
                                                _serde::__private225::Err(__err) => {
                                                    return _serde::__private225::Err(__err);
                                                }
                                            }
                                        });
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private225::Option::is_some(&__field8) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("mac"),
                                            );
                                        }
                                        __field8 = _serde::__private225::Some({
                                            #[doc(hidden)]
                                            struct __DeserializeWith<'de> {
                                                value: Option<[u8; 16]>,
                                                phantom: _serde::__private225::PhantomData<
                                                    PlabblePacketBase,
                                                >,
                                                lifetime: _serde::__private225::PhantomData<&'de ()>,
                                            }
                                            #[automatically_derived]
                                            impl<'de> _serde::Deserialize<'de>
                                            for __DeserializeWith<'de> {
                                                fn deserialize<__D>(
                                                    __deserializer: __D,
                                                ) -> _serde::__private225::Result<Self, __D::Error>
                                                where
                                                    __D: _serde::Deserializer<'de>,
                                                {
                                                    _serde::__private225::Ok(__DeserializeWith {
                                                        value: ::serde_with::As::<
                                                            Option<Base64<UrlSafe, Unpadded>>,
                                                        >::deserialize(__deserializer)?,
                                                        phantom: _serde::__private225::PhantomData,
                                                        lifetime: _serde::__private225::PhantomData,
                                                    })
                                                }
                                            }
                                            match _serde::de::MapAccess::next_value::<
                                                __DeserializeWith<'de>,
                                            >(&mut __map) {
                                                _serde::__private225::Ok(__wrapper) => __wrapper.value,
                                                _serde::__private225::Err(__err) => {
                                                    return _serde::__private225::Err(__err);
                                                }
                                            }
                                        });
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private225::Some(__field0) => __field0,
                                _serde::__private225::None => {
                                    _serde::__private225::de::missing_field("version")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private225::Some(__field1) => __field1,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private225::Some(__field2) => __field2,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private225::Some(__field3) => __field3,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private225::Some(__field4) => __field4,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private225::Some(__field5) => __field5,
                                _serde::__private225::None => {
                                    _serde::__private225::de::missing_field("crypto_settings")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private225::Some(__field6) => __field6,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private225::Some(__field7) => __field7,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private225::Some(__field8) => __field8,
                                _serde::__private225::None => {
                                    _serde::__private225::Default::default()
                                }
                            };
                            _serde::__private225::Ok(PlabblePacketBase {
                                version: __field0,
                                fire_and_forget: __field1,
                                pre_shared_key: __field2,
                                use_encryption: __field3,
                                specify_crypto_settings: __field4,
                                crypto_settings: __field5,
                                psk_id: __field6,
                                psk_salt: __field7,
                                mac: __field8,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "version",
                        "fire_and_forget",
                        "pre_shared_key",
                        "use_encryption",
                        "specify_crypto_settings",
                        "crypto_settings",
                        "psk_id",
                        "psk_salt",
                        "mac",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "PlabblePacketBase",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private225::PhantomData::<
                                PlabblePacketBase,
                            >,
                            lifetime: _serde::__private225::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PlabblePacketBase {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PlabblePacketBase {
            #[inline]
            fn eq(&self, other: &PlabblePacketBase) -> bool {
                self.version == other.version
                    && self.fire_and_forget == other.fire_and_forget
                    && self.pre_shared_key == other.pre_shared_key
                    && self.use_encryption == other.use_encryption
                    && self.specify_crypto_settings == other.specify_crypto_settings
                    && self.crypto_settings == other.crypto_settings
                    && self.psk_id == other.psk_id && self.psk_salt == other.psk_salt
                    && self.mac == other.mac
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlabblePacketBase {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "version",
                    "fire_and_forget",
                    "pre_shared_key",
                    "use_encryption",
                    "specify_crypto_settings",
                    "crypto_settings",
                    "psk_id",
                    "psk_salt",
                    "mac",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.version,
                    &self.fire_and_forget,
                    &self.pre_shared_key,
                    &self.use_encryption,
                    &self.specify_crypto_settings,
                    &self.crypto_settings,
                    &self.psk_id,
                    &self.psk_salt,
                    &&self.mac,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "PlabblePacketBase",
                    names,
                    values,
                )
            }
        }
    }
    pub mod header {
        pub mod request_header {
            use binary_codec::{FromBytes, ToBytes};
            use serde::{Deserialize, Serialize};
            use crate::packets::header::type_and_flags::RequestPacketType;
            pub struct PlabbleRequestHeader {
                #[serde(skip_serializing, skip_deserializing)]
                #[bits = 4]
                #[variant_for("packet_type")]
                _type: u8,
                /// Packet type (derived from `_type`)
                #[variant_by = "packet_type"]
                pub packet_type: RequestPacketType,
            }
            impl binary_codec::BinaryDeserializer for PlabbleRequestHeader {
                fn from_bytes(
                    bytes: &[u8],
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Self, binary_codec::DeserializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = bytes;
                    let _p_val = binary_codec::dynamics::read_small_dynamic_unsigned(
                        _p_bytes,
                        _p_config,
                        4u8,
                    )?;
                    let _type = _p_val;
                    _p_config.set_variant("packet_type", _p_val as u8);
                    _p_config.discriminator = _p_config.get_variant("packet_type");
                    let _p_val = binary_codec::variable::read_object(
                        _p_bytes,
                        None,
                        _p_config,
                    )?;
                    let packet_type = _p_val;
                    Ok(Self { _type, packet_type })
                }
            }
            impl binary_codec::BinarySerializer for PlabbleRequestHeader {
                fn to_bytes(
                    &self,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Vec<u8>, binary_codec::SerializationError> {
                    let mut bytes = Vec::new();
                    Self::write_bytes(self, &mut bytes, config)?;
                    Ok(bytes)
                }
                fn write_bytes(
                    &self,
                    buffer: &mut Vec<u8>,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<(), binary_codec::SerializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = buffer;
                    let _p_val = &self._type;
                    _p_config.set_variant("packet_type", *_p_val as u8);
                    binary_codec::dynamics::write_small_dynamic_unsigned(
                        *_p_val,
                        _p_bytes,
                        _p_config,
                        4u8,
                    )?;
                    let _p_val = &self.packet_type;
                    _p_config.discriminator = _p_config.get_variant("packet_type");
                    binary_codec::variable::write_object(
                        _p_val,
                        None,
                        _p_bytes,
                        _p_config,
                    )?;
                    Ok(())
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for PlabbleRequestHeader {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "PlabbleRequestHeader",
                            false as usize + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "packet_type",
                            &self.packet_type,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for PlabbleRequestHeader {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field1),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "packet_type" => _serde::__private225::Ok(__Field::__field1),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"packet_type" => {
                                        _serde::__private225::Ok(__Field::__field1)
                                    }
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private225::PhantomData<
                                PlabbleRequestHeader,
                            >,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = PlabbleRequestHeader;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "struct PlabbleRequestHeader",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = _serde::__private225::Default::default();
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    RequestPacketType,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct PlabbleRequestHeader with 1 element",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private225::Ok(PlabbleRequestHeader {
                                    _type: __field0,
                                    packet_type: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field1: _serde::__private225::Option<
                                    RequestPacketType,
                                > = _serde::__private225::None;
                                while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field1 => {
                                            if _serde::__private225::Option::is_some(&__field1) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "packet_type",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    RequestPacketType,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field1 = match __field1 {
                                    _serde::__private225::Some(__field1) => __field1,
                                    _serde::__private225::None => {
                                        _serde::__private225::de::missing_field("packet_type")?
                                    }
                                };
                                _serde::__private225::Ok(PlabbleRequestHeader {
                                    _type: _serde::__private225::Default::default(),
                                    packet_type: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["packet_type"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "PlabbleRequestHeader",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<
                                    PlabbleRequestHeader,
                                >,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for PlabbleRequestHeader {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for PlabbleRequestHeader {
                #[inline]
                fn eq(&self, other: &PlabbleRequestHeader) -> bool {
                    self._type == other._type && self.packet_type == other.packet_type
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PlabbleRequestHeader {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "PlabbleRequestHeader",
                        "_type",
                        &self._type,
                        "packet_type",
                        &&self.packet_type,
                    )
                }
            }
            impl PlabbleRequestHeader {
                pub fn new(packet_type: RequestPacketType) -> Self {
                    Self { _type: 0, packet_type }
                }
            }
        }
        pub mod response_header {
            use binary_codec::{FromBytes, ToBytes};
            use serde::{Deserialize, Serialize};
            use crate::packets::header::type_and_flags::ResponsePacketType;
            pub struct PlabbleResponseHeader {
                #[serde(skip_serializing, skip_deserializing)]
                #[bits = 4]
                #[variant_for("packet_type")]
                _type: u8,
                /// Packet type (derived from `_type`)
                #[variant_by = "packet_type"]
                pub packet_type: ResponsePacketType,
                /// Counter of request to reply to, if in session
                #[toggled_by = "TODO"]
                pub request_counter: Option<u16>,
            }
            impl binary_codec::BinaryDeserializer for PlabbleResponseHeader {
                fn from_bytes(
                    bytes: &[u8],
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Self, binary_codec::DeserializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = bytes;
                    let _p_val = binary_codec::dynamics::read_small_dynamic_unsigned(
                        _p_bytes,
                        _p_config,
                        4u8,
                    )?;
                    let _type = _p_val;
                    _p_config.set_variant("packet_type", _p_val as u8);
                    _p_config.discriminator = _p_config.get_variant("packet_type");
                    let _p_val = binary_codec::variable::read_object(
                        _p_bytes,
                        None,
                        _p_config,
                    )?;
                    let packet_type = _p_val;
                    let mut __option_0: Option<u16> = None;
                    if _p_config.get_toggle("TODO").unwrap_or(false) {
                        let _p_val = binary_codec::fixed_int::FixedInt::read(
                            _p_bytes,
                            _p_config,
                        )?;
                        __option_0 = Some(_p_val);
                    }
                    let _p_val = __option_0;
                    let request_counter = _p_val;
                    Ok(Self {
                        _type,
                        packet_type,
                        request_counter,
                    })
                }
            }
            impl binary_codec::BinarySerializer for PlabbleResponseHeader {
                fn to_bytes(
                    &self,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Vec<u8>, binary_codec::SerializationError> {
                    let mut bytes = Vec::new();
                    Self::write_bytes(self, &mut bytes, config)?;
                    Ok(bytes)
                }
                fn write_bytes(
                    &self,
                    buffer: &mut Vec<u8>,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<(), binary_codec::SerializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = buffer;
                    let _p_val = &self._type;
                    _p_config.set_variant("packet_type", *_p_val as u8);
                    binary_codec::dynamics::write_small_dynamic_unsigned(
                        *_p_val,
                        _p_bytes,
                        _p_config,
                        4u8,
                    )?;
                    let _p_val = &self.packet_type;
                    _p_config.discriminator = _p_config.get_variant("packet_type");
                    binary_codec::variable::write_object(
                        _p_val,
                        None,
                        _p_bytes,
                        _p_config,
                    )?;
                    let _p_val = &self.request_counter;
                    if _p_config.get_toggle("TODO").unwrap_or(false) {
                        let _p_val = _p_val
                            .as_ref()
                            .expect(
                                "Expected Some value, because toggled_by field is true",
                            );
                        binary_codec::fixed_int::FixedInt::write(
                            *_p_val,
                            _p_bytes,
                            _p_config,
                        )?;
                    }
                    Ok(())
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for PlabbleResponseHeader {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "PlabbleResponseHeader",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "packet_type",
                            &self.packet_type,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "request_counter",
                            &self.request_counter,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for PlabbleResponseHeader {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field1,
                            __field2,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field1),
                                    1u64 => _serde::__private225::Ok(__Field::__field2),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "packet_type" => _serde::__private225::Ok(__Field::__field1),
                                    "request_counter" => {
                                        _serde::__private225::Ok(__Field::__field2)
                                    }
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"packet_type" => {
                                        _serde::__private225::Ok(__Field::__field1)
                                    }
                                    b"request_counter" => {
                                        _serde::__private225::Ok(__Field::__field2)
                                    }
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private225::PhantomData<
                                PlabbleResponseHeader,
                            >,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = PlabbleResponseHeader;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "struct PlabbleResponseHeader",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = _serde::__private225::Default::default();
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    ResponsePacketType,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct PlabbleResponseHeader with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    Option<u16>,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct PlabbleResponseHeader with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private225::Ok(PlabbleResponseHeader {
                                    _type: __field0,
                                    packet_type: __field1,
                                    request_counter: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field1: _serde::__private225::Option<
                                    ResponsePacketType,
                                > = _serde::__private225::None;
                                let mut __field2: _serde::__private225::Option<
                                    Option<u16>,
                                > = _serde::__private225::None;
                                while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field1 => {
                                            if _serde::__private225::Option::is_some(&__field1) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "packet_type",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    ResponsePacketType,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private225::Option::is_some(&__field2) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "request_counter",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<u16>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field1 = match __field1 {
                                    _serde::__private225::Some(__field1) => __field1,
                                    _serde::__private225::None => {
                                        _serde::__private225::de::missing_field("packet_type")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private225::Some(__field2) => __field2,
                                    _serde::__private225::None => {
                                        _serde::__private225::de::missing_field("request_counter")?
                                    }
                                };
                                _serde::__private225::Ok(PlabbleResponseHeader {
                                    _type: _serde::__private225::Default::default(),
                                    packet_type: __field1,
                                    request_counter: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "packet_type",
                            "request_counter",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "PlabbleResponseHeader",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<
                                    PlabbleResponseHeader,
                                >,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for PlabbleResponseHeader {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for PlabbleResponseHeader {
                #[inline]
                fn eq(&self, other: &PlabbleResponseHeader) -> bool {
                    self._type == other._type && self.packet_type == other.packet_type
                        && self.request_counter == other.request_counter
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PlabbleResponseHeader {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "PlabbleResponseHeader",
                        "_type",
                        &self._type,
                        "packet_type",
                        &self.packet_type,
                        "request_counter",
                        &&self.request_counter,
                    )
                }
            }
        }
        pub mod type_and_flags {
            use binary_codec::{FromBytes, ToBytes};
            use serde::{Deserialize, Serialize};
            #[serde(tag = "packet_type")]
            #[repr(u8)]
            #[no_discriminator]
            pub enum RequestPacketType {
                Certificate { full_chain: bool, challenge: bool, query_mode: bool },
                Session { persist_key: bool, enable_encryption: bool },
                Get { binary_keys: bool, subscribe: bool, range_mode_until: bool },
                Stream {
                    binary_keys: bool,
                    subscribe: bool,
                    range_mode_until: bool,
                    stream_append: bool,
                },
                Post {
                    binary_keys: bool,
                    subscribe: bool,
                    range_mode_until: bool,
                    do_not_persist: bool,
                },
                Patch,
                Put {
                    binary_keys: bool,
                    subscribe: bool,
                    with_keys: bool,
                    append: bool,
                },
                Delete { binary_keys: bool, range_mode_until: bool },
                Subscribe { binary_keys: bool, range_mode_until: bool },
                Unsubscribe { binary_keys: bool, range_mode_until: bool },
                Register,
                Identify,
                Proxy {
                    init_session: bool,
                    keep_connection: bool,
                    select_random_hops: bool,
                },
                _Reserved13,
                _Reserved14,
                _Reserved15,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for RequestPacketType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        RequestPacketType::Certificate {
                            full_chain: __self_0,
                            challenge: __self_1,
                            query_mode: __self_2,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field3_finish(
                                f,
                                "Certificate",
                                "full_chain",
                                __self_0,
                                "challenge",
                                __self_1,
                                "query_mode",
                                &__self_2,
                            )
                        }
                        RequestPacketType::Session {
                            persist_key: __self_0,
                            enable_encryption: __self_1,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field2_finish(
                                f,
                                "Session",
                                "persist_key",
                                __self_0,
                                "enable_encryption",
                                &__self_1,
                            )
                        }
                        RequestPacketType::Get {
                            binary_keys: __self_0,
                            subscribe: __self_1,
                            range_mode_until: __self_2,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field3_finish(
                                f,
                                "Get",
                                "binary_keys",
                                __self_0,
                                "subscribe",
                                __self_1,
                                "range_mode_until",
                                &__self_2,
                            )
                        }
                        RequestPacketType::Stream {
                            binary_keys: __self_0,
                            subscribe: __self_1,
                            range_mode_until: __self_2,
                            stream_append: __self_3,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field4_finish(
                                f,
                                "Stream",
                                "binary_keys",
                                __self_0,
                                "subscribe",
                                __self_1,
                                "range_mode_until",
                                __self_2,
                                "stream_append",
                                &__self_3,
                            )
                        }
                        RequestPacketType::Post {
                            binary_keys: __self_0,
                            subscribe: __self_1,
                            range_mode_until: __self_2,
                            do_not_persist: __self_3,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field4_finish(
                                f,
                                "Post",
                                "binary_keys",
                                __self_0,
                                "subscribe",
                                __self_1,
                                "range_mode_until",
                                __self_2,
                                "do_not_persist",
                                &__self_3,
                            )
                        }
                        RequestPacketType::Patch => {
                            ::core::fmt::Formatter::write_str(f, "Patch")
                        }
                        RequestPacketType::Put {
                            binary_keys: __self_0,
                            subscribe: __self_1,
                            with_keys: __self_2,
                            append: __self_3,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field4_finish(
                                f,
                                "Put",
                                "binary_keys",
                                __self_0,
                                "subscribe",
                                __self_1,
                                "with_keys",
                                __self_2,
                                "append",
                                &__self_3,
                            )
                        }
                        RequestPacketType::Delete {
                            binary_keys: __self_0,
                            range_mode_until: __self_1,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field2_finish(
                                f,
                                "Delete",
                                "binary_keys",
                                __self_0,
                                "range_mode_until",
                                &__self_1,
                            )
                        }
                        RequestPacketType::Subscribe {
                            binary_keys: __self_0,
                            range_mode_until: __self_1,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field2_finish(
                                f,
                                "Subscribe",
                                "binary_keys",
                                __self_0,
                                "range_mode_until",
                                &__self_1,
                            )
                        }
                        RequestPacketType::Unsubscribe {
                            binary_keys: __self_0,
                            range_mode_until: __self_1,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field2_finish(
                                f,
                                "Unsubscribe",
                                "binary_keys",
                                __self_0,
                                "range_mode_until",
                                &__self_1,
                            )
                        }
                        RequestPacketType::Register => {
                            ::core::fmt::Formatter::write_str(f, "Register")
                        }
                        RequestPacketType::Identify => {
                            ::core::fmt::Formatter::write_str(f, "Identify")
                        }
                        RequestPacketType::Proxy {
                            init_session: __self_0,
                            keep_connection: __self_1,
                            select_random_hops: __self_2,
                        } => {
                            ::core::fmt::Formatter::debug_struct_field3_finish(
                                f,
                                "Proxy",
                                "init_session",
                                __self_0,
                                "keep_connection",
                                __self_1,
                                "select_random_hops",
                                &__self_2,
                            )
                        }
                        RequestPacketType::_Reserved13 => {
                            ::core::fmt::Formatter::write_str(f, "_Reserved13")
                        }
                        RequestPacketType::_Reserved14 => {
                            ::core::fmt::Formatter::write_str(f, "_Reserved14")
                        }
                        RequestPacketType::_Reserved15 => {
                            ::core::fmt::Formatter::write_str(f, "_Reserved15")
                        }
                    }
                }
            }
            impl binary_codec::BinaryDeserializer for RequestPacketType {
                fn from_bytes(
                    bytes: &[u8],
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Self, binary_codec::DeserializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = bytes;
                    let _p_disc = if let Some(disc) = _p_config.discriminator.take() {
                        disc
                    } else {
                        binary_codec::fixed_int::FixedInt::read(_p_bytes, _p_config)?
                    };
                    match _p_disc {
                        0u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let full_chain = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let challenge = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let query_mode = _p_val;
                            Ok(Self::Certificate {
                                full_chain,
                                challenge,
                                query_mode,
                            })
                        }
                        1u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let persist_key = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let enable_encryption = _p_val;
                            Ok(Self::Session {
                                persist_key,
                                enable_encryption,
                            })
                        }
                        2u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let binary_keys = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let subscribe = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let range_mode_until = _p_val;
                            Ok(Self::Get {
                                binary_keys,
                                subscribe,
                                range_mode_until,
                            })
                        }
                        3u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let binary_keys = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let subscribe = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let range_mode_until = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let stream_append = _p_val;
                            Ok(Self::Stream {
                                binary_keys,
                                subscribe,
                                range_mode_until,
                                stream_append,
                            })
                        }
                        4u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let binary_keys = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let subscribe = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let range_mode_until = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let do_not_persist = _p_val;
                            Ok(Self::Post {
                                binary_keys,
                                subscribe,
                                range_mode_until,
                                do_not_persist,
                            })
                        }
                        5u8 => Ok(Self::Patch),
                        6u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let binary_keys = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let subscribe = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let with_keys = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let append = _p_val;
                            Ok(Self::Put {
                                binary_keys,
                                subscribe,
                                with_keys,
                                append,
                            })
                        }
                        7u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let binary_keys = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let range_mode_until = _p_val;
                            Ok(Self::Delete {
                                binary_keys,
                                range_mode_until,
                            })
                        }
                        8u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let binary_keys = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let range_mode_until = _p_val;
                            Ok(Self::Subscribe {
                                binary_keys,
                                range_mode_until,
                            })
                        }
                        9u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let binary_keys = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let range_mode_until = _p_val;
                            Ok(Self::Unsubscribe {
                                binary_keys,
                                range_mode_until,
                            })
                        }
                        10u8 => Ok(Self::Register),
                        11u8 => Ok(Self::Identify),
                        12u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let init_session = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let keep_connection = _p_val;
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let select_random_hops = _p_val;
                            Ok(Self::Proxy {
                                init_session,
                                keep_connection,
                                select_random_hops,
                            })
                        }
                        13u8 => Ok(Self::_Reserved13),
                        14u8 => Ok(Self::_Reserved14),
                        15u8 => Ok(Self::_Reserved15),
                        _ => {
                            Err(
                                binary_codec::DeserializationError::UnknownDiscriminant(
                                    _p_disc,
                                ),
                            )
                        }
                    }
                }
            }
            impl binary_codec::BinarySerializer for RequestPacketType {
                fn to_bytes(
                    &self,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Vec<u8>, binary_codec::SerializationError> {
                    let mut bytes = Vec::new();
                    Self::write_bytes(self, &mut bytes, config)?;
                    Ok(bytes)
                }
                fn write_bytes(
                    &self,
                    buffer: &mut Vec<u8>,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<(), binary_codec::SerializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = buffer;
                    match self {
                        Self::Certificate { full_chain, challenge, query_mode } => {
                            let _p_val = full_chain;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = challenge;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = query_mode;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Session { persist_key, enable_encryption } => {
                            let _p_val = persist_key;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = enable_encryption;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Get { binary_keys, subscribe, range_mode_until } => {
                            let _p_val = binary_keys;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = subscribe;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = range_mode_until;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Stream {
                            binary_keys,
                            subscribe,
                            range_mode_until,
                            stream_append,
                        } => {
                            let _p_val = binary_keys;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = subscribe;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = range_mode_until;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = stream_append;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Post {
                            binary_keys,
                            subscribe,
                            range_mode_until,
                            do_not_persist,
                        } => {
                            let _p_val = binary_keys;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = subscribe;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = range_mode_until;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = do_not_persist;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Patch => {}
                        Self::Put { binary_keys, subscribe, with_keys, append } => {
                            let _p_val = binary_keys;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = subscribe;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = with_keys;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = append;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Delete { binary_keys, range_mode_until } => {
                            let _p_val = binary_keys;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = range_mode_until;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Subscribe { binary_keys, range_mode_until } => {
                            let _p_val = binary_keys;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = range_mode_until;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Unsubscribe { binary_keys, range_mode_until } => {
                            let _p_val = binary_keys;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = range_mode_until;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Register => {}
                        Self::Identify => {}
                        Self::Proxy {
                            init_session,
                            keep_connection,
                            select_random_hops,
                        } => {
                            let _p_val = init_session;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = keep_connection;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                            let _p_val = select_random_hops;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::_Reserved13 => {}
                        Self::_Reserved14 => {}
                        Self::_Reserved15 => {}
                    }
                    Ok(())
                }
            }
            impl RequestPacketType {
                fn get_discriminator(&self) -> u8 {
                    match self {
                        Self::Certificate { .. } => 0u8,
                        Self::Session { .. } => 1u8,
                        Self::Get { .. } => 2u8,
                        Self::Stream { .. } => 3u8,
                        Self::Post { .. } => 4u8,
                        Self::Patch => 5u8,
                        Self::Put { .. } => 6u8,
                        Self::Delete { .. } => 7u8,
                        Self::Subscribe { .. } => 8u8,
                        Self::Unsubscribe { .. } => 9u8,
                        Self::Register => 10u8,
                        Self::Identify => 11u8,
                        Self::Proxy { .. } => 12u8,
                        Self::_Reserved13 => 13u8,
                        Self::_Reserved14 => 14u8,
                        Self::_Reserved15 => 15u8,
                    }
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for RequestPacketType {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            RequestPacketType::Certificate {
                                ref full_chain,
                                ref challenge,
                                ref query_mode,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Certificate",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "full_chain",
                                    full_chain,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "challenge",
                                    challenge,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "query_mode",
                                    query_mode,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::Session {
                                ref persist_key,
                                ref enable_encryption,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Session",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "persist_key",
                                    persist_key,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "enable_encryption",
                                    enable_encryption,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::Get {
                                ref binary_keys,
                                ref subscribe,
                                ref range_mode_until,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Get",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "binary_keys",
                                    binary_keys,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "subscribe",
                                    subscribe,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "range_mode_until",
                                    range_mode_until,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::Stream {
                                ref binary_keys,
                                ref subscribe,
                                ref range_mode_until,
                                ref stream_append,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Stream",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "binary_keys",
                                    binary_keys,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "subscribe",
                                    subscribe,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "range_mode_until",
                                    range_mode_until,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "stream_append",
                                    stream_append,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::Post {
                                ref binary_keys,
                                ref subscribe,
                                ref range_mode_until,
                                ref do_not_persist,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Post",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "binary_keys",
                                    binary_keys,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "subscribe",
                                    subscribe,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "range_mode_until",
                                    range_mode_until,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "do_not_persist",
                                    do_not_persist,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::Patch => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Patch",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            RequestPacketType::Put {
                                ref binary_keys,
                                ref subscribe,
                                ref with_keys,
                                ref append,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Put",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "binary_keys",
                                    binary_keys,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "subscribe",
                                    subscribe,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "with_keys",
                                    with_keys,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "append",
                                    append,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::Delete {
                                ref binary_keys,
                                ref range_mode_until,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Delete",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "binary_keys",
                                    binary_keys,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "range_mode_until",
                                    range_mode_until,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::Subscribe {
                                ref binary_keys,
                                ref range_mode_until,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Subscribe",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "binary_keys",
                                    binary_keys,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "range_mode_until",
                                    range_mode_until,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::Unsubscribe {
                                ref binary_keys,
                                ref range_mode_until,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Unsubscribe",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "binary_keys",
                                    binary_keys,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "range_mode_until",
                                    range_mode_until,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::Register => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Register",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            RequestPacketType::Identify => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Identify",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            RequestPacketType::Proxy {
                                ref init_session,
                                ref keep_connection,
                                ref select_random_hops,
                            } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    0 + 1 + 1 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Proxy",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "init_session",
                                    init_session,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "keep_connection",
                                    keep_connection,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "select_random_hops",
                                    select_random_hops,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            RequestPacketType::_Reserved13 => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "_Reserved13",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            RequestPacketType::_Reserved14 => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "_Reserved14",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            RequestPacketType::_Reserved15 => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "RequestPacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "_Reserved15",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                        }
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for RequestPacketType {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __field8,
                            __field9,
                            __field10,
                            __field11,
                            __field12,
                            __field13,
                            __field14,
                            __field15,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field0),
                                    1u64 => _serde::__private225::Ok(__Field::__field1),
                                    2u64 => _serde::__private225::Ok(__Field::__field2),
                                    3u64 => _serde::__private225::Ok(__Field::__field3),
                                    4u64 => _serde::__private225::Ok(__Field::__field4),
                                    5u64 => _serde::__private225::Ok(__Field::__field5),
                                    6u64 => _serde::__private225::Ok(__Field::__field6),
                                    7u64 => _serde::__private225::Ok(__Field::__field7),
                                    8u64 => _serde::__private225::Ok(__Field::__field8),
                                    9u64 => _serde::__private225::Ok(__Field::__field9),
                                    10u64 => _serde::__private225::Ok(__Field::__field10),
                                    11u64 => _serde::__private225::Ok(__Field::__field11),
                                    12u64 => _serde::__private225::Ok(__Field::__field12),
                                    13u64 => _serde::__private225::Ok(__Field::__field13),
                                    14u64 => _serde::__private225::Ok(__Field::__field14),
                                    15u64 => _serde::__private225::Ok(__Field::__field15),
                                    _ => {
                                        _serde::__private225::Err(
                                            _serde::de::Error::invalid_value(
                                                _serde::de::Unexpected::Unsigned(__value),
                                                &"variant index 0 <= i < 16",
                                            ),
                                        )
                                    }
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "Certificate" => _serde::__private225::Ok(__Field::__field0),
                                    "Session" => _serde::__private225::Ok(__Field::__field1),
                                    "Get" => _serde::__private225::Ok(__Field::__field2),
                                    "Stream" => _serde::__private225::Ok(__Field::__field3),
                                    "Post" => _serde::__private225::Ok(__Field::__field4),
                                    "Patch" => _serde::__private225::Ok(__Field::__field5),
                                    "Put" => _serde::__private225::Ok(__Field::__field6),
                                    "Delete" => _serde::__private225::Ok(__Field::__field7),
                                    "Subscribe" => _serde::__private225::Ok(__Field::__field8),
                                    "Unsubscribe" => _serde::__private225::Ok(__Field::__field9),
                                    "Register" => _serde::__private225::Ok(__Field::__field10),
                                    "Identify" => _serde::__private225::Ok(__Field::__field11),
                                    "Proxy" => _serde::__private225::Ok(__Field::__field12),
                                    "_Reserved13" => {
                                        _serde::__private225::Ok(__Field::__field13)
                                    }
                                    "_Reserved14" => {
                                        _serde::__private225::Ok(__Field::__field14)
                                    }
                                    "_Reserved15" => {
                                        _serde::__private225::Ok(__Field::__field15)
                                    }
                                    _ => {
                                        _serde::__private225::Err(
                                            _serde::de::Error::unknown_variant(__value, VARIANTS),
                                        )
                                    }
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"Certificate" => {
                                        _serde::__private225::Ok(__Field::__field0)
                                    }
                                    b"Session" => _serde::__private225::Ok(__Field::__field1),
                                    b"Get" => _serde::__private225::Ok(__Field::__field2),
                                    b"Stream" => _serde::__private225::Ok(__Field::__field3),
                                    b"Post" => _serde::__private225::Ok(__Field::__field4),
                                    b"Patch" => _serde::__private225::Ok(__Field::__field5),
                                    b"Put" => _serde::__private225::Ok(__Field::__field6),
                                    b"Delete" => _serde::__private225::Ok(__Field::__field7),
                                    b"Subscribe" => _serde::__private225::Ok(__Field::__field8),
                                    b"Unsubscribe" => {
                                        _serde::__private225::Ok(__Field::__field9)
                                    }
                                    b"Register" => _serde::__private225::Ok(__Field::__field10),
                                    b"Identify" => _serde::__private225::Ok(__Field::__field11),
                                    b"Proxy" => _serde::__private225::Ok(__Field::__field12),
                                    b"_Reserved13" => {
                                        _serde::__private225::Ok(__Field::__field13)
                                    }
                                    b"_Reserved14" => {
                                        _serde::__private225::Ok(__Field::__field14)
                                    }
                                    b"_Reserved15" => {
                                        _serde::__private225::Ok(__Field::__field15)
                                    }
                                    _ => {
                                        let __value = &_serde::__private225::from_utf8_lossy(
                                            __value,
                                        );
                                        _serde::__private225::Err(
                                            _serde::de::Error::unknown_variant(__value, VARIANTS),
                                        )
                                    }
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        const VARIANTS: &'static [&'static str] = &[
                            "Certificate",
                            "Session",
                            "Get",
                            "Stream",
                            "Post",
                            "Patch",
                            "Put",
                            "Delete",
                            "Subscribe",
                            "Unsubscribe",
                            "Register",
                            "Identify",
                            "Proxy",
                            "_Reserved13",
                            "_Reserved14",
                            "_Reserved15",
                        ];
                        let (__tag, __content) = _serde::Deserializer::deserialize_any(
                            __deserializer,
                            _serde::__private225::de::TaggedContentVisitor::<
                                __Field,
                            >::new(
                                "packet_type",
                                "internally tagged enum RequestPacketType",
                            ),
                        )?;
                        let __deserializer = _serde::__private225::de::ContentDeserializer::<
                            __D::Error,
                        >::new(__content);
                        match __tag {
                            __Field::__field0 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            2u64 => _serde::__private225::Ok(__Field::__field2),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "full_chain" => _serde::__private225::Ok(__Field::__field0),
                                            "challenge" => _serde::__private225::Ok(__Field::__field1),
                                            "query_mode" => _serde::__private225::Ok(__Field::__field2),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"full_chain" => _serde::__private225::Ok(__Field::__field0),
                                            b"challenge" => _serde::__private225::Ok(__Field::__field1),
                                            b"query_mode" => _serde::__private225::Ok(__Field::__field2),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Certificate",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Certificate with 3 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Certificate with 3 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field2 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        2usize,
                                                        &"struct variant RequestPacketType::Certificate with 3 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Certificate {
                                            full_chain: __field0,
                                            challenge: __field1,
                                            query_mode: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field2: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "full_chain",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "challenge",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private225::Option::is_some(&__field2) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "query_mode",
                                                            ),
                                                        );
                                                    }
                                                    __field2 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("full_chain")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("challenge")?
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private225::Some(__field2) => __field2,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("query_mode")?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Certificate {
                                            full_chain: __field0,
                                            challenge: __field1,
                                            query_mode: __field2,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "full_chain",
                                    "challenge",
                                    "query_mode",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field1 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "persist_key" => _serde::__private225::Ok(__Field::__field0),
                                            "enable_encryption" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"persist_key" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            b"enable_encryption" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Session",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Session with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Session with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Session {
                                            persist_key: __field0,
                                            enable_encryption: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "persist_key",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "enable_encryption",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("persist_key")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field(
                                                    "enable_encryption",
                                                )?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Session {
                                            persist_key: __field0,
                                            enable_encryption: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "persist_key",
                                    "enable_encryption",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field2 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            2u64 => _serde::__private225::Ok(__Field::__field2),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "binary_keys" => _serde::__private225::Ok(__Field::__field0),
                                            "subscribe" => _serde::__private225::Ok(__Field::__field1),
                                            "range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"binary_keys" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            b"subscribe" => _serde::__private225::Ok(__Field::__field1),
                                            b"range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Get",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Get with 3 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Get with 3 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field2 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        2usize,
                                                        &"struct variant RequestPacketType::Get with 3 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Get {
                                            binary_keys: __field0,
                                            subscribe: __field1,
                                            range_mode_until: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field2: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "binary_keys",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "subscribe",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private225::Option::is_some(&__field2) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "range_mode_until",
                                                            ),
                                                        );
                                                    }
                                                    __field2 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("binary_keys")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("subscribe")?
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private225::Some(__field2) => __field2,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("range_mode_until")?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Get {
                                            binary_keys: __field0,
                                            subscribe: __field1,
                                            range_mode_until: __field2,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "binary_keys",
                                    "subscribe",
                                    "range_mode_until",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field3 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __field3,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            2u64 => _serde::__private225::Ok(__Field::__field2),
                                            3u64 => _serde::__private225::Ok(__Field::__field3),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "binary_keys" => _serde::__private225::Ok(__Field::__field0),
                                            "subscribe" => _serde::__private225::Ok(__Field::__field1),
                                            "range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field2)
                                            }
                                            "stream_append" => {
                                                _serde::__private225::Ok(__Field::__field3)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"binary_keys" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            b"subscribe" => _serde::__private225::Ok(__Field::__field1),
                                            b"range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field2)
                                            }
                                            b"stream_append" => {
                                                _serde::__private225::Ok(__Field::__field3)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Stream",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Stream with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Stream with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field2 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        2usize,
                                                        &"struct variant RequestPacketType::Stream with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field3 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        3usize,
                                                        &"struct variant RequestPacketType::Stream with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Stream {
                                            binary_keys: __field0,
                                            subscribe: __field1,
                                            range_mode_until: __field2,
                                            stream_append: __field3,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field2: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field3: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "binary_keys",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "subscribe",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private225::Option::is_some(&__field2) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "range_mode_until",
                                                            ),
                                                        );
                                                    }
                                                    __field2 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field3 => {
                                                    if _serde::__private225::Option::is_some(&__field3) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "stream_append",
                                                            ),
                                                        );
                                                    }
                                                    __field3 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("binary_keys")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("subscribe")?
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private225::Some(__field2) => __field2,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("range_mode_until")?
                                            }
                                        };
                                        let __field3 = match __field3 {
                                            _serde::__private225::Some(__field3) => __field3,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("stream_append")?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Stream {
                                            binary_keys: __field0,
                                            subscribe: __field1,
                                            range_mode_until: __field2,
                                            stream_append: __field3,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "binary_keys",
                                    "subscribe",
                                    "range_mode_until",
                                    "stream_append",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field4 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __field3,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            2u64 => _serde::__private225::Ok(__Field::__field2),
                                            3u64 => _serde::__private225::Ok(__Field::__field3),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "binary_keys" => _serde::__private225::Ok(__Field::__field0),
                                            "subscribe" => _serde::__private225::Ok(__Field::__field1),
                                            "range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field2)
                                            }
                                            "do_not_persist" => {
                                                _serde::__private225::Ok(__Field::__field3)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"binary_keys" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            b"subscribe" => _serde::__private225::Ok(__Field::__field1),
                                            b"range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field2)
                                            }
                                            b"do_not_persist" => {
                                                _serde::__private225::Ok(__Field::__field3)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Post",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Post with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Post with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field2 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        2usize,
                                                        &"struct variant RequestPacketType::Post with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field3 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        3usize,
                                                        &"struct variant RequestPacketType::Post with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Post {
                                            binary_keys: __field0,
                                            subscribe: __field1,
                                            range_mode_until: __field2,
                                            do_not_persist: __field3,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field2: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field3: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "binary_keys",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "subscribe",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private225::Option::is_some(&__field2) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "range_mode_until",
                                                            ),
                                                        );
                                                    }
                                                    __field2 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field3 => {
                                                    if _serde::__private225::Option::is_some(&__field3) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "do_not_persist",
                                                            ),
                                                        );
                                                    }
                                                    __field3 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("binary_keys")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("subscribe")?
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private225::Some(__field2) => __field2,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("range_mode_until")?
                                            }
                                        };
                                        let __field3 = match __field3 {
                                            _serde::__private225::Some(__field3) => __field3,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("do_not_persist")?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Post {
                                            binary_keys: __field0,
                                            subscribe: __field1,
                                            range_mode_until: __field2,
                                            do_not_persist: __field3,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "binary_keys",
                                    "subscribe",
                                    "range_mode_until",
                                    "do_not_persist",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field5 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "RequestPacketType",
                                        "Patch",
                                    ),
                                )?;
                                _serde::__private225::Ok(RequestPacketType::Patch)
                            }
                            __Field::__field6 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __field3,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            2u64 => _serde::__private225::Ok(__Field::__field2),
                                            3u64 => _serde::__private225::Ok(__Field::__field3),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "binary_keys" => _serde::__private225::Ok(__Field::__field0),
                                            "subscribe" => _serde::__private225::Ok(__Field::__field1),
                                            "with_keys" => _serde::__private225::Ok(__Field::__field2),
                                            "append" => _serde::__private225::Ok(__Field::__field3),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"binary_keys" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            b"subscribe" => _serde::__private225::Ok(__Field::__field1),
                                            b"with_keys" => _serde::__private225::Ok(__Field::__field2),
                                            b"append" => _serde::__private225::Ok(__Field::__field3),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Put",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Put with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Put with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field2 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        2usize,
                                                        &"struct variant RequestPacketType::Put with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field3 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        3usize,
                                                        &"struct variant RequestPacketType::Put with 4 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Put {
                                            binary_keys: __field0,
                                            subscribe: __field1,
                                            with_keys: __field2,
                                            append: __field3,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field2: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field3: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "binary_keys",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "subscribe",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private225::Option::is_some(&__field2) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "with_keys",
                                                            ),
                                                        );
                                                    }
                                                    __field2 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field3 => {
                                                    if _serde::__private225::Option::is_some(&__field3) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field("append"),
                                                        );
                                                    }
                                                    __field3 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("binary_keys")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("subscribe")?
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private225::Some(__field2) => __field2,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("with_keys")?
                                            }
                                        };
                                        let __field3 = match __field3 {
                                            _serde::__private225::Some(__field3) => __field3,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("append")?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Put {
                                            binary_keys: __field0,
                                            subscribe: __field1,
                                            with_keys: __field2,
                                            append: __field3,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "binary_keys",
                                    "subscribe",
                                    "with_keys",
                                    "append",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field7 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "binary_keys" => _serde::__private225::Ok(__Field::__field0),
                                            "range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"binary_keys" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            b"range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Delete",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Delete with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Delete with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Delete {
                                            binary_keys: __field0,
                                            range_mode_until: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "binary_keys",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "range_mode_until",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("binary_keys")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("range_mode_until")?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Delete {
                                            binary_keys: __field0,
                                            range_mode_until: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "binary_keys",
                                    "range_mode_until",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field8 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "binary_keys" => _serde::__private225::Ok(__Field::__field0),
                                            "range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"binary_keys" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            b"range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Subscribe",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Subscribe with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Subscribe with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Subscribe {
                                            binary_keys: __field0,
                                            range_mode_until: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "binary_keys",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "range_mode_until",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("binary_keys")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("range_mode_until")?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Subscribe {
                                            binary_keys: __field0,
                                            range_mode_until: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "binary_keys",
                                    "range_mode_until",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field9 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "binary_keys" => _serde::__private225::Ok(__Field::__field0),
                                            "range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"binary_keys" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            b"range_mode_until" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Unsubscribe",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Unsubscribe with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Unsubscribe with 2 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Unsubscribe {
                                            binary_keys: __field0,
                                            range_mode_until: __field1,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "binary_keys",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "range_mode_until",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("binary_keys")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("range_mode_until")?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Unsubscribe {
                                            binary_keys: __field0,
                                            range_mode_until: __field1,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "binary_keys",
                                    "range_mode_until",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field10 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "RequestPacketType",
                                        "Register",
                                    ),
                                )?;
                                _serde::__private225::Ok(RequestPacketType::Register)
                            }
                            __Field::__field11 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "RequestPacketType",
                                        "Identify",
                                    ),
                                )?;
                                _serde::__private225::Ok(RequestPacketType::Identify)
                            }
                            __Field::__field12 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __field1,
                                    __field2,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            1u64 => _serde::__private225::Ok(__Field::__field1),
                                            2u64 => _serde::__private225::Ok(__Field::__field2),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "init_session" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            "keep_connection" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            "select_random_hops" => {
                                                _serde::__private225::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"init_session" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            b"keep_connection" => {
                                                _serde::__private225::Ok(__Field::__field1)
                                            }
                                            b"select_random_hops" => {
                                                _serde::__private225::Ok(__Field::__field2)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        RequestPacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = RequestPacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant RequestPacketType::Proxy",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant RequestPacketType::Proxy with 3 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field1 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        1usize,
                                                        &"struct variant RequestPacketType::Proxy with 3 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        let __field2 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        2usize,
                                                        &"struct variant RequestPacketType::Proxy with 3 elements",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Proxy {
                                            init_session: __field0,
                                            keep_connection: __field1,
                                            select_random_hops: __field2,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field1: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        let mut __field2: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "init_session",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field1 => {
                                                    if _serde::__private225::Option::is_some(&__field1) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "keep_connection",
                                                            ),
                                                        );
                                                    }
                                                    __field1 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                __Field::__field2 => {
                                                    if _serde::__private225::Option::is_some(&__field2) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "select_random_hops",
                                                            ),
                                                        );
                                                    }
                                                    __field2 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("init_session")?
                                            }
                                        };
                                        let __field1 = match __field1 {
                                            _serde::__private225::Some(__field1) => __field1,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("keep_connection")?
                                            }
                                        };
                                        let __field2 = match __field2 {
                                            _serde::__private225::Some(__field2) => __field2,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field(
                                                    "select_random_hops",
                                                )?
                                            }
                                        };
                                        _serde::__private225::Ok(RequestPacketType::Proxy {
                                            init_session: __field0,
                                            keep_connection: __field1,
                                            select_random_hops: __field2,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "init_session",
                                    "keep_connection",
                                    "select_random_hops",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            RequestPacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field13 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "RequestPacketType",
                                        "_Reserved13",
                                    ),
                                )?;
                                _serde::__private225::Ok(RequestPacketType::_Reserved13)
                            }
                            __Field::__field14 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "RequestPacketType",
                                        "_Reserved14",
                                    ),
                                )?;
                                _serde::__private225::Ok(RequestPacketType::_Reserved14)
                            }
                            __Field::__field15 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "RequestPacketType",
                                        "_Reserved15",
                                    ),
                                )?;
                                _serde::__private225::Ok(RequestPacketType::_Reserved15)
                            }
                        }
                    }
                }
            };
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for RequestPacketType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for RequestPacketType {
                #[inline]
                fn eq(&self, other: &RequestPacketType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                RequestPacketType::Certificate {
                                    full_chain: __self_0,
                                    challenge: __self_1,
                                    query_mode: __self_2,
                                },
                                RequestPacketType::Certificate {
                                    full_chain: __arg1_0,
                                    challenge: __arg1_1,
                                    query_mode: __arg1_2,
                                },
                            ) => {
                                __self_0 == __arg1_0 && __self_1 == __arg1_1
                                    && __self_2 == __arg1_2
                            }
                            (
                                RequestPacketType::Session {
                                    persist_key: __self_0,
                                    enable_encryption: __self_1,
                                },
                                RequestPacketType::Session {
                                    persist_key: __arg1_0,
                                    enable_encryption: __arg1_1,
                                },
                            ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                            (
                                RequestPacketType::Get {
                                    binary_keys: __self_0,
                                    subscribe: __self_1,
                                    range_mode_until: __self_2,
                                },
                                RequestPacketType::Get {
                                    binary_keys: __arg1_0,
                                    subscribe: __arg1_1,
                                    range_mode_until: __arg1_2,
                                },
                            ) => {
                                __self_0 == __arg1_0 && __self_1 == __arg1_1
                                    && __self_2 == __arg1_2
                            }
                            (
                                RequestPacketType::Stream {
                                    binary_keys: __self_0,
                                    subscribe: __self_1,
                                    range_mode_until: __self_2,
                                    stream_append: __self_3,
                                },
                                RequestPacketType::Stream {
                                    binary_keys: __arg1_0,
                                    subscribe: __arg1_1,
                                    range_mode_until: __arg1_2,
                                    stream_append: __arg1_3,
                                },
                            ) => {
                                __self_0 == __arg1_0 && __self_1 == __arg1_1
                                    && __self_2 == __arg1_2 && __self_3 == __arg1_3
                            }
                            (
                                RequestPacketType::Post {
                                    binary_keys: __self_0,
                                    subscribe: __self_1,
                                    range_mode_until: __self_2,
                                    do_not_persist: __self_3,
                                },
                                RequestPacketType::Post {
                                    binary_keys: __arg1_0,
                                    subscribe: __arg1_1,
                                    range_mode_until: __arg1_2,
                                    do_not_persist: __arg1_3,
                                },
                            ) => {
                                __self_0 == __arg1_0 && __self_1 == __arg1_1
                                    && __self_2 == __arg1_2 && __self_3 == __arg1_3
                            }
                            (
                                RequestPacketType::Put {
                                    binary_keys: __self_0,
                                    subscribe: __self_1,
                                    with_keys: __self_2,
                                    append: __self_3,
                                },
                                RequestPacketType::Put {
                                    binary_keys: __arg1_0,
                                    subscribe: __arg1_1,
                                    with_keys: __arg1_2,
                                    append: __arg1_3,
                                },
                            ) => {
                                __self_0 == __arg1_0 && __self_1 == __arg1_1
                                    && __self_2 == __arg1_2 && __self_3 == __arg1_3
                            }
                            (
                                RequestPacketType::Delete {
                                    binary_keys: __self_0,
                                    range_mode_until: __self_1,
                                },
                                RequestPacketType::Delete {
                                    binary_keys: __arg1_0,
                                    range_mode_until: __arg1_1,
                                },
                            ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                            (
                                RequestPacketType::Subscribe {
                                    binary_keys: __self_0,
                                    range_mode_until: __self_1,
                                },
                                RequestPacketType::Subscribe {
                                    binary_keys: __arg1_0,
                                    range_mode_until: __arg1_1,
                                },
                            ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                            (
                                RequestPacketType::Unsubscribe {
                                    binary_keys: __self_0,
                                    range_mode_until: __self_1,
                                },
                                RequestPacketType::Unsubscribe {
                                    binary_keys: __arg1_0,
                                    range_mode_until: __arg1_1,
                                },
                            ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                            (
                                RequestPacketType::Proxy {
                                    init_session: __self_0,
                                    keep_connection: __self_1,
                                    select_random_hops: __self_2,
                                },
                                RequestPacketType::Proxy {
                                    init_session: __arg1_0,
                                    keep_connection: __arg1_1,
                                    select_random_hops: __arg1_2,
                                },
                            ) => {
                                __self_0 == __arg1_0 && __self_1 == __arg1_1
                                    && __self_2 == __arg1_2
                            }
                            _ => true,
                        }
                }
            }
            #[serde(tag = "packet_type")]
            #[repr(u8)]
            #[no_discriminator]
            pub enum ResponsePacketType {
                Certificate,
                Session { with_psk: bool },
                Get { binary_keys: bool },
                Stream,
                Post,
                Patch,
                Put,
                Delete,
                Subscribe,
                Unsubscribe,
                Register,
                Identify,
                Proxy { include_hop_info: bool },
                _Reserved13,
                _Reserved14,
                Error,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for ResponsePacketType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ResponsePacketType::Certificate => {
                            ::core::fmt::Formatter::write_str(f, "Certificate")
                        }
                        ResponsePacketType::Session { with_psk: __self_0 } => {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Session",
                                "with_psk",
                                &__self_0,
                            )
                        }
                        ResponsePacketType::Get { binary_keys: __self_0 } => {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Get",
                                "binary_keys",
                                &__self_0,
                            )
                        }
                        ResponsePacketType::Stream => {
                            ::core::fmt::Formatter::write_str(f, "Stream")
                        }
                        ResponsePacketType::Post => {
                            ::core::fmt::Formatter::write_str(f, "Post")
                        }
                        ResponsePacketType::Patch => {
                            ::core::fmt::Formatter::write_str(f, "Patch")
                        }
                        ResponsePacketType::Put => {
                            ::core::fmt::Formatter::write_str(f, "Put")
                        }
                        ResponsePacketType::Delete => {
                            ::core::fmt::Formatter::write_str(f, "Delete")
                        }
                        ResponsePacketType::Subscribe => {
                            ::core::fmt::Formatter::write_str(f, "Subscribe")
                        }
                        ResponsePacketType::Unsubscribe => {
                            ::core::fmt::Formatter::write_str(f, "Unsubscribe")
                        }
                        ResponsePacketType::Register => {
                            ::core::fmt::Formatter::write_str(f, "Register")
                        }
                        ResponsePacketType::Identify => {
                            ::core::fmt::Formatter::write_str(f, "Identify")
                        }
                        ResponsePacketType::Proxy { include_hop_info: __self_0 } => {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Proxy",
                                "include_hop_info",
                                &__self_0,
                            )
                        }
                        ResponsePacketType::_Reserved13 => {
                            ::core::fmt::Formatter::write_str(f, "_Reserved13")
                        }
                        ResponsePacketType::_Reserved14 => {
                            ::core::fmt::Formatter::write_str(f, "_Reserved14")
                        }
                        ResponsePacketType::Error => {
                            ::core::fmt::Formatter::write_str(f, "Error")
                        }
                    }
                }
            }
            impl binary_codec::BinaryDeserializer for ResponsePacketType {
                fn from_bytes(
                    bytes: &[u8],
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Self, binary_codec::DeserializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = bytes;
                    let _p_disc = if let Some(disc) = _p_config.discriminator.take() {
                        disc
                    } else {
                        binary_codec::fixed_int::FixedInt::read(_p_bytes, _p_config)?
                    };
                    match _p_disc {
                        0u8 => Ok(Self::Certificate),
                        1u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let with_psk = _p_val;
                            Ok(Self::Session { with_psk })
                        }
                        2u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let binary_keys = _p_val;
                            Ok(Self::Get { binary_keys })
                        }
                        3u8 => Ok(Self::Stream),
                        4u8 => Ok(Self::Post),
                        5u8 => Ok(Self::Patch),
                        6u8 => Ok(Self::Put),
                        7u8 => Ok(Self::Delete),
                        8u8 => Ok(Self::Subscribe),
                        9u8 => Ok(Self::Unsubscribe),
                        10u8 => Ok(Self::Register),
                        11u8 => Ok(Self::Identify),
                        12u8 => {
                            let _p_val = binary_codec::dynamics::read_bool(
                                _p_bytes,
                                _p_config,
                            )?;
                            let include_hop_info = _p_val;
                            Ok(Self::Proxy { include_hop_info })
                        }
                        13u8 => Ok(Self::_Reserved13),
                        14u8 => Ok(Self::_Reserved14),
                        15u8 => Ok(Self::Error),
                        _ => {
                            Err(
                                binary_codec::DeserializationError::UnknownDiscriminant(
                                    _p_disc,
                                ),
                            )
                        }
                    }
                }
            }
            impl binary_codec::BinarySerializer for ResponsePacketType {
                fn to_bytes(
                    &self,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Vec<u8>, binary_codec::SerializationError> {
                    let mut bytes = Vec::new();
                    Self::write_bytes(self, &mut bytes, config)?;
                    Ok(bytes)
                }
                fn write_bytes(
                    &self,
                    buffer: &mut Vec<u8>,
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<(), binary_codec::SerializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = buffer;
                    match self {
                        Self::Certificate => {}
                        Self::Session { with_psk } => {
                            let _p_val = with_psk;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Get { binary_keys } => {
                            let _p_val = binary_keys;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::Stream => {}
                        Self::Post => {}
                        Self::Patch => {}
                        Self::Put => {}
                        Self::Delete => {}
                        Self::Subscribe => {}
                        Self::Unsubscribe => {}
                        Self::Register => {}
                        Self::Identify => {}
                        Self::Proxy { include_hop_info } => {
                            let _p_val = include_hop_info;
                            binary_codec::dynamics::write_bool(
                                *_p_val,
                                _p_bytes,
                                _p_config,
                            )?;
                        }
                        Self::_Reserved13 => {}
                        Self::_Reserved14 => {}
                        Self::Error => {}
                    }
                    Ok(())
                }
            }
            impl ResponsePacketType {
                fn get_discriminator(&self) -> u8 {
                    match self {
                        Self::Certificate => 0u8,
                        Self::Session { .. } => 1u8,
                        Self::Get { .. } => 2u8,
                        Self::Stream => 3u8,
                        Self::Post => 4u8,
                        Self::Patch => 5u8,
                        Self::Put => 6u8,
                        Self::Delete => 7u8,
                        Self::Subscribe => 8u8,
                        Self::Unsubscribe => 9u8,
                        Self::Register => 10u8,
                        Self::Identify => 11u8,
                        Self::Proxy { .. } => 12u8,
                        Self::_Reserved13 => 13u8,
                        Self::_Reserved14 => 14u8,
                        Self::Error => 15u8,
                    }
                }
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for ResponsePacketType {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            ResponsePacketType::Certificate => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Certificate",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Session { ref with_psk } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    0 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Session",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "with_psk",
                                    with_psk,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            ResponsePacketType::Get { ref binary_keys } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    0 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Get",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "binary_keys",
                                    binary_keys,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            ResponsePacketType::Stream => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Stream",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Post => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Post",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Patch => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Patch",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Put => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Put",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Delete => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Delete",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Subscribe => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Subscribe",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Unsubscribe => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Unsubscribe",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Register => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Register",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Identify => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Identify",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Proxy { ref include_hop_info } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    0 + 1 + 1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "packet_type",
                                    "Proxy",
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __serde_state,
                                    "include_hop_info",
                                    include_hop_info,
                                )?;
                                _serde::ser::SerializeStruct::end(__serde_state)
                            }
                            ResponsePacketType::_Reserved13 => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "_Reserved13",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::_Reserved14 => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "_Reserved14",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketType::Error => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketType",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Error",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                        }
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for ResponsePacketType {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __field8,
                            __field9,
                            __field10,
                            __field11,
                            __field12,
                            __field13,
                            __field14,
                            __field15,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "variant identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field0),
                                    1u64 => _serde::__private225::Ok(__Field::__field1),
                                    2u64 => _serde::__private225::Ok(__Field::__field2),
                                    3u64 => _serde::__private225::Ok(__Field::__field3),
                                    4u64 => _serde::__private225::Ok(__Field::__field4),
                                    5u64 => _serde::__private225::Ok(__Field::__field5),
                                    6u64 => _serde::__private225::Ok(__Field::__field6),
                                    7u64 => _serde::__private225::Ok(__Field::__field7),
                                    8u64 => _serde::__private225::Ok(__Field::__field8),
                                    9u64 => _serde::__private225::Ok(__Field::__field9),
                                    10u64 => _serde::__private225::Ok(__Field::__field10),
                                    11u64 => _serde::__private225::Ok(__Field::__field11),
                                    12u64 => _serde::__private225::Ok(__Field::__field12),
                                    13u64 => _serde::__private225::Ok(__Field::__field13),
                                    14u64 => _serde::__private225::Ok(__Field::__field14),
                                    15u64 => _serde::__private225::Ok(__Field::__field15),
                                    _ => {
                                        _serde::__private225::Err(
                                            _serde::de::Error::invalid_value(
                                                _serde::de::Unexpected::Unsigned(__value),
                                                &"variant index 0 <= i < 16",
                                            ),
                                        )
                                    }
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "Certificate" => _serde::__private225::Ok(__Field::__field0),
                                    "Session" => _serde::__private225::Ok(__Field::__field1),
                                    "Get" => _serde::__private225::Ok(__Field::__field2),
                                    "Stream" => _serde::__private225::Ok(__Field::__field3),
                                    "Post" => _serde::__private225::Ok(__Field::__field4),
                                    "Patch" => _serde::__private225::Ok(__Field::__field5),
                                    "Put" => _serde::__private225::Ok(__Field::__field6),
                                    "Delete" => _serde::__private225::Ok(__Field::__field7),
                                    "Subscribe" => _serde::__private225::Ok(__Field::__field8),
                                    "Unsubscribe" => _serde::__private225::Ok(__Field::__field9),
                                    "Register" => _serde::__private225::Ok(__Field::__field10),
                                    "Identify" => _serde::__private225::Ok(__Field::__field11),
                                    "Proxy" => _serde::__private225::Ok(__Field::__field12),
                                    "_Reserved13" => {
                                        _serde::__private225::Ok(__Field::__field13)
                                    }
                                    "_Reserved14" => {
                                        _serde::__private225::Ok(__Field::__field14)
                                    }
                                    "Error" => _serde::__private225::Ok(__Field::__field15),
                                    _ => {
                                        _serde::__private225::Err(
                                            _serde::de::Error::unknown_variant(__value, VARIANTS),
                                        )
                                    }
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"Certificate" => {
                                        _serde::__private225::Ok(__Field::__field0)
                                    }
                                    b"Session" => _serde::__private225::Ok(__Field::__field1),
                                    b"Get" => _serde::__private225::Ok(__Field::__field2),
                                    b"Stream" => _serde::__private225::Ok(__Field::__field3),
                                    b"Post" => _serde::__private225::Ok(__Field::__field4),
                                    b"Patch" => _serde::__private225::Ok(__Field::__field5),
                                    b"Put" => _serde::__private225::Ok(__Field::__field6),
                                    b"Delete" => _serde::__private225::Ok(__Field::__field7),
                                    b"Subscribe" => _serde::__private225::Ok(__Field::__field8),
                                    b"Unsubscribe" => {
                                        _serde::__private225::Ok(__Field::__field9)
                                    }
                                    b"Register" => _serde::__private225::Ok(__Field::__field10),
                                    b"Identify" => _serde::__private225::Ok(__Field::__field11),
                                    b"Proxy" => _serde::__private225::Ok(__Field::__field12),
                                    b"_Reserved13" => {
                                        _serde::__private225::Ok(__Field::__field13)
                                    }
                                    b"_Reserved14" => {
                                        _serde::__private225::Ok(__Field::__field14)
                                    }
                                    b"Error" => _serde::__private225::Ok(__Field::__field15),
                                    _ => {
                                        let __value = &_serde::__private225::from_utf8_lossy(
                                            __value,
                                        );
                                        _serde::__private225::Err(
                                            _serde::de::Error::unknown_variant(__value, VARIANTS),
                                        )
                                    }
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        const VARIANTS: &'static [&'static str] = &[
                            "Certificate",
                            "Session",
                            "Get",
                            "Stream",
                            "Post",
                            "Patch",
                            "Put",
                            "Delete",
                            "Subscribe",
                            "Unsubscribe",
                            "Register",
                            "Identify",
                            "Proxy",
                            "_Reserved13",
                            "_Reserved14",
                            "Error",
                        ];
                        let (__tag, __content) = _serde::Deserializer::deserialize_any(
                            __deserializer,
                            _serde::__private225::de::TaggedContentVisitor::<
                                __Field,
                            >::new(
                                "packet_type",
                                "internally tagged enum ResponsePacketType",
                            ),
                        )?;
                        let __deserializer = _serde::__private225::de::ContentDeserializer::<
                            __D::Error,
                        >::new(__content);
                        match __tag {
                            __Field::__field0 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Certificate",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Certificate)
                            }
                            __Field::__field1 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "with_psk" => _serde::__private225::Ok(__Field::__field0),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"with_psk" => _serde::__private225::Ok(__Field::__field0),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        ResponsePacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ResponsePacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant ResponsePacketType::Session",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant ResponsePacketType::Session with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(ResponsePacketType::Session {
                                            with_psk: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "with_psk",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("with_psk")?
                                            }
                                        };
                                        _serde::__private225::Ok(ResponsePacketType::Session {
                                            with_psk: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["with_psk"];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            ResponsePacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field2 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "binary_keys" => _serde::__private225::Ok(__Field::__field0),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"binary_keys" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        ResponsePacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ResponsePacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant ResponsePacketType::Get",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant ResponsePacketType::Get with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(ResponsePacketType::Get {
                                            binary_keys: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "binary_keys",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("binary_keys")?
                                            }
                                        };
                                        _serde::__private225::Ok(ResponsePacketType::Get {
                                            binary_keys: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &["binary_keys"];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            ResponsePacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field3 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Stream",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Stream)
                            }
                            __Field::__field4 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Post",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Post)
                            }
                            __Field::__field5 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Patch",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Patch)
                            }
                            __Field::__field6 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Put",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Put)
                            }
                            __Field::__field7 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Delete",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Delete)
                            }
                            __Field::__field8 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Subscribe",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Subscribe)
                            }
                            __Field::__field9 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Unsubscribe",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Unsubscribe)
                            }
                            __Field::__field10 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Register",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Register)
                            }
                            __Field::__field11 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Identify",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Identify)
                            }
                            __Field::__field12 => {
                                #[allow(non_camel_case_types)]
                                #[doc(hidden)]
                                enum __Field {
                                    __field0,
                                    __ignore,
                                }
                                #[doc(hidden)]
                                struct __FieldVisitor;
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                    type Value = __Field;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "field identifier",
                                        )
                                    }
                                    fn visit_u64<__E>(
                                        self,
                                        __value: u64,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            0u64 => _serde::__private225::Ok(__Field::__field0),
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_str<__E>(
                                        self,
                                        __value: &str,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            "include_hop_info" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                    fn visit_bytes<__E>(
                                        self,
                                        __value: &[u8],
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        match __value {
                                            b"include_hop_info" => {
                                                _serde::__private225::Ok(__Field::__field0)
                                            }
                                            _ => _serde::__private225::Ok(__Field::__ignore),
                                        }
                                    }
                                }
                                #[automatically_derived]
                                impl<'de> _serde::Deserialize<'de> for __Field {
                                    #[inline]
                                    fn deserialize<__D>(
                                        __deserializer: __D,
                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                    where
                                        __D: _serde::Deserializer<'de>,
                                    {
                                        _serde::Deserializer::deserialize_identifier(
                                            __deserializer,
                                            __FieldVisitor,
                                        )
                                    }
                                }
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        ResponsePacketType,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ResponsePacketType;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant ResponsePacketType::Proxy",
                                        )
                                    }
                                    #[inline]
                                    fn visit_seq<__A>(
                                        self,
                                        mut __seq: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::SeqAccess<'de>,
                                    {
                                        let __field0 = match _serde::de::SeqAccess::next_element::<
                                            bool,
                                        >(&mut __seq)? {
                                            _serde::__private225::Some(__value) => __value,
                                            _serde::__private225::None => {
                                                return _serde::__private225::Err(
                                                    _serde::de::Error::invalid_length(
                                                        0usize,
                                                        &"struct variant ResponsePacketType::Proxy with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(ResponsePacketType::Proxy {
                                            include_hop_info: __field0,
                                        })
                                    }
                                    #[inline]
                                    fn visit_map<__A>(
                                        self,
                                        mut __map: __A,
                                    ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                    where
                                        __A: _serde::de::MapAccess<'de>,
                                    {
                                        let mut __field0: _serde::__private225::Option<bool> = _serde::__private225::None;
                                        while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                            __Field,
                                        >(&mut __map)? {
                                            match __key {
                                                __Field::__field0 => {
                                                    if _serde::__private225::Option::is_some(&__field0) {
                                                        return _serde::__private225::Err(
                                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                                "include_hop_info",
                                                            ),
                                                        );
                                                    }
                                                    __field0 = _serde::__private225::Some(
                                                        _serde::de::MapAccess::next_value::<bool>(&mut __map)?,
                                                    );
                                                }
                                                _ => {
                                                    let _ = _serde::de::MapAccess::next_value::<
                                                        _serde::de::IgnoredAny,
                                                    >(&mut __map)?;
                                                }
                                            }
                                        }
                                        let __field0 = match __field0 {
                                            _serde::__private225::Some(__field0) => __field0,
                                            _serde::__private225::None => {
                                                _serde::__private225::de::missing_field("include_hop_info")?
                                            }
                                        };
                                        _serde::__private225::Ok(ResponsePacketType::Proxy {
                                            include_hop_info: __field0,
                                        })
                                    }
                                }
                                #[doc(hidden)]
                                const FIELDS: &'static [&'static str] = &[
                                    "include_hop_info",
                                ];
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            ResponsePacketType,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field13 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "_Reserved13",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::_Reserved13)
                            }
                            __Field::__field14 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "_Reserved14",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::_Reserved14)
                            }
                            __Field::__field15 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketType",
                                        "Error",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketType::Error)
                            }
                        }
                    }
                }
            };
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ResponsePacketType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ResponsePacketType {
                #[inline]
                fn eq(&self, other: &ResponsePacketType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ResponsePacketType::Session { with_psk: __self_0 },
                                ResponsePacketType::Session { with_psk: __arg1_0 },
                            ) => __self_0 == __arg1_0,
                            (
                                ResponsePacketType::Get { binary_keys: __self_0 },
                                ResponsePacketType::Get { binary_keys: __arg1_0 },
                            ) => __self_0 == __arg1_0,
                            (
                                ResponsePacketType::Proxy { include_hop_info: __self_0 },
                                ResponsePacketType::Proxy { include_hop_info: __arg1_0 },
                            ) => __self_0 == __arg1_0,
                            _ => true,
                        }
                }
            }
        }
    }
    pub mod body {
        use binary_codec::{DeserializationError, SerializationError, SerializerConfig};
        use crate::packets::{
            base::PlabblePacketBase,
            header::{
                request_header::PlabbleRequestHeader,
                response_header::PlabbleResponseHeader,
            },
        };
        mod session {
            use binary_codec::{
                BinarySerializer, DeserializationError, SerializationError, utils::slice,
            };
            use serde::{Deserialize, Serialize};
            use serde_with::serde_as;
            use serde_with::base64::{Base64, UrlSafe};
            use serde_with::formats::Unpadded;
            use crate::packets::base::crypto_keys::CryptoSignature;
            use crate::packets::body::SerializableResponseBody;
            use crate::packets::{
                base::crypto_keys::CryptoKey, body::SerializableRequestBody,
                header::type_and_flags::RequestPacketType,
            };
            /// Session request body
            pub struct SessionRequestBody {
                /// PSK expiration Plabble timestamp. Filled if request flag persist_key is set.
                psk_expiration: Option<[u8; 4]>,
                /// Public/encapsulation keys for creating a shared secret with the server
                keys: Vec<CryptoKey>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for SessionRequestBody {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "SessionRequestBody",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "psk_expiration",
                            &self.psk_expiration,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "keys",
                            &self.keys,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for SessionRequestBody {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field0),
                                    1u64 => _serde::__private225::Ok(__Field::__field1),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "psk_expiration" => {
                                        _serde::__private225::Ok(__Field::__field0)
                                    }
                                    "keys" => _serde::__private225::Ok(__Field::__field1),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"psk_expiration" => {
                                        _serde::__private225::Ok(__Field::__field0)
                                    }
                                    b"keys" => _serde::__private225::Ok(__Field::__field1),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private225::PhantomData<
                                SessionRequestBody,
                            >,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SessionRequestBody;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "struct SessionRequestBody",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match _serde::de::SeqAccess::next_element::<
                                    Option<[u8; 4]>,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct SessionRequestBody with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Vec<CryptoKey>,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct SessionRequestBody with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private225::Ok(SessionRequestBody {
                                    psk_expiration: __field0,
                                    keys: __field1,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private225::Option<
                                    Option<[u8; 4]>,
                                > = _serde::__private225::None;
                                let mut __field1: _serde::__private225::Option<
                                    Vec<CryptoKey>,
                                > = _serde::__private225::None;
                                while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private225::Option::is_some(&__field0) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "psk_expiration",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<[u8; 4]>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private225::Option::is_some(&__field1) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("keys"),
                                                );
                                            }
                                            __field1 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Vec<CryptoKey>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private225::Some(__field0) => __field0,
                                    _serde::__private225::None => {
                                        _serde::__private225::de::missing_field("psk_expiration")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private225::Some(__field1) => __field1,
                                    _serde::__private225::None => {
                                        _serde::__private225::de::missing_field("keys")?
                                    }
                                };
                                _serde::__private225::Ok(SessionRequestBody {
                                    psk_expiration: __field0,
                                    keys: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "psk_expiration",
                            "keys",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SessionRequestBody",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<
                                    SessionRequestBody,
                                >,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            impl SerializableRequestBody for SessionRequestBody {
                fn to_bytes(
                    &self,
                    context: super::RequestSerializationContext,
                ) -> Result<Vec<u8>, SerializationError> {
                    let mut bytes: Vec<u8> = Vec::new();
                    if let RequestPacketType::Session { persist_key, .. } = context
                        .header
                        .packet_type
                    {
                        if persist_key && self.psk_expiration.is_none() {
                            return Err(
                                SerializationError::InvalidData(
                                    String::from(
                                        "psk_expiration should be set if persist_key flag is set",
                                    ),
                                ),
                            );
                        }
                        if let Some(expiration_bytes) = self.psk_expiration {
                            bytes.extend_from_slice(&expiration_bytes);
                            context.config.pos += 4;
                        }
                    } else {
                        return Err(
                            SerializationError::InvalidData(
                                String::from("Header type did not match body"),
                            ),
                        );
                    }
                    let crypto_settings = context
                        .packet
                        .crypto_settings
                        .clone()
                        .unwrap_or_default();
                    let key_types = CryptoKey::get_key_exchange_key_types(
                        &crypto_settings,
                        true,
                    );
                    CryptoKey::verify_keys(key_types, &self.keys)?;
                    for key in self.keys.iter() {
                        key.write_bytes(&mut bytes, Some(context.config))?;
                    }
                    Ok(bytes)
                }
                fn from_bytes(
                    bytes: &[u8],
                    context: super::RequestSerializationContext,
                ) -> Result<Self, DeserializationError>
                where
                    Self: Sized,
                {
                    if let RequestPacketType::Session { persist_key, .. } = context
                        .header
                        .packet_type
                    {
                        let psk_expiration = if persist_key {
                            Some(
                                slice(context.config, bytes, 4, true)?.try_into().unwrap(),
                            )
                        } else {
                            None
                        };
                        let crypto_settings = context
                            .packet
                            .crypto_settings
                            .clone()
                            .unwrap_or_default();
                        let key_types = CryptoKey::get_key_exchange_key_types(
                            &crypto_settings,
                            true,
                        );
                        let keys = CryptoKey::read_keys(
                            bytes,
                            key_types,
                            context.config,
                        )?;
                        Ok(Self { psk_expiration, keys })
                    } else {
                        Err(
                            DeserializationError::InvalidData(
                                String::from("Header type did not match body"),
                            ),
                        )
                    }
                }
            }
            pub struct SessionResponseBody {
                #[serde_as(r#as = "Option<Base64<UrlSafe, Unpadded>>")]
                #[serde(default)]
                #[serde(
                    with = ":: serde_with :: As :: < Option < Base64 < UrlSafe, Unpadded > > >"
                )]
                psk_id: Option<[u8; 12]>,
                /// Public keys or encapsulated secret for creating a shared secret
                keys: Vec<CryptoKey>,
                /// Signatures of the request
                signatures: Vec<CryptoSignature>,
            }
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl _serde::Serialize for SessionResponseBody {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "SessionResponseBody",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "psk_id",
                            &{
                                #[doc(hidden)]
                                struct __SerializeWith<'__a> {
                                    values: (&'__a Option<[u8; 12]>,),
                                    phantom: _serde::__private225::PhantomData<
                                        SessionResponseBody,
                                    >,
                                }
                                #[automatically_derived]
                                impl<'__a> _serde::Serialize for __SerializeWith<'__a> {
                                    fn serialize<__S>(
                                        &self,
                                        __s: __S,
                                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                                    where
                                        __S: _serde::Serializer,
                                    {
                                        ::serde_with::As::<
                                            Option<Base64<UrlSafe, Unpadded>>,
                                        >::serialize(self.values.0, __s)
                                    }
                                }
                                __SerializeWith {
                                    values: (&self.psk_id,),
                                    phantom: _serde::__private225::PhantomData::<
                                        SessionResponseBody,
                                    >,
                                }
                            },
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "keys",
                            &self.keys,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "signatures",
                            &self.signatures,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
                    }
                }
            };
            #[doc(hidden)]
            #[allow(
                non_upper_case_globals,
                unused_attributes,
                unused_qualifications,
                clippy::absolute_paths,
            )]
            const _: () = {
                #[allow(unused_extern_crates, clippy::useless_attribute)]
                extern crate serde as _serde;
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for SessionResponseBody {
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private225::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        #[allow(non_camel_case_types)]
                        #[doc(hidden)]
                        enum __Field {
                            __field0,
                            __field1,
                            __field2,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                            type Value = __Field;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "field identifier",
                                )
                            }
                            fn visit_u64<__E>(
                                self,
                                __value: u64,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    0u64 => _serde::__private225::Ok(__Field::__field0),
                                    1u64 => _serde::__private225::Ok(__Field::__field1),
                                    2u64 => _serde::__private225::Ok(__Field::__field2),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_str<__E>(
                                self,
                                __value: &str,
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    "psk_id" => _serde::__private225::Ok(__Field::__field0),
                                    "keys" => _serde::__private225::Ok(__Field::__field1),
                                    "signatures" => _serde::__private225::Ok(__Field::__field2),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                            fn visit_bytes<__E>(
                                self,
                                __value: &[u8],
                            ) -> _serde::__private225::Result<Self::Value, __E>
                            where
                                __E: _serde::de::Error,
                            {
                                match __value {
                                    b"psk_id" => _serde::__private225::Ok(__Field::__field0),
                                    b"keys" => _serde::__private225::Ok(__Field::__field1),
                                    b"signatures" => _serde::__private225::Ok(__Field::__field2),
                                    _ => _serde::__private225::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for __Field {
                            #[inline]
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                _serde::Deserializer::deserialize_identifier(
                                    __deserializer,
                                    __FieldVisitor,
                                )
                            }
                        }
                        #[doc(hidden)]
                        struct __Visitor<'de> {
                            marker: _serde::__private225::PhantomData<
                                SessionResponseBody,
                            >,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = SessionResponseBody;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "struct SessionResponseBody",
                                )
                            }
                            #[inline]
                            fn visit_seq<__A>(
                                self,
                                mut __seq: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::SeqAccess<'de>,
                            {
                                let __field0 = match {
                                    #[doc(hidden)]
                                    struct __DeserializeWith<'de> {
                                        value: Option<[u8; 12]>,
                                        phantom: _serde::__private225::PhantomData<
                                            SessionResponseBody,
                                        >,
                                        lifetime: _serde::__private225::PhantomData<&'de ()>,
                                    }
                                    #[automatically_derived]
                                    impl<'de> _serde::Deserialize<'de>
                                    for __DeserializeWith<'de> {
                                        fn deserialize<__D>(
                                            __deserializer: __D,
                                        ) -> _serde::__private225::Result<Self, __D::Error>
                                        where
                                            __D: _serde::Deserializer<'de>,
                                        {
                                            _serde::__private225::Ok(__DeserializeWith {
                                                value: ::serde_with::As::<
                                                    Option<Base64<UrlSafe, Unpadded>>,
                                                >::deserialize(__deserializer)?,
                                                phantom: _serde::__private225::PhantomData,
                                                lifetime: _serde::__private225::PhantomData,
                                            })
                                        }
                                    }
                                    _serde::__private225::Option::map(
                                        _serde::de::SeqAccess::next_element::<
                                            __DeserializeWith<'de>,
                                        >(&mut __seq)?,
                                        |__wrap| __wrap.value,
                                    )
                                } {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Vec<CryptoKey>,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct SessionResponseBody with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    Vec<CryptoSignature>,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct SessionResponseBody with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private225::Ok(SessionResponseBody {
                                    psk_id: __field0,
                                    keys: __field1,
                                    signatures: __field2,
                                })
                            }
                            #[inline]
                            fn visit_map<__A>(
                                self,
                                mut __map: __A,
                            ) -> _serde::__private225::Result<Self::Value, __A::Error>
                            where
                                __A: _serde::de::MapAccess<'de>,
                            {
                                let mut __field0: _serde::__private225::Option<
                                    Option<[u8; 12]>,
                                > = _serde::__private225::None;
                                let mut __field1: _serde::__private225::Option<
                                    Vec<CryptoKey>,
                                > = _serde::__private225::None;
                                let mut __field2: _serde::__private225::Option<
                                    Vec<CryptoSignature>,
                                > = _serde::__private225::None;
                                while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private225::Option::is_some(&__field0) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("psk_id"),
                                                );
                                            }
                                            __field0 = _serde::__private225::Some({
                                                #[doc(hidden)]
                                                struct __DeserializeWith<'de> {
                                                    value: Option<[u8; 12]>,
                                                    phantom: _serde::__private225::PhantomData<
                                                        SessionResponseBody,
                                                    >,
                                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                                }
                                                #[automatically_derived]
                                                impl<'de> _serde::Deserialize<'de>
                                                for __DeserializeWith<'de> {
                                                    fn deserialize<__D>(
                                                        __deserializer: __D,
                                                    ) -> _serde::__private225::Result<Self, __D::Error>
                                                    where
                                                        __D: _serde::Deserializer<'de>,
                                                    {
                                                        _serde::__private225::Ok(__DeserializeWith {
                                                            value: ::serde_with::As::<
                                                                Option<Base64<UrlSafe, Unpadded>>,
                                                            >::deserialize(__deserializer)?,
                                                            phantom: _serde::__private225::PhantomData,
                                                            lifetime: _serde::__private225::PhantomData,
                                                        })
                                                    }
                                                }
                                                match _serde::de::MapAccess::next_value::<
                                                    __DeserializeWith<'de>,
                                                >(&mut __map) {
                                                    _serde::__private225::Ok(__wrapper) => __wrapper.value,
                                                    _serde::__private225::Err(__err) => {
                                                        return _serde::__private225::Err(__err);
                                                    }
                                                }
                                            });
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private225::Option::is_some(&__field1) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("keys"),
                                                );
                                            }
                                            __field1 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Vec<CryptoKey>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private225::Option::is_some(&__field2) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "signatures",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Vec<CryptoSignature>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        _ => {
                                            let _ = _serde::de::MapAccess::next_value::<
                                                _serde::de::IgnoredAny,
                                            >(&mut __map)?;
                                        }
                                    }
                                }
                                let __field0 = match __field0 {
                                    _serde::__private225::Some(__field0) => __field0,
                                    _serde::__private225::None => {
                                        _serde::__private225::Default::default()
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private225::Some(__field1) => __field1,
                                    _serde::__private225::None => {
                                        _serde::__private225::de::missing_field("keys")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private225::Some(__field2) => __field2,
                                    _serde::__private225::None => {
                                        _serde::__private225::de::missing_field("signatures")?
                                    }
                                };
                                _serde::__private225::Ok(SessionResponseBody {
                                    psk_id: __field0,
                                    keys: __field1,
                                    signatures: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "psk_id",
                            "keys",
                            "signatures",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "SessionResponseBody",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<
                                    SessionResponseBody,
                                >,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            impl SerializableResponseBody for SessionResponseBody {
                fn to_bytes(
                    &self,
                    context: super::ResponseSerializationContext,
                ) -> Result<Vec<u8>, SerializationError> {
                    ::core::panicking::panic("not yet implemented")
                }
                fn from_bytes(
                    bytes: &[u8],
                    context: super::ResponseSerializationContext,
                ) -> Result<Self, DeserializationError>
                where
                    Self: Sized,
                {
                    ::core::panicking::panic("not yet implemented")
                }
            }
        }
        pub struct RequestSerializationContext<'a> {
            pub header: &'a PlabbleRequestHeader,
            pub packet: &'a PlabblePacketBase,
            pub config: &'a mut SerializerConfig,
        }
        pub struct ResponseSerializationContext<'a> {
            pub header: &'a PlabbleResponseHeader,
            pub packet: &'a PlabblePacketBase,
            pub config: &'a mut SerializerConfig,
        }
        pub trait SerializableRequestBody {
            fn to_bytes(
                &self,
                context: RequestSerializationContext,
            ) -> Result<Vec<u8>, SerializationError>;
            fn from_bytes(
                bytes: &[u8],
                context: RequestSerializationContext,
            ) -> Result<Self, DeserializationError>
            where
                Self: Sized;
        }
        pub trait SerializableResponseBody {
            fn to_bytes(
                &self,
                context: ResponseSerializationContext,
            ) -> Result<Vec<u8>, SerializationError>;
            fn from_bytes(
                bytes: &[u8],
                context: ResponseSerializationContext,
            ) -> Result<Self, DeserializationError>
            where
                Self: Sized;
        }
    }
    pub mod request {
        use binary_codec::{BinaryDeserializer, SerializerConfig};
        use serde::{Deserialize, Serialize};
        use crate::packets::{
            base::PlabblePacketBase, body::RequestSerializationContext,
            header::request_header::PlabbleRequestHeader,
        };
        pub struct PlabbleRequestPacket {
            #[serde(flatten)]
            base: PlabblePacketBase,
            header: PlabbleRequestHeader,
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Serialize for PlabbleRequestPacket {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_map(
                        __serializer,
                        _serde::__private225::None,
                    )?;
                    _serde::Serialize::serialize(
                        &&self.base,
                        _serde::__private225::ser::FlatMapSerializer(&mut __serde_state),
                    )?;
                    _serde::ser::SerializeMap::serialize_entry(
                        &mut __serde_state,
                        "header",
                        &self.header,
                    )?;
                    _serde::ser::SerializeMap::end(__serde_state)
                }
            }
        };
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for PlabbleRequestPacket {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private225::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field<'de> {
                        __field1,
                        __other(_serde::__private225::de::Content<'de>),
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field<'de>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private225::Formatter,
                        ) -> _serde::__private225::fmt::Result {
                            _serde::__private225::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_bool<__E>(
                            self,
                            __value: bool,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::Bool(__value),
                                ),
                            )
                        }
                        fn visit_i8<__E>(
                            self,
                            __value: i8,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::I8(__value),
                                ),
                            )
                        }
                        fn visit_i16<__E>(
                            self,
                            __value: i16,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::I16(__value),
                                ),
                            )
                        }
                        fn visit_i32<__E>(
                            self,
                            __value: i32,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::I32(__value),
                                ),
                            )
                        }
                        fn visit_i64<__E>(
                            self,
                            __value: i64,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::I64(__value),
                                ),
                            )
                        }
                        fn visit_u8<__E>(
                            self,
                            __value: u8,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::U8(__value),
                                ),
                            )
                        }
                        fn visit_u16<__E>(
                            self,
                            __value: u16,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::U16(__value),
                                ),
                            )
                        }
                        fn visit_u32<__E>(
                            self,
                            __value: u32,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::U32(__value),
                                ),
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::U64(__value),
                                ),
                            )
                        }
                        fn visit_f32<__E>(
                            self,
                            __value: f32,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::F32(__value),
                                ),
                            )
                        }
                        fn visit_f64<__E>(
                            self,
                            __value: f64,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::F64(__value),
                                ),
                            )
                        }
                        fn visit_char<__E>(
                            self,
                            __value: char,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(
                                    _serde::__private225::de::Content::Char(__value),
                                ),
                            )
                        }
                        fn visit_unit<__E>(
                            self,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            _serde::__private225::Ok(
                                __Field::__other(_serde::__private225::de::Content::Unit),
                            )
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "header" => _serde::__private225::Ok(__Field::__field1),
                                _ => {
                                    let __value = _serde::__private225::de::Content::String(
                                        _serde::__private225::ToString::to_string(__value),
                                    );
                                    _serde::__private225::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"header" => _serde::__private225::Ok(__Field::__field1),
                                _ => {
                                    let __value = _serde::__private225::de::Content::ByteBuf(
                                        __value.to_vec(),
                                    );
                                    _serde::__private225::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_borrowed_str<__E>(
                            self,
                            __value: &'de str,
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "header" => _serde::__private225::Ok(__Field::__field1),
                                _ => {
                                    let __value = _serde::__private225::de::Content::Str(
                                        __value,
                                    );
                                    _serde::__private225::Ok(__Field::__other(__value))
                                }
                            }
                        }
                        fn visit_borrowed_bytes<__E>(
                            self,
                            __value: &'de [u8],
                        ) -> _serde::__private225::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"header" => _serde::__private225::Ok(__Field::__field1),
                                _ => {
                                    let __value = _serde::__private225::de::Content::Bytes(
                                        __value,
                                    );
                                    _serde::__private225::Ok(__Field::__other(__value))
                                }
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private225::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private225::PhantomData<PlabbleRequestPacket>,
                        lifetime: _serde::__private225::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = PlabbleRequestPacket;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private225::Formatter,
                        ) -> _serde::__private225::fmt::Result {
                            _serde::__private225::Formatter::write_str(
                                __formatter,
                                "struct PlabbleRequestPacket",
                            )
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private225::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field1: _serde::__private225::Option<
                                PlabbleRequestHeader,
                            > = _serde::__private225::None;
                            let mut __collect = _serde::__private225::Vec::<
                                _serde::__private225::Option<
                                    (
                                        _serde::__private225::de::Content,
                                        _serde::__private225::de::Content,
                                    ),
                                >,
                            >::new();
                            while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field1 => {
                                        if _serde::__private225::Option::is_some(&__field1) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("header"),
                                            );
                                        }
                                        __field1 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<
                                                PlabbleRequestHeader,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__other(__name) => {
                                        __collect
                                            .push(
                                                _serde::__private225::Some((
                                                    __name,
                                                    _serde::de::MapAccess::next_value_seed(
                                                        &mut __map,
                                                        _serde::__private225::de::ContentVisitor::new(),
                                                    )?,
                                                )),
                                            );
                                    }
                                }
                            }
                            let __field1 = match __field1 {
                                _serde::__private225::Some(__field1) => __field1,
                                _serde::__private225::None => {
                                    _serde::__private225::de::missing_field("header")?
                                }
                            };
                            let __field0: PlabblePacketBase = _serde::de::Deserialize::deserialize(
                                _serde::__private225::de::FlatMapDeserializer(
                                    &mut __collect,
                                    _serde::__private225::PhantomData,
                                ),
                            )?;
                            _serde::__private225::Ok(PlabbleRequestPacket {
                                base: __field0,
                                header: __field1,
                            })
                        }
                    }
                    _serde::Deserializer::deserialize_map(
                        __deserializer,
                        __Visitor {
                            marker: _serde::__private225::PhantomData::<
                                PlabbleRequestPacket,
                            >,
                            lifetime: _serde::__private225::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for PlabbleRequestPacket {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for PlabbleRequestPacket {
            #[inline]
            fn eq(&self, other: &PlabbleRequestPacket) -> bool {
                self.base == other.base && self.header == other.header
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for PlabbleRequestPacket {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "PlabbleRequestPacket",
                    "base",
                    &self.base,
                    "header",
                    &&self.header,
                )
            }
        }
        impl BinaryDeserializer for PlabbleRequestPacket {
            fn from_bytes(
                bytes: &[u8],
                config: Option<&mut binary_codec::SerializerConfig>,
            ) -> Result<Self, binary_codec::DeserializationError> {
                let mut default_config = SerializerConfig::new();
                let config = config.unwrap_or(&mut default_config);
                let base = PlabblePacketBase::from_bytes(bytes, Some(config))?;
                let header = PlabbleRequestHeader::from_bytes(bytes, Some(config))?;
                config.reset_bits(true);
                let context = RequestSerializationContext {
                    header: &header,
                    packet: &base,
                    config,
                };
                Ok(Self { base, header })
            }
        }
    }
}
/// Default to true for serde boolean fields
fn default_true() -> bool {
    true
}
