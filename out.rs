#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod packets {
    pub mod base {
        use binary_codec::{FromBytes, ToBytes};
        use serde::{Deserialize, Serialize};
        use serde_with::serde_as;
        use serde_with::base64::{Base64, UrlSafe};
        use serde_with::formats::Unpadded;
        use settings::EncryptionSettings;
        mod flags {
            use binary_codec::{FromBytes, ToBytes};
            use serde::{Deserialize, Serialize};
            #[serde(tag = "packet_type")]
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
                    let _p_disc = _p_config
                        .discriminator
                        .take()
                        .unwrap_or(
                            binary_codec::fixed_int::FixedInt::read(_p_bytes, _p_config)?,
                        );
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
            #[no_discriminator]
            pub enum ResponsePacketFlags {
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
            impl ::core::fmt::Debug for ResponsePacketFlags {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        ResponsePacketFlags::Certificate => {
                            ::core::fmt::Formatter::write_str(f, "Certificate")
                        }
                        ResponsePacketFlags::Session { with_psk: __self_0 } => {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Session",
                                "with_psk",
                                &__self_0,
                            )
                        }
                        ResponsePacketFlags::Get { binary_keys: __self_0 } => {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Get",
                                "binary_keys",
                                &__self_0,
                            )
                        }
                        ResponsePacketFlags::Stream => {
                            ::core::fmt::Formatter::write_str(f, "Stream")
                        }
                        ResponsePacketFlags::Post => {
                            ::core::fmt::Formatter::write_str(f, "Post")
                        }
                        ResponsePacketFlags::Patch => {
                            ::core::fmt::Formatter::write_str(f, "Patch")
                        }
                        ResponsePacketFlags::Put => {
                            ::core::fmt::Formatter::write_str(f, "Put")
                        }
                        ResponsePacketFlags::Delete => {
                            ::core::fmt::Formatter::write_str(f, "Delete")
                        }
                        ResponsePacketFlags::Subscribe => {
                            ::core::fmt::Formatter::write_str(f, "Subscribe")
                        }
                        ResponsePacketFlags::Unsubscribe => {
                            ::core::fmt::Formatter::write_str(f, "Unsubscribe")
                        }
                        ResponsePacketFlags::Register => {
                            ::core::fmt::Formatter::write_str(f, "Register")
                        }
                        ResponsePacketFlags::Identify => {
                            ::core::fmt::Formatter::write_str(f, "Identify")
                        }
                        ResponsePacketFlags::Proxy { include_hop_info: __self_0 } => {
                            ::core::fmt::Formatter::debug_struct_field1_finish(
                                f,
                                "Proxy",
                                "include_hop_info",
                                &__self_0,
                            )
                        }
                        ResponsePacketFlags::_Reserved13 => {
                            ::core::fmt::Formatter::write_str(f, "_Reserved13")
                        }
                        ResponsePacketFlags::_Reserved14 => {
                            ::core::fmt::Formatter::write_str(f, "_Reserved14")
                        }
                        ResponsePacketFlags::Error => {
                            ::core::fmt::Formatter::write_str(f, "Error")
                        }
                    }
                }
            }
            impl binary_codec::BinaryDeserializer for ResponsePacketFlags {
                fn from_bytes(
                    bytes: &[u8],
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Self, binary_codec::DeserializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = bytes;
                    let _p_disc = _p_config
                        .discriminator
                        .take()
                        .unwrap_or(
                            binary_codec::fixed_int::FixedInt::read(_p_bytes, _p_config)?,
                        );
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
            impl binary_codec::BinarySerializer for ResponsePacketFlags {
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
                impl _serde::Serialize for ResponsePacketFlags {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            ResponsePacketFlags::Certificate => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Certificate",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Session { ref with_psk } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
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
                            ResponsePacketFlags::Get { ref binary_keys } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
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
                            ResponsePacketFlags::Stream => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Stream",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Post => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Post",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Patch => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Patch",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Put => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Put",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Delete => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Delete",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Subscribe => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Subscribe",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Unsubscribe => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Unsubscribe",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Register => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Register",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Identify => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "Identify",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Proxy { ref include_hop_info } => {
                                let mut __serde_state = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
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
                            ResponsePacketFlags::_Reserved13 => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "_Reserved13",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::_Reserved14 => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
                                    1,
                                )?;
                                _serde::ser::SerializeStruct::serialize_field(
                                    &mut __struct,
                                    "packet_type",
                                    "_Reserved14",
                                )?;
                                _serde::ser::SerializeStruct::end(__struct)
                            }
                            ResponsePacketFlags::Error => {
                                let mut __struct = _serde::Serializer::serialize_struct(
                                    __serializer,
                                    "ResponsePacketFlags",
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
                impl<'de> _serde::Deserialize<'de> for ResponsePacketFlags {
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
                                "internally tagged enum ResponsePacketFlags",
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
                                        "ResponsePacketFlags",
                                        "Certificate",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Certificate)
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
                                        ResponsePacketFlags,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ResponsePacketFlags;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant ResponsePacketFlags::Session",
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
                                                        &"struct variant ResponsePacketFlags::Session with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(ResponsePacketFlags::Session {
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
                                        _serde::__private225::Ok(ResponsePacketFlags::Session {
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
                                            ResponsePacketFlags,
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
                                        ResponsePacketFlags,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ResponsePacketFlags;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant ResponsePacketFlags::Get",
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
                                                        &"struct variant ResponsePacketFlags::Get with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(ResponsePacketFlags::Get {
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
                                        _serde::__private225::Ok(ResponsePacketFlags::Get {
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
                                            ResponsePacketFlags,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field3 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Stream",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Stream)
                            }
                            __Field::__field4 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Post",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Post)
                            }
                            __Field::__field5 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Patch",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Patch)
                            }
                            __Field::__field6 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Put",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Put)
                            }
                            __Field::__field7 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Delete",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Delete)
                            }
                            __Field::__field8 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Subscribe",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Subscribe)
                            }
                            __Field::__field9 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Unsubscribe",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Unsubscribe)
                            }
                            __Field::__field10 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Register",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Register)
                            }
                            __Field::__field11 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Identify",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Identify)
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
                                        ResponsePacketFlags,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = ResponsePacketFlags;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "struct variant ResponsePacketFlags::Proxy",
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
                                                        &"struct variant ResponsePacketFlags::Proxy with 1 element",
                                                    ),
                                                );
                                            }
                                        };
                                        _serde::__private225::Ok(ResponsePacketFlags::Proxy {
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
                                        _serde::__private225::Ok(ResponsePacketFlags::Proxy {
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
                                            ResponsePacketFlags,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                            __Field::__field13 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "_Reserved13",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::_Reserved13)
                            }
                            __Field::__field14 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "_Reserved14",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::_Reserved14)
                            }
                            __Field::__field15 => {
                                _serde::Deserializer::deserialize_any(
                                    __deserializer,
                                    _serde::__private225::de::InternallyTaggedUnitVisitor::new(
                                        "ResponsePacketFlags",
                                        "Error",
                                    ),
                                )?;
                                _serde::__private225::Ok(ResponsePacketFlags::Error)
                            }
                        }
                    }
                }
            };
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for ResponsePacketFlags {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for ResponsePacketFlags {
                #[inline]
                fn eq(&self, other: &ResponsePacketFlags) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                        && match (self, other) {
                            (
                                ResponsePacketFlags::Session { with_psk: __self_0 },
                                ResponsePacketFlags::Session { with_psk: __arg1_0 },
                            ) => __self_0 == __arg1_0,
                            (
                                ResponsePacketFlags::Get { binary_keys: __self_0 },
                                ResponsePacketFlags::Get { binary_keys: __arg1_0 },
                            ) => __self_0 == __arg1_0,
                            (
                                ResponsePacketFlags::Proxy { include_hop_info: __self_0 },
                                ResponsePacketFlags::Proxy { include_hop_info: __arg1_0 },
                            ) => __self_0 == __arg1_0,
                            _ => true,
                        }
                }
            }
            enum RequestBody {
                Session { keys: Vec<Vec<u8>>, psk_expiration: Option<u32> },
                Bucket { bucket_id: [u8; 16], from: Option<u32>, to: Option<u32> },
                BucketBinary { bucket_id: [u8; 16], from: Option<u32>, to: Option<u32> },
            }
        }
        pub mod header {
            use binary_codec::{FromBytes, ToBytes};
            use serde::{Deserialize, Serialize};
            /// Plabble Protocol packet types
            #[no_discriminator]
            pub enum PacketType {
                Certificate = 0,
                Session = 1,
                Get = 2,
                Stream = 3,
                Post = 4,
                Patch = 5,
                Put = 6,
                Delete = 7,
                Subscribe = 8,
                Unsubscribe = 9,
                Register = 10,
                Identify = 11,
                Proxy = 12,
                _Reserved13 = 13,
                _Reserved14 = 14,
                Error = 15,
            }
            impl binary_codec::BinarySerializer for PacketType {
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
                        Self::Session => {}
                        Self::Get => {}
                        Self::Stream => {}
                        Self::Post => {}
                        Self::Patch => {}
                        Self::Put => {}
                        Self::Delete => {}
                        Self::Subscribe => {}
                        Self::Unsubscribe => {}
                        Self::Register => {}
                        Self::Identify => {}
                        Self::Proxy => {}
                        Self::_Reserved13 => {}
                        Self::_Reserved14 => {}
                        Self::Error => {}
                    }
                    Ok(())
                }
            }
            impl binary_codec::BinaryDeserializer for PacketType {
                fn from_bytes(
                    bytes: &[u8],
                    config: Option<&mut binary_codec::SerializerConfig>,
                ) -> Result<Self, binary_codec::DeserializationError> {
                    let mut _new_config = binary_codec::SerializerConfig::new();
                    let _p_config = config.unwrap_or(&mut _new_config);
                    let _p_bytes = bytes;
                    let _p_disc = _p_config
                        .discriminator
                        .take()
                        .unwrap_or(
                            binary_codec::fixed_int::FixedInt::read(_p_bytes, _p_config)?,
                        );
                    match _p_disc {
                        0u8 => Ok(Self::Certificate),
                        1u8 => Ok(Self::Session),
                        2u8 => Ok(Self::Get),
                        3u8 => Ok(Self::Stream),
                        4u8 => Ok(Self::Post),
                        5u8 => Ok(Self::Patch),
                        6u8 => Ok(Self::Put),
                        7u8 => Ok(Self::Delete),
                        8u8 => Ok(Self::Subscribe),
                        9u8 => Ok(Self::Unsubscribe),
                        10u8 => Ok(Self::Register),
                        11u8 => Ok(Self::Identify),
                        12u8 => Ok(Self::Proxy),
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
                impl _serde::Serialize for PacketType {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        match *self {
                            PacketType::Certificate => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    0u32,
                                    "Certificate",
                                )
                            }
                            PacketType::Session => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    1u32,
                                    "Session",
                                )
                            }
                            PacketType::Get => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    2u32,
                                    "Get",
                                )
                            }
                            PacketType::Stream => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    3u32,
                                    "Stream",
                                )
                            }
                            PacketType::Post => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    4u32,
                                    "Post",
                                )
                            }
                            PacketType::Patch => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    5u32,
                                    "Patch",
                                )
                            }
                            PacketType::Put => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    6u32,
                                    "Put",
                                )
                            }
                            PacketType::Delete => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    7u32,
                                    "Delete",
                                )
                            }
                            PacketType::Subscribe => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    8u32,
                                    "Subscribe",
                                )
                            }
                            PacketType::Unsubscribe => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    9u32,
                                    "Unsubscribe",
                                )
                            }
                            PacketType::Register => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    10u32,
                                    "Register",
                                )
                            }
                            PacketType::Identify => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    11u32,
                                    "Identify",
                                )
                            }
                            PacketType::Proxy => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    12u32,
                                    "Proxy",
                                )
                            }
                            PacketType::_Reserved13 => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    13u32,
                                    "_Reserved13",
                                )
                            }
                            PacketType::_Reserved14 => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    14u32,
                                    "_Reserved14",
                                )
                            }
                            PacketType::Error => {
                                _serde::Serializer::serialize_unit_variant(
                                    __serializer,
                                    "PacketType",
                                    15u32,
                                    "Error",
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
                impl<'de> _serde::Deserialize<'de> for PacketType {
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
                        struct __Visitor<'de> {
                            marker: _serde::__private225::PhantomData<PacketType>,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = PacketType;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "enum PacketType",
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
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Certificate)
                                    }
                                    (__Field::__field1, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Session)
                                    }
                                    (__Field::__field2, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Get)
                                    }
                                    (__Field::__field3, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Stream)
                                    }
                                    (__Field::__field4, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Post)
                                    }
                                    (__Field::__field5, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Patch)
                                    }
                                    (__Field::__field6, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Put)
                                    }
                                    (__Field::__field7, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Delete)
                                    }
                                    (__Field::__field8, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Subscribe)
                                    }
                                    (__Field::__field9, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Unsubscribe)
                                    }
                                    (__Field::__field10, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Register)
                                    }
                                    (__Field::__field11, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Identify)
                                    }
                                    (__Field::__field12, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Proxy)
                                    }
                                    (__Field::__field13, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::_Reserved13)
                                    }
                                    (__Field::__field14, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::_Reserved14)
                                    }
                                    (__Field::__field15, __variant) => {
                                        _serde::de::VariantAccess::unit_variant(__variant)?;
                                        _serde::__private225::Ok(PacketType::Error)
                                    }
                                }
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
                        _serde::Deserializer::deserialize_enum(
                            __deserializer,
                            "PacketType",
                            VARIANTS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<PacketType>,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::fmt::Debug for PacketType {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::write_str(
                        f,
                        match self {
                            PacketType::Certificate => "Certificate",
                            PacketType::Session => "Session",
                            PacketType::Get => "Get",
                            PacketType::Stream => "Stream",
                            PacketType::Post => "Post",
                            PacketType::Patch => "Patch",
                            PacketType::Put => "Put",
                            PacketType::Delete => "Delete",
                            PacketType::Subscribe => "Subscribe",
                            PacketType::Unsubscribe => "Unsubscribe",
                            PacketType::Register => "Register",
                            PacketType::Identify => "Identify",
                            PacketType::Proxy => "Proxy",
                            PacketType::_Reserved13 => "_Reserved13",
                            PacketType::_Reserved14 => "_Reserved14",
                            PacketType::Error => "Error",
                        },
                    )
                }
            }
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for PacketType {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for PacketType {
                #[inline]
                fn eq(&self, other: &PacketType) -> bool {
                    let __self_discr = ::core::intrinsics::discriminant_value(self);
                    let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                    __self_discr == __arg1_discr
                }
            }
            #[automatically_derived]
            impl ::core::marker::Copy for PacketType {}
            #[automatically_derived]
            impl ::core::clone::Clone for PacketType {
                #[inline]
                fn clone(&self) -> PacketType {
                    *self
                }
            }
            /// Plabble Protocol packet header
            pub struct PlabblePacketHeader {
                /// Packet type
                #[serde(skip_serializing, skip_deserializing)]
                #[bits = 4]
                #[variant_for("packet_type")]
                _type: u8,
                /// Packet type (derived from `_type`)
                #[variant_by = "packet_type"]
                packet_type: PacketType,
                /// Packet flags, specific to the packet type
                flags: [bool; 4],
                /// If in a session, the counter of the request to respond to
                response_to: Option<u16>,
            }
            impl binary_codec::BinaryDeserializer for PlabblePacketHeader {
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
                    let mut __val_0 = Vec::<bool>::with_capacity(4usize);
                    for _ in 0..4usize {
                        let _p_val = binary_codec::dynamics::read_bool(
                            _p_bytes,
                            _p_config,
                        )?;
                        __val_0.push(_p_val);
                    }
                    let _p_val = TryInto::<[bool; 4usize]>::try_into(__val_0)
                        .expect("Failed to convert Vec to array");
                    let flags = _p_val;
                    let mut __option_0: Option<u16> = None;
                    if _p_config.next_reset_bits_pos() < _p_bytes.len() {
                        let _p_val = binary_codec::fixed_int::FixedInt::read(
                            _p_bytes,
                            _p_config,
                        )?;
                        __option_0 = Some(_p_val);
                    }
                    let _p_val = __option_0;
                    let response_to = _p_val;
                    Ok(Self {
                        _type,
                        packet_type,
                        flags,
                        response_to,
                    })
                }
            }
            impl binary_codec::BinarySerializer for PlabblePacketHeader {
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
                    let _p_val = &self.flags;
                    for _p_val in _p_val {
                        binary_codec::dynamics::write_bool(
                            *_p_val,
                            _p_bytes,
                            _p_config,
                        )?;
                    }
                    let _p_val = &self.response_to;
                    if let Some(_p_val) = _p_val.as_ref() {
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
                impl _serde::Serialize for PlabblePacketHeader {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "PlabblePacketHeader",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "packet_type",
                            &self.packet_type,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "flags",
                            &self.flags,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "response_to",
                            &self.response_to,
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
                impl<'de> _serde::Deserialize<'de> for PlabblePacketHeader {
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
                                    0u64 => _serde::__private225::Ok(__Field::__field1),
                                    1u64 => _serde::__private225::Ok(__Field::__field2),
                                    2u64 => _serde::__private225::Ok(__Field::__field3),
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
                                    "flags" => _serde::__private225::Ok(__Field::__field2),
                                    "response_to" => _serde::__private225::Ok(__Field::__field3),
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
                                    b"flags" => _serde::__private225::Ok(__Field::__field2),
                                    b"response_to" => {
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
                                PlabblePacketHeader,
                            >,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = PlabblePacketHeader;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "struct PlabblePacketHeader",
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
                                    PacketType,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct PlabblePacketHeader with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    [bool; 4],
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct PlabblePacketHeader with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field3 = match _serde::de::SeqAccess::next_element::<
                                    Option<u16>,
                                >(&mut __seq)? {
                                    _serde::__private225::Some(__value) => __value,
                                    _serde::__private225::None => {
                                        return _serde::__private225::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct PlabblePacketHeader with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private225::Ok(PlabblePacketHeader {
                                    _type: __field0,
                                    packet_type: __field1,
                                    flags: __field2,
                                    response_to: __field3,
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
                                    PacketType,
                                > = _serde::__private225::None;
                                let mut __field2: _serde::__private225::Option<[bool; 4]> = _serde::__private225::None;
                                let mut __field3: _serde::__private225::Option<
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
                                                _serde::de::MapAccess::next_value::<PacketType>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private225::Option::is_some(&__field2) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("flags"),
                                                );
                                            }
                                            __field2 = _serde::__private225::Some(
                                                _serde::de::MapAccess::next_value::<[bool; 4]>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private225::Option::is_some(&__field3) {
                                                return _serde::__private225::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "response_to",
                                                    ),
                                                );
                                            }
                                            __field3 = _serde::__private225::Some(
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
                                        _serde::__private225::de::missing_field("flags")?
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private225::Some(__field3) => __field3,
                                    _serde::__private225::None => {
                                        _serde::__private225::de::missing_field("response_to")?
                                    }
                                };
                                _serde::__private225::Ok(PlabblePacketHeader {
                                    _type: _serde::__private225::Default::default(),
                                    packet_type: __field1,
                                    flags: __field2,
                                    response_to: __field3,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "packet_type",
                            "flags",
                            "response_to",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "PlabblePacketHeader",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<
                                    PlabblePacketHeader,
                                >,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for PlabblePacketHeader {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for PlabblePacketHeader {
                #[inline]
                fn eq(&self, other: &PlabblePacketHeader) -> bool {
                    self._type == other._type && self.packet_type == other.packet_type
                        && self.flags == other.flags
                        && self.response_to == other.response_to
                }
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for PlabblePacketHeader {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "PlabblePacketHeader",
                        "_type",
                        &self._type,
                        "packet_type",
                        &self.packet_type,
                        "flags",
                        &self.flags,
                        "response_to",
                        &&self.response_to,
                    )
                }
            }
            impl PlabblePacketHeader {
                /// Pre-process the header before binary serialization
                pub fn pre_process(mut self) -> Self {
                    self._type = self.packet_type as u8;
                    self
                }
            }
        }
        pub mod settings {
            use binary_codec::{FromBytes, ToBytes};
            use serde::{Deserialize, Serialize};
            use crate::default_true;
            pub struct EncryptionSettings {
                /// If true, encrypt with ChaCha20 (Poly1305).
                /// This is the default if no encryption settings are specified.
                #[serde(default = "default_true")]
                encrypt_with_cha_cha20: bool,
                /// If true, encrypt with AES-CTR or AES-GCM.
                #[serde(default)]
                encrypt_with_aes_ctr: bool,
                /// Use 32-byte hashes instead of 16-byte ones.
                #[serde(default)]
                larger_hashes: bool,
                /// Use Blake3 for hashing, MAC and key derivation instead of Blake2.
                #[serde(default)]
                use_blake3: bool,
                /// Sign with Ed25519 (default)
                #[serde(default = "default_true")]
                sign_ed25519: bool,
                /// Key exchange with X25519 (default)
                #[serde(default = "default_true")]
                key_exchange_x25519: bool,
                /// Reserved for future use
                #[serde(default)]
                flag_64: bool,
                /// Use post-quantum cryptography (e.g., Kyber etc.)
                /// This adds the Post-Quantum settings
                #[serde(default)]
                use_post_quantum: bool,
                /// Post-Quantum settings
                #[toggled_by = "use_post_quantum"]
                post_quantum_settings: Option<PostQuantumSettings>,
            }
            impl binary_codec::BinaryDeserializer for EncryptionSettings {
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
                    let encrypt_with_aes_ctr = _p_val;
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
                    let mut __option_0: Option<PostQuantumSettings> = None;
                    if _p_config.get_toggle("use_post_quantum").unwrap_or(false) {
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
                        encrypt_with_aes_ctr,
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
            impl binary_codec::BinarySerializer for EncryptionSettings {
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
                    let _p_val = &self.encrypt_with_aes_ctr;
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
                    binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                    let _p_val = &self.post_quantum_settings;
                    if _p_config.get_toggle("use_post_quantum").unwrap_or(false) {
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
                impl _serde::Serialize for EncryptionSettings {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "EncryptionSettings",
                            false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "encrypt_with_cha_cha20",
                            &self.encrypt_with_cha_cha20,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "encrypt_with_aes_ctr",
                            &self.encrypt_with_aes_ctr,
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
                impl<'de> _serde::Deserialize<'de> for EncryptionSettings {
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
                                    "encrypt_with_aes_ctr" => {
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
                                    b"encrypt_with_aes_ctr" => {
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
                            marker: _serde::__private225::PhantomData<
                                EncryptionSettings,
                            >,
                            lifetime: _serde::__private225::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = EncryptionSettings;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private225::Formatter,
                            ) -> _serde::__private225::fmt::Result {
                                _serde::__private225::Formatter::write_str(
                                    __formatter,
                                    "struct EncryptionSettings",
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
                                                &"struct EncryptionSettings with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private225::Ok(EncryptionSettings {
                                    encrypt_with_cha_cha20: __field0,
                                    encrypt_with_aes_ctr: __field1,
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
                                                        "encrypt_with_aes_ctr",
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
                                _serde::__private225::Ok(EncryptionSettings {
                                    encrypt_with_cha_cha20: __field0,
                                    encrypt_with_aes_ctr: __field1,
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
                            "encrypt_with_aes_ctr",
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
                            "EncryptionSettings",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private225::PhantomData::<
                                    EncryptionSettings,
                                >,
                                lifetime: _serde::__private225::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl ::core::marker::StructuralPartialEq for EncryptionSettings {}
            #[automatically_derived]
            impl ::core::cmp::PartialEq for EncryptionSettings {
                #[inline]
                fn eq(&self, other: &EncryptionSettings) -> bool {
                    self.encrypt_with_cha_cha20 == other.encrypt_with_cha_cha20
                        && self.encrypt_with_aes_ctr == other.encrypt_with_aes_ctr
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
            impl ::core::fmt::Debug for EncryptionSettings {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "encrypt_with_cha_cha20",
                        "encrypt_with_aes_ctr",
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
                        &self.encrypt_with_aes_ctr,
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
                        "EncryptionSettings",
                        names,
                        values,
                    )
                }
            }
            pub struct PostQuantumSettings {
                /// Sign with ML-DSA-44, public key size 1312 B, signature 2420 B.
                /// Super fast, NIST level 1 security.
                #[serde(default)]
                sign_pqc_dsa_44: bool,
                /// Sign with ML-DSA-65, public key size 1952 B, signature 3309 B.
                /// Super fast, NIST level 3 security.
                #[serde(default)]
                sign_pqc_dsa_65: bool,
                /// Sign with Falcon-1024, public key size 1793 B, signature 1462 B.
                /// 3x slower than ML-DSA, NIST level 5 security.
                #[serde(default)]
                sign_pqc_falcon: bool,
                /// Sign with SLH-DSA-SHA128s, public key size 32 B, signature 7856 B.
                /// Very slow, but might be more secure because its based on hash functions only.
                /// NIST level 1 security.
                #[serde(default)]
                sign_pqc_slh_dsa: bool,
                /// Use ML-KEM-512 for key exchange, public key size 800 B, ciphertext size 768 B
                #[serde(default)]
                key_exchange_pqc_kem_512: bool,
                /// Use ML-KEM-768 for key exchange, public key size 1184 B, ciphertext size 1088 B
                #[serde(default)]
                key_exchange_pqc_kem_768: bool,
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
        }
        /// Plabble Protocol Packet
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
            #[toggles("pre_shared_key")]
            pre_shared_key: bool,
            /// If set to true, this packet uses encryption. If false, use a MAC (Message Authentication Code).
            #[serde(default)]
            #[toggles("encryption")]
            use_encryption: bool,
            /// If set to true, use custom encryption settings.
            #[serde(default)]
            #[toggles("encryption_settings")]
            specify_encryption_settings: bool,
            /// Encryption settings
            #[toggled_by = "encryption_settings"]
            encryption_settings: Option<EncryptionSettings>,
            /// Pre-shared key ID, if using a pre-shared key
            #[serde_as(r#as = "Option<Base64<UrlSafe, Unpadded>>")]
            #[toggled_by = "pre_shared_key"]
            #[serde(default)]
            #[serde(
                with = ":: serde_with :: As :: < Option < Base64 < UrlSafe, Unpadded > > >"
            )]
            psk_id: Option<[u8; 16]>,
            /// Pre-shared key salt, if using a pre-shared key
            #[serde_as(r#as = "Option<Base64<UrlSafe, Unpadded>>")]
            #[toggled_by = "pre_shared_key"]
            #[serde(default)]
            #[serde(
                with = ":: serde_with :: As :: < Option < Base64 < UrlSafe, Unpadded > > >"
            )]
            psk_salt: Option<[u8; 16]>,
            /// Message Authentication Code (MAC)
            #[serde_as(r#as = "Option<Base64<UrlSafe, Unpadded>>")]
            #[toggled_by = "!encryption"]
            #[serde(default)]
            #[serde(
                with = ":: serde_with :: As :: < Option < Base64 < UrlSafe, Unpadded > > >"
            )]
            mac: Option<[u8; 16]>,
            /// Packet payload, encrypted or not depending on the settings above.
            /// It also contains the encrypted part of the header
            #[serde(skip_serializing)]
            payload: Option<Vec<u8>>,
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
                let specify_encryption_settings = _p_val;
                _p_config.set_toggle("encryption_settings", _p_val);
                let mut __option_0: Option<EncryptionSettings> = None;
                if _p_config.get_toggle("encryption_settings").unwrap_or(false) {
                    let _p_val = binary_codec::variable::read_object(
                        _p_bytes,
                        None,
                        _p_config,
                    )?;
                    __option_0 = Some(_p_val);
                }
                let _p_val = __option_0;
                let encryption_settings = _p_val;
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
                let mut __option_0: Option<Vec<u8>> = None;
                if _p_config.next_reset_bits_pos() < _p_bytes.len() {
                    let mut __val_1 = Vec::<u8>::new();
                    while _p_config.next_reset_bits_pos() < _p_bytes.len() {
                        let _p_val = binary_codec::fixed_int::FixedInt::read(
                            _p_bytes,
                            _p_config,
                        )?;
                        __val_1.push(_p_val);
                    }
                    let _p_val = __val_1;
                    __option_0 = Some(_p_val);
                }
                let _p_val = __option_0;
                let payload = _p_val;
                Ok(Self {
                    version,
                    fire_and_forget,
                    pre_shared_key,
                    use_encryption,
                    specify_encryption_settings,
                    encryption_settings,
                    psk_id,
                    psk_salt,
                    mac,
                    payload,
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
                let _p_val = &self.specify_encryption_settings;
                _p_config.set_toggle("encryption_settings", *_p_val);
                binary_codec::dynamics::write_bool(*_p_val, _p_bytes, _p_config)?;
                let _p_val = &self.encryption_settings;
                if _p_config.get_toggle("encryption_settings").unwrap_or(false) {
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
                let _p_val = &self.payload;
                if let Some(_p_val) = _p_val.as_ref() {
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
                        "specify_encryption_settings",
                        &self.specify_encryption_settings,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "encryption_settings",
                        &self.encryption_settings,
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
                        __field9,
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
                                9u64 => _serde::__private225::Ok(__Field::__field9),
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
                                "specify_encryption_settings" => {
                                    _serde::__private225::Ok(__Field::__field4)
                                }
                                "encryption_settings" => {
                                    _serde::__private225::Ok(__Field::__field5)
                                }
                                "psk_id" => _serde::__private225::Ok(__Field::__field6),
                                "psk_salt" => _serde::__private225::Ok(__Field::__field7),
                                "mac" => _serde::__private225::Ok(__Field::__field8),
                                "payload" => _serde::__private225::Ok(__Field::__field9),
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
                                b"specify_encryption_settings" => {
                                    _serde::__private225::Ok(__Field::__field4)
                                }
                                b"encryption_settings" => {
                                    _serde::__private225::Ok(__Field::__field5)
                                }
                                b"psk_id" => _serde::__private225::Ok(__Field::__field6),
                                b"psk_salt" => _serde::__private225::Ok(__Field::__field7),
                                b"mac" => _serde::__private225::Ok(__Field::__field8),
                                b"payload" => _serde::__private225::Ok(__Field::__field9),
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
                                            &"struct PlabblePacketBase with 10 elements",
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
                                Option<EncryptionSettings>,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    return _serde::__private225::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct PlabblePacketBase with 10 elements",
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
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                Option<Vec<u8>>,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    return _serde::__private225::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct PlabblePacketBase with 10 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private225::Ok(PlabblePacketBase {
                                version: __field0,
                                fire_and_forget: __field1,
                                pre_shared_key: __field2,
                                use_encryption: __field3,
                                specify_encryption_settings: __field4,
                                encryption_settings: __field5,
                                psk_id: __field6,
                                psk_salt: __field7,
                                mac: __field8,
                                payload: __field9,
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
                                Option<EncryptionSettings>,
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
                            let mut __field9: _serde::__private225::Option<
                                Option<Vec<u8>>,
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
                                                    "specify_encryption_settings",
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
                                                    "encryption_settings",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<EncryptionSettings>,
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
                                    __Field::__field9 => {
                                        if _serde::__private225::Option::is_some(&__field9) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "payload",
                                                ),
                                            );
                                        }
                                        __field9 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Vec<u8>>,
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
                                    _serde::__private225::de::missing_field(
                                        "encryption_settings",
                                    )?
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
                            let __field9 = match __field9 {
                                _serde::__private225::Some(__field9) => __field9,
                                _serde::__private225::None => {
                                    _serde::__private225::de::missing_field("payload")?
                                }
                            };
                            _serde::__private225::Ok(PlabblePacketBase {
                                version: __field0,
                                fire_and_forget: __field1,
                                pre_shared_key: __field2,
                                use_encryption: __field3,
                                specify_encryption_settings: __field4,
                                encryption_settings: __field5,
                                psk_id: __field6,
                                psk_salt: __field7,
                                mac: __field8,
                                payload: __field9,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "version",
                        "fire_and_forget",
                        "pre_shared_key",
                        "use_encryption",
                        "specify_encryption_settings",
                        "encryption_settings",
                        "psk_id",
                        "psk_salt",
                        "mac",
                        "payload",
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
                    && self.specify_encryption_settings
                        == other.specify_encryption_settings
                    && self.encryption_settings == other.encryption_settings
                    && self.psk_id == other.psk_id && self.psk_salt == other.psk_salt
                    && self.mac == other.mac && self.payload == other.payload
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
                    "specify_encryption_settings",
                    "encryption_settings",
                    "psk_id",
                    "psk_salt",
                    "mac",
                    "payload",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.version,
                    &self.fire_and_forget,
                    &self.pre_shared_key,
                    &self.use_encryption,
                    &self.specify_encryption_settings,
                    &self.encryption_settings,
                    &self.psk_id,
                    &self.psk_salt,
                    &self.mac,
                    &&self.payload,
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
    pub mod body {
        use binary_codec::{FromBytes, ToBytes};
        use serde::{Deserialize, Serialize};
        struct SessionRequest {
            keys: Vec<Vec<u8>>,
            psk_expiration: Option<u32>,
        }
        impl binary_codec::BinarySerializer for SessionRequest {
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
                let _p_val = &self.keys;
                for _p_val in _p_val {
                    for _p_val in _p_val {
                        binary_codec::fixed_int::FixedInt::write(
                            *_p_val,
                            _p_bytes,
                            _p_config,
                        )?;
                    }
                }
                let _p_val = &self.psk_expiration;
                if let Some(_p_val) = _p_val.as_ref() {
                    binary_codec::fixed_int::FixedInt::write(
                        *_p_val,
                        _p_bytes,
                        _p_config,
                    )?;
                }
                Ok(())
            }
        }
        impl binary_codec::BinaryDeserializer for SessionRequest {
            fn from_bytes(
                bytes: &[u8],
                config: Option<&mut binary_codec::SerializerConfig>,
            ) -> Result<Self, binary_codec::DeserializationError> {
                let mut _new_config = binary_codec::SerializerConfig::new();
                let _p_config = config.unwrap_or(&mut _new_config);
                let _p_bytes = bytes;
                let mut __val_0 = Vec::<Vec<u8>>::new();
                while _p_config.next_reset_bits_pos() < _p_bytes.len() {
                    let mut __val_1 = Vec::<u8>::new();
                    while _p_config.next_reset_bits_pos() < _p_bytes.len() {
                        let _p_val = binary_codec::fixed_int::FixedInt::read(
                            _p_bytes,
                            _p_config,
                        )?;
                        __val_1.push(_p_val);
                    }
                    let _p_val = __val_1;
                    __val_0.push(_p_val);
                }
                let _p_val = __val_0;
                let keys = _p_val;
                let mut __option_0: Option<u32> = None;
                if _p_config.next_reset_bits_pos() < _p_bytes.len() {
                    let _p_val = binary_codec::fixed_int::FixedInt::read(
                        _p_bytes,
                        _p_config,
                    )?;
                    __option_0 = Some(_p_val);
                }
                let _p_val = __option_0;
                let psk_expiration = _p_val;
                Ok(Self { keys, psk_expiration })
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for SessionRequest {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "SessionRequest",
                    "keys",
                    &self.keys,
                    "psk_expiration",
                    &&self.psk_expiration,
                )
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for SessionRequest {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for SessionRequest {
            #[inline]
            fn eq(&self, other: &SessionRequest) -> bool {
                self.keys == other.keys && self.psk_expiration == other.psk_expiration
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
            impl _serde::Serialize for SessionRequest {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "SessionRequest",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "keys",
                        &self.keys,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "psk_expiration",
                        &self.psk_expiration,
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
            impl<'de> _serde::Deserialize<'de> for SessionRequest {
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
                                "keys" => _serde::__private225::Ok(__Field::__field0),
                                "psk_expiration" => {
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
                                b"keys" => _serde::__private225::Ok(__Field::__field0),
                                b"psk_expiration" => {
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
                        marker: _serde::__private225::PhantomData<SessionRequest>,
                        lifetime: _serde::__private225::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = SessionRequest;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private225::Formatter,
                        ) -> _serde::__private225::fmt::Result {
                            _serde::__private225::Formatter::write_str(
                                __formatter,
                                "struct SessionRequest",
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
                                Vec<Vec<u8>>,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    return _serde::__private225::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct SessionRequest with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Option<u32>,
                            >(&mut __seq)? {
                                _serde::__private225::Some(__value) => __value,
                                _serde::__private225::None => {
                                    return _serde::__private225::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct SessionRequest with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private225::Ok(SessionRequest {
                                keys: __field0,
                                psk_expiration: __field1,
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
                                Vec<Vec<u8>>,
                            > = _serde::__private225::None;
                            let mut __field1: _serde::__private225::Option<
                                Option<u32>,
                            > = _serde::__private225::None;
                            while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private225::Option::is_some(&__field0) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("keys"),
                                            );
                                        }
                                        __field0 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<Vec<u8>>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private225::Option::is_some(&__field1) {
                                            return _serde::__private225::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "psk_expiration",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private225::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<u32>,
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
                                    _serde::__private225::de::missing_field("keys")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private225::Some(__field1) => __field1,
                                _serde::__private225::None => {
                                    _serde::__private225::de::missing_field("psk_expiration")?
                                }
                            };
                            _serde::__private225::Ok(SessionRequest {
                                keys: __field0,
                                psk_expiration: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["keys", "psk_expiration"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "SessionRequest",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private225::PhantomData::<SessionRequest>,
                            lifetime: _serde::__private225::PhantomData,
                        },
                    )
                }
            }
        };
        struct SessionResponse {
            psk_id: Option<[u8; 16]>,
            keys: Vec<Vec<u8>>,
            signatures: Vec<Vec<u8>>,
        }
    }
}
/// Default to true for serde boolean fields
fn default_true() -> bool {
    true
}
