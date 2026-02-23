use std::cell::RefCell;

use crate::packets::{base::settings::CryptoSettings, context::PlabbleConnectionContext};

#[cfg(feature = "server")]
pub mod server;

#[cfg(feature = "client")]
pub mod client;

pub struct PlabbleConnection {
    context: RefCell<PlabbleConnectionContext>,
    crypto_settings: Option<CryptoSettings>
}