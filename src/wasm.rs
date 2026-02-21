use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use js_sys::{Function, Uint8Array};
use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

use crate::{
    core::BucketId,
    packets::{
        context::PlabbleConnectionContext,
        request::PlabbleRequestPacket,
        response::PlabbleResponsePacket,
    },
};

// Global callback storage
static GLOBAL_GET_BUCKET_KEY: Mutex<Option<Function>> = Mutex::new(None);
static GLOBAL_GET_PSK: Mutex<Option<Function>> = Mutex::new(None);

// Wrapper functions that call the global callbacks
fn call_global_bucket_key(id: &BucketId) -> Option<[u8; 32]> {
    let guard = GLOBAL_GET_BUCKET_KEY.lock().unwrap();
    if let Some(cb) = guard.as_ref() {
        let bytes = id.to_bytes(None::<&mut SerializerConfig<()>>).unwrap();
        let input = Uint8Array::from(&bytes[..]);
        
        if let Ok(result) = cb.call1(&JsValue::NULL, &input.into()) {
            if let Ok(arr) = result.dyn_into::<Uint8Array>() {
                if arr.length() == 32 {
                    let mut output = [0u8; 32];
                    arr.copy_to(&mut output);
                    return Some(output);
                }
            }
        }
    }
    None
}

fn call_global_psk(id: &[u8; 12]) -> Option<[u8; 64]> {
    let guard = GLOBAL_GET_PSK.lock().unwrap();
    if let Some(cb) = guard.as_ref() {
        let input = Uint8Array::from(&id[..]);
        
        if let Ok(result) = cb.call1(&JsValue::NULL, &input.into()) {
            if let Ok(arr) = result.dyn_into::<Uint8Array>() {
                if arr.length() == 64 {
                    let mut output = [0u8; 64];
                    arr.copy_to(&mut output);
                    return Some(output);
                }
            }
        }
    }
    None
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct ConnectionContext(PlabbleConnectionContext);

#[wasm_bindgen]
impl ConnectionContext {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut context = PlabbleConnectionContext::new();
        context.get_bucket_key = Some(call_global_bucket_key);
        context.get_psk = Some(call_global_psk);
        
        Self(context)
    }
}

#[wasm_bindgen]
pub fn encode_packet(
    input: &str,
    is_request: bool,
    context: &mut ConnectionContext,
) -> Result<Uint8Array, JsValue> {
    let mut config = Some(SerializerConfig::new(Some(context.0.clone())));

    let packet_result = if is_request {
        serde_json::from_str::<PlabbleRequestPacket>(input)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {:?}", e)))?
            .to_bytes(config.as_mut())
    } else {
        serde_json::from_str::<PlabbleResponsePacket>(input)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {:?}", e)))?
            .to_bytes(config.as_mut())
    };

    let res = match packet_result {
        Ok(bytes) => Ok(Uint8Array::from(&bytes[..])),
        Err(e) => Err(JsValue::from_str(&format!("{:?}", e))),
    };

    if res.is_ok() {
        context.0.increment(is_request);
    }

    res
}

#[wasm_bindgen]
pub fn decode_packet(
    input: Uint8Array,
    is_request: bool,
    context: &mut ConnectionContext,
) -> Result<String, JsValue> {
    let bytes = input.to_vec();
    
    let mut config = Some(SerializerConfig::new(Some(context.0.clone())));

    let res = if is_request {
        let packet = PlabbleRequestPacket::from_bytes(&bytes, config.as_mut())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        serde_json::to_string(&packet)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
    } else {
        let packet = PlabbleResponsePacket::from_bytes(&bytes, config.as_mut())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        
        serde_json::to_string(&packet)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))
    };

    if res.is_ok() {
        context.0.increment(is_request);
    }

    res
}

#[wasm_bindgen]
pub fn set_get_bucket_key_callback(cb: Function) {
    *GLOBAL_GET_BUCKET_KEY.lock().unwrap() = Some(cb);
}

#[wasm_bindgen]
pub fn set_get_psk_callback(cb: Function) {
    *GLOBAL_GET_PSK.lock().unwrap() = Some(cb);
}
