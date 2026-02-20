use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

use binary_codec::{BinaryDeserializer, BinarySerializer};
use serde::Serialize;

use crate::packets::{request::PlabbleRequestPacket, response::PlabbleResponsePacket};

#[repr(C)]
pub enum FfiStatus {
    Ok = 0,
    NullPointer = 1,
    InvalidInput = 2,
    InputParsingFailed = 3,
    Error = 255,
}

#[repr(C)]
pub struct FfiBytes {
    pub buff: *mut u8,
    pub len: usize,
}

#[repr(C)]
pub struct FfiBytesOutput {
    pub status: FfiStatus,
    pub data: FfiBytes,
}

#[repr(C)]
pub struct FfiStringOutput {
    pub status: FfiStatus,
    pub data: *mut c_char,
}

impl FfiBytesOutput {
    pub fn fail(status: FfiStatus) -> Self {
        Self {
            status,
            data: FfiBytes {
                buff: std::ptr::null_mut(),
                len: 0,
            },
        }
    }

    pub fn error<T: Serialize>(data: T) -> Self {
        let mut bytes = toml::to_string(&data).unwrap();
        let len = bytes.len();
        let buff = bytes.as_mut_ptr();
        std::mem::forget(bytes);
        Self {
            status: FfiStatus::Error,
            data: FfiBytes { buff, len },
        }
    }
}

impl FfiStringOutput {
    pub fn fail(status: FfiStatus) -> Self {
        Self {
            status,
            data: CString::new("").unwrap().into_raw(),
        }
    }

    pub fn error<T: Serialize>(data: T) -> Self {
        let s = toml::to_string(&data).unwrap();
        Self {
            status: FfiStatus::Error,
            data: CString::new(s).unwrap().into_raw(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn version() -> *const c_char {
    CString::new(env!("CARGO_PKG_VERSION")).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn encode_packet(input: *const c_char, is_request: bool) -> FfiBytesOutput {
    if input.is_null() {
        return FfiBytesOutput::fail(FfiStatus::NullPointer);
    }

    unsafe {
        let cstr = CStr::from_ptr(input);
        match cstr.to_str() {
            Ok(s) => {
                let packet_result = if is_request {
                    toml::from_str::<PlabbleRequestPacket>(s).map(|p| p.to_bytes(None))
                } else {
                    toml::from_str::<PlabbleResponsePacket>(s).map(|p| p.to_bytes(None))
                };

                match packet_result {
                    Ok(serialization_result) => match serialization_result {
                        Ok(mut bytes) => {
                            let buff = bytes.as_mut_ptr();
                            let len = bytes.len();

                            std::mem::forget(bytes);

                            FfiBytesOutput {
                                status: FfiStatus::Ok,
                                data: FfiBytes { buff, len },
                            }
                        }
                        Err(e) => FfiBytesOutput::error(e),
                    },
                    Err(_) => FfiBytesOutput::fail(FfiStatus::InputParsingFailed),
                }
            }
            Err(_) => FfiBytesOutput::fail(FfiStatus::InvalidInput),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn decode_packet(input: FfiBytes, is_request: bool) -> FfiStringOutput {
    if input.buff.is_null() || input.len == 0 {
        return FfiStringOutput::fail(FfiStatus::NullPointer);
    }

    unsafe {
        let bytes = std::slice::from_raw_parts(input.buff, input.len);
        let packet_result = if is_request {
            PlabbleRequestPacket::from_bytes(bytes, None).map(|p| toml::to_string(&p))
        } else {
            PlabbleResponsePacket::from_bytes(bytes, None).map(|p| toml::to_string(&p))
        };

        match packet_result {
            Ok(serialization_result) => match serialization_result {
                Ok(str) => {
                    FfiStringOutput {
                        status: FfiStatus::Ok,
                        data: CString::new(str).unwrap().into_raw(),
                    }
                }
                Err(_) => FfiStringOutput::fail(FfiStatus::InputParsingFailed),
            },
            Err(e) => FfiStringOutput::error(e),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_bytes(data: FfiBytes) {
    if data.buff.is_null() || data.len == 0 {
        return;
    }

    unsafe {
        // Reconstruct the Vec and drop it to free the memory.
        let _ = Vec::from_raw_parts(data.buff, data.len, data.len);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        // Reconstruct the CString and drop it to free the memory.
        let _ = CString::from_raw(s);
    }
}
