use std::cell::RefCell;

use crate::{packets::{base::settings::CryptoSettings, context::PlabbleConnectionContext}, protocol::PlabbleConnection};


impl PlabbleConnection {
    pub fn new(endpoint: &str) -> Self {
        Self {
            context: RefCell::new(PlabbleConnectionContext::new()),
            crypto_settings: None
        }
    }

    pub fn set_crypto_settings(&mut self, settings: Option<CryptoSettings>) {
        self.crypto_settings = settings;
    }

    // TODO: start session
}