use std::{
    ffi::{CStr, CString},
    fmt::Debug,
    os::raw::c_char,
    slice,
    sync::Mutex,
};

use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};
use serde::Serialize;

use crate::{
    core::BucketId,
    packets::{
        context::PlabbleConnectionContext, request::PlabbleRequestPacket,
        response::PlabbleResponsePacket,
    },
};

pub type LookupBytesCallback = unsafe extern "C" fn(*const u8, *mut u8) -> bool;

// Global storage/settings
static GLOBAL_GET_BUCKET_KEY: Mutex<Option<LookupBytesCallback>> = Mutex::new(None);
static GLOBAL_GET_PSK: Mutex<Option<LookupBytesCallback>> = Mutex::new(None);

// Wrapper functions that call the global callbacks
fn call_global_bucket_key(id: &BucketId) -> Option<[u8; 32]> {
    let guard = GLOBAL_GET_BUCKET_KEY.lock().unwrap();
    if let Some(cb) = *guard {
        let bytes = id.to_bytes(None::<&mut SerializerConfig<()>>).unwrap();
        let mut output = [0u8; 32];
        let res = unsafe { cb(bytes.as_ptr(), output.as_mut_ptr()) };
        if res { Some(output) } else { None }
    } else {
        None
    }
}

fn call_global_psk(id: &[u8; 12]) -> Option<[u8; 64]> {
    let guard = GLOBAL_GET_PSK.lock().unwrap();
    if let Some(cb) = *guard {
        let mut output = [0u8; 64];
        let res = unsafe { cb(id.as_ptr(), output.as_mut_ptr()) };
        if res { Some(output) } else { None }
    } else {
        None
    }
}

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

    pub fn error<T: Serialize + Debug>(data: T) -> Self {
        let mut bytes = format!("{:?}", data).into_bytes();
        let len = bytes.len();
        let buff = bytes.as_mut_ptr();
        std::mem::forget(bytes);
        Self {
            status: FfiStatus::Error,
            data: FfiBytes { buff, len },
        }
    }

    pub fn new(mut data: Vec<u8>) -> Self {
        let buff = data.as_mut_ptr();
        let len = data.len();

        std::mem::forget(data);

        FfiBytesOutput {
            status: FfiStatus::Ok,
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

    pub fn error<T: Serialize + Debug>(data: T) -> Self {
        Self {
            status: FfiStatus::Error,
            data: CString::new(format!("{:?}", &data)).unwrap().into_raw(),
        }
    }

    pub fn new(s: String) -> Self {
        Self {
            status: FfiStatus::Ok,
            data: CString::new(s).unwrap().into_raw(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn version() -> *const c_char {
    CString::new(env!("CARGO_PKG_VERSION")).unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn encode_packet(
    input: *const c_char,
    is_request: bool,
    context: *mut PlabbleConnectionContext,
) -> FfiBytesOutput {
    if input.is_null() {
        return FfiBytesOutput::fail(FfiStatus::NullPointer);
    }

    let context = if context.is_null() {
        None
    } else {
        Some(unsafe { Box::from_raw(context) })
    };

    let mut config = context
        .as_ref()
        .map(|ctx| SerializerConfig::new(Some(*ctx.clone())));

    let result = unsafe {
        let cstr = CStr::from_ptr(input);
        match cstr.to_str() {
            Ok(s) => {
                let packet_result = if is_request {
                    #[cfg(feature = "with-json")]
                    {
                        serde_json::from_str::<PlabbleRequestPacket>(s)
                            .map(|p| p.to_bytes(config.as_mut()))
                    }

                    #[cfg(all(feature = "with-toml", not(feature = "with-json")))]
                    toml::from_str::<PlabbleRequestPacket>(s).map(|p| p.to_bytes(config.as_mut()))
                } else {
                    #[cfg(feature = "with-json")]
                    {
                        serde_json::from_str::<PlabbleResponsePacket>(s)
                            .map(|p| p.to_bytes(config.as_mut()))
                    }

                    #[cfg(all(feature = "with-toml", not(feature = "with-json")))]
                    toml::from_str::<PlabbleResponsePacket>(s).map(|p| p.to_bytes(config.as_mut()))
                };

                match packet_result {
                    Ok(serialization_result) => match serialization_result {
                        Ok(bytes) => FfiBytesOutput::new(bytes),
                        Err(e) => FfiBytesOutput::error(e),
                    },
                    Err(_) => FfiBytesOutput::fail(FfiStatus::InputParsingFailed),
                }
            }
            Err(_) => FfiBytesOutput::fail(FfiStatus::InvalidInput),
        }
    };

    // Increment counter on the owned Box and return ownership
    if let Some(mut ctx) = context {
        if result.is_ok() {
            ctx.increment(is_request);
        }
        std::mem::forget(ctx);
    }

    result
}

#[unsafe(no_mangle)]
pub extern "C" fn decode_packet(
    input: FfiBytes,
    is_request: bool,
    context: *mut PlabbleConnectionContext,
) -> FfiStringOutput {
    if input.buff.is_null() || input.len == 0 {
        return FfiStringOutput::fail(FfiStatus::NullPointer);
    }

    let context = if context.is_null() {
        None
    } else {
        Some(unsafe { Box::from_raw(context) })
    };

    let mut config = context
        .as_ref()
        .map(|ctx| SerializerConfig::new(Some(*ctx.clone())));

    let bytes = unsafe { slice::from_raw_parts(input.buff, input.len) };
    let packet_result = if is_request {
        #[cfg(feature = "with-json")]
        {
            PlabbleRequestPacket::from_bytes(bytes, config.as_mut()).map(|p| serde_json::to_string(&p))
        }

        #[cfg(all(feature = "with-toml", not(feature = "with-json")))]
        PlabbleRequestPacket::from_bytes(bytes, config.as_mut()).map(|p| toml::to_string(&p))
    } else {
        #[cfg(feature = "with-json")]
        {
            PlabbleResponsePacket::from_bytes(bytes, config.as_mut()).map(|p| serde_json::to_string(&p))
        }
        
        #[cfg(all(feature = "with-toml", not(feature = "with-json")))]
        PlabbleResponsePacket::from_bytes(bytes, config.as_mut()).map(|p| toml::to_string(&p))
    };

    let result = match packet_result {
        Ok(serialization_result) => match serialization_result {
            Ok(str) => FfiStringOutput::new(str),
            Err(_) => FfiStringOutput::fail(FfiStatus::InputParsingFailed),
        },
        Err(e) => FfiStringOutput::error(e),
    };

    // Return ownership back to the same pointer address
    if let Some(mut ctx) = context {
        if result.is_ok() {
            ctx.increment(is_request);
        }
        std::mem::forget(ctx);
    }

    result
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

#[unsafe(no_mangle)]
pub extern "C" fn new_connection_context() -> *mut PlabbleConnectionContext {
    let mut context = PlabbleConnectionContext::new();
    context.get_bucket_key = Some(call_global_bucket_key);
    context.get_psk = Some(call_global_psk);
    Box::into_raw(Box::new(context))
}

#[unsafe(no_mangle)]
pub extern "C" fn free_connection_context(ctx: *mut PlabbleConnectionContext) {
    if ctx.is_null() {
        return;
    }
    unsafe {
        let _ = Box::from_raw(ctx);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn set_get_bucket_key_callback(cb: LookupBytesCallback) {
    *GLOBAL_GET_BUCKET_KEY.lock().unwrap() = Some(cb);
}

#[unsafe(no_mangle)]
pub extern "C" fn set_get_psk_callback(cb: LookupBytesCallback) {
    *GLOBAL_GET_PSK.lock().unwrap() = Some(cb);
}
