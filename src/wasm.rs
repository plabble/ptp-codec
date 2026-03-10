use async_channel::Sender;
use js_sys::{Function, Uint8Array};
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{
    packets::
        request::PlabbleRequestPacket
    ,
    protocol::{PlabbleConnection as InnerPlabbleConnection},
};

#[wasm_bindgen]
pub fn plabble_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[wasm_bindgen]
pub fn setup_logging(level: u8) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(match level {
        1 => log::Level::Error,
        2 => log::Level::Warn,
        3 => log::Level::Info,
        4 => log::Level::Debug,
        _ => log::Level::Trace,
    }).expect("Failed to setup logging");
}

#[wasm_bindgen]
pub struct PlabbleConnection {
    inner: InnerPlabbleConnection,
    rx: Sender<Vec<u8>>,
}

/// Plabble Connection
#[wasm_bindgen]
impl PlabbleConnection {
    /// Create new PlabbleConnection instance
    /// 
    /// - `handle_send`: JS callback to handle outgoing packets (called with Uint8Array)
    /// - `get_bucket_key`: Optional JS callback to get bucket key (called with bucket ID as Uint8Array(16), should return Uint8Array(32))
    /// - `get_psk`: Optional JS callback to get PSK (called with PSK ID as Uint8Array(12), should return Uint8Array(64))
    #[wasm_bindgen(constructor)]
    pub fn new(handle_send: Function, get_bucket_key: Option<Function>, get_psk: Option<Function>) -> Self {
        let (rx, recv) = async_channel::unbounded();
        let (send, tx) = async_channel::unbounded();
        let mut inner = InnerPlabbleConnection::new(send, recv);

        let data = inner.config.data.as_mut().unwrap();

        if let Some(get_bucket_key) = get_bucket_key {
            data.get_bucket_key = Some(Arc::new(move |bucket_id| {
                call_js_byte_array_cb_1(&get_bucket_key, &bucket_id.data)
            }));
        }

        if let Some(get_psk) = get_psk {
            data.get_psk = Some(Arc::new(move |psk_id| {
                call_js_byte_array_cb_1(&get_psk, psk_id)
            }));
        }

        spawn_local(async move {
            while let Ok(res) = tx.recv().await {
                let array = Uint8Array::from(&res[..]);
                let _ = handle_send.call1(&JsValue::NULL, &array.into());
            }
        });

        Self { inner, rx }
    }

    /// Send a packet to the Plabble connection (accepts a JSON string representing PlabbleRequestPacket)
    pub async fn send(&mut self, packet: &str) -> Result<(), JsValue> {
        let request = serde_json::from_str::<PlabbleRequestPacket>(packet)
            .map_err(|e| JsValue::from_str(&format!("Parse error: {:?}", e)))?;

        self.inner
            .send(request)
            .await
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        Ok(())
    }

    /// Internal method to handle received packet bytes (called from JS)
    pub async fn on_recv(&self, data: Vec<u8>) {
        let _ = self.rx.send(data).await;
    }
}

/// Helper function to call JS callbacks and convert result to fixed-size array
fn call_js_byte_array_cb_1<const N: usize>(callback: &Function, input: &[u8]) -> Option<[u8; N]> {
    callback
        .call1(&JsValue::NULL, &Uint8Array::from(input).into())
        .ok()?
        .dyn_into::<Uint8Array>()
        .ok()
        .filter(|arr| arr.length() == N as u32)
        .map(|arr| {
            let mut output = [0u8; N];
            arr.copy_to(&mut output);
            output
        })
}