use std::sync::Arc;

use async_channel::Sender;
use js_sys::{Function, Uint8Array};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::{
    providers::KeyProvider,
    protocol::{PlabbleConnection as InnerPlabbleConnection, deserialize_input, serialize_output},
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
    })
    .expect("Failed to setup logging");
}

#[wasm_bindgen]
pub struct PlabbleConnection {
    inner: InnerPlabbleConnection,
    rx: Sender<Vec<u8>>,
}

#[wasm_bindgen]
pub struct SessionKeyProvider {
    get_bucket_key: Function,
    get_psk: Function,
    store_psk: Function,
}

#[wasm_bindgen]
impl SessionKeyProvider {
    #[wasm_bindgen(constructor)]
    pub fn new(
        get_bucket_key: Function,
        get_psk: Function,
        store_psk: Function,
    ) -> SessionKeyProvider {
        Self {
            get_bucket_key,
            get_psk,
            store_psk,
        }
    }
}

impl KeyProvider for SessionKeyProvider {
    fn get_bucket_key(&self, bucket_id: &[u8; 16]) -> Option<[u8; 32]> {
        call_js_byte_array_cb_1(&self.get_bucket_key, bucket_id)
    }

    fn get_psk(&self, psk_id: &[u8; 12]) -> Option<[u8; 64]> {
        call_js_byte_array_cb_1(&self.get_psk, psk_id)
    }

    fn store_psk(&self, psk_id: [u8; 12], psk: [u8; 64], expiration: Option<u32>) {
        let psk_id_array = Uint8Array::from(&psk_id[..]);
        let psk_array = Uint8Array::from(&psk[..]);
        // TODO: error logging?
        let _ = self.store_psk.call3(
            &JsValue::NULL,
            &psk_id_array.into(),
            &psk_array.into(),
            &JsValue::from_f64(expiration.map(|v| v as f64).unwrap_or(f64::NAN)),
        );
    }
}

/// Plabble Connection
#[wasm_bindgen]
impl PlabbleConnection {
    /// Create new PlabbleConnection instance
    ///
    /// - `handle_send`: JS callback to handle outgoing packets (called with Uint8Array)
    #[wasm_bindgen(constructor)]
    pub fn new(handle_send: Function) -> Self {
        let (rx, recv) = async_channel::unbounded();
        let (send, tx) = async_channel::unbounded();
        let inner = InnerPlabbleConnection::new(send, recv);

        spawn_local(async move {
            while let Ok(res) = tx.recv().await {
                let array = Uint8Array::from(&res[..]);
                let _ = handle_send.call1(&JsValue::NULL, &array.into());
            }
        });

        Self { inner, rx }
    }

    /// Set key providers/JS callbacks
    pub fn set_key_provider(&mut self, provider: SessionKeyProvider) {
        let data = self.inner.config.data.as_mut().unwrap();

        let provider: Arc<dyn KeyProvider> = Arc::from(provider);
        data.key_provider = Some(provider);
    }

    /// Send a packet to the Plabble connection (accepts a JSON/TOML string representing PlabbleRequestPacket)
    pub async fn send_request(&mut self, packet: &str) -> Result<(), JsValue> {
        let request = deserialize_input(packet)
            .map_err(|e| JsValue::from_str(&format!("Deserialize error: {:?}", e)))?;

        self.inner
            .send_request(request)
            .await
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
        Ok(())
    }

    /// Send a packet and wait for the response (accepts a JSON/TOML string representing PlabbleRequestPacket, returns a JSON/TOML string representing PlabbleResponsePacket)
    pub async fn send_and_recv(&mut self, packet: &str) -> Result<String, JsValue> {
        let request = deserialize_input(packet)
            .map_err(|e| JsValue::from_str(&format!("Deserialize error: {:?}", e)))?;

        let response = self
            .inner
            .send_and_recv(request)
            .await
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        serialize_output(&response)
            .map_err(|e| JsValue::from_str(&format!("Serialize error: {:?}", e)))
    }

    /// Wait for the next incoming response packet and return it as a JSON/TOML string representing PlabbleResponsePacket
    pub async fn recv_response(&mut self) -> Result<String, JsValue> {
        let response = self
            .inner
            .recv_response()
            .await
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        serialize_output(&response)
            .map_err(|e| JsValue::from_str(&format!("Serialize error: {:?}", e)))
    }

    // TODO: same options as ffi.rs has

    /// Internal method to handle received packet bytes (called from JS)
    pub async fn handle_incoming(&self, data: Vec<u8>) {
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
