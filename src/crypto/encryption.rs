use aes::Aes256;
use aes_gcm::{Aes256Gcm, aead::Payload};
use binary_codec::CryptoStream;
use chacha20::XChaCha20;
use chacha20poly1305::{XChaCha20Poly1305, aead::Aead};
use cipher::{KeyInit, KeyIvInit, StreamCipher};
use log::debug;

use crate::packets::{base::PlabblePacketBase, context::PlabbleConnectionContext};
type Aes256Ctr64Le = ctr::Ctr64LE<Aes256>;

/// Stream cipher crypto stream
pub struct StreamCipherCryptoStream {
    original: Vec<u8>,
    modified: Vec<u8>,
    ciphers: Vec<Box<dyn StreamCipher>>,
}

impl StreamCipherCryptoStream {
    /// Create new stream cipher stream
    pub fn new(ciphers: Vec<Box<dyn StreamCipher>>) -> Self {
        Self {
            original: Vec::new(),
            modified: Vec::new(),
            ciphers,
        }
    }
}

impl CryptoStream for StreamCipherCryptoStream {
    fn apply_keystream_byte(&mut self, b: u8) -> u8 {
        self.apply_keystream(&[b])[0]
    }

    fn apply_keystream(&mut self, slice: &[u8]) -> &[u8] {
        let offset = self.modified.len();
        self.original.extend_from_slice(slice);
        self.modified.extend_from_slice(slice);
        let slice = &mut self.modified[offset..];
        for cipher in self.ciphers.iter_mut() {
            cipher.as_mut().apply_keystream(slice);
        }
        slice
    }

    fn get_cached(&self, original: bool) -> &[u8] {
        if original {
            &self.original
        } else {
            &self.modified
        }
    }
}

impl PlabbleConnectionContext {
    /// Create a cryptographic stream for encrypting/decrypting a packet using one of the stream ciphers (XChaCha20/Aes256-CTR)
    pub fn create_crypto_stream(
        &self,
        base: Option<&PlabblePacketBase>,
        is_request: bool,
    ) -> Option<Box<dyn CryptoStream>> {
        let settings = self.crypto_settings.unwrap_or_default();
        let mut ciphers: Vec<Box<dyn StreamCipher>> = Vec::new();

        let mut keys = (0..10).map(|i| self.create_key(base, i, is_request));

        // If ChaCha20-encryption is enabled, use XChaCha20 with 32 bytes from the IKM plus 12 bytes from the IKM as nonce
        if settings.encrypt_with_chacha
            && let Some(key) = keys.next()?
        {
            debug!("Using XChaCha cipher, req: {is_request}");
            ciphers.push(Box::new(XChaCha20::new(
                key[..32].into(),
                key[32..56].into(),
            )));
        }

        // If AES-encryption is enabled, use AES256-CTR with 32 bytes from the IKM plus 16 bytes from the IKM as IV
        if settings.encrypt_with_aes
            && let Some(key) = keys.next()?
        {
            debug!("Using Aes256-CTR (64LE) cipher, req: {is_request}");
            ciphers.push(Box::new(Aes256Ctr64Le::new(
                key[..32].into(),
                key[32..48].into(),
            )));
        }

        if ciphers.is_empty() {
            None
        } else {
            Some(Box::new(StreamCipherCryptoStream::new(ciphers)))
        }
    }

    /// Encrypt Plabble packet body with AES256-GCM/XChaCha20-Poly1305
    pub fn encrypt(
        &self,
        base: &PlabblePacketBase,
        is_request: bool,
        data: &[u8],
        aad: &[u8],
    ) -> Option<Vec<u8>> {
        let settings = self.crypto_settings.unwrap_or_default();
        let mut keys = (0..2).map(|i| self.create_key(Some(base), i + 0x77, is_request));

        let mut buff = data.to_vec();

        if settings.encrypt_with_chacha
            && let Some(key) = keys.next()?
        {
            debug!("Encrypting body using XChaCha20-Poly1305, req: {is_request}");
            let cipher = XChaCha20Poly1305::new(key[..32].into());
            buff = cipher
                .encrypt(
                    key[32..56].into(),
                    Payload {
                        msg: &buff[..],
                        aad,
                    },
                )
                .ok()?;
        }

        if settings.encrypt_with_aes
            && let Some(key) = keys.next()?
        {
            debug!("Encrypting body using AES-256-GCM, req: {is_request}");
            let cipher = Aes256Gcm::new(key[..32].into());
            buff = cipher
                .encrypt(
                    key[32..48].into(),
                    Payload {
                        msg: &buff[..],
                        aad,
                    },
                )
                .ok()?;
        }

        Some(buff)
    }

    /// Decrypt Plabble packet body with AES256-GCM/XChaCha20-Poly1305
    pub fn decrypt(
        &self,
        base: &PlabblePacketBase,
        is_request: bool,
        data: &[u8],
        aad: &[u8],
    ) -> Option<Vec<u8>> {
        let settings = self.crypto_settings.unwrap_or_default();
        let mut keys = (0..2).map(|i| self.create_key(Some(base), i + 0x77, is_request));

        let mut buff = data.to_vec();

        // Important: the order of decrypting should be the exact opposite of encrypting!
        // For stream ciphers, this doesn't matter, but for AEAD it does!

        if settings.encrypt_with_aes
            && let Some(key) = keys.next()?
        {
            debug!("Decrypting body using AES-256-GCM, req: {is_request}");
            let cipher = Aes256Gcm::new(key[..32].into());
            buff = cipher
                .decrypt(
                    key[32..48].into(),
                    Payload {
                        msg: &buff[..],
                        aad,
                    },
                )
                .ok()?;
        }

        if settings.encrypt_with_chacha
            && let Some(key) = keys.next()?
        {
            debug!("Decrypting body using XChaCha20-Poly1305, req: {is_request}");
            let cipher = XChaCha20Poly1305::new(key[..32].into());
            buff = cipher
                .decrypt(
                    key[32..56].into(),
                    Payload {
                        msg: &buff[..],
                        aad,
                    },
                )
                .ok()?;
        }

        Some(buff)
    }
}

#[cfg(test)]
mod tests {
    use base64::{Engine, prelude::BASE64_STANDARD};
    use binary_codec::BitStreamReader;
    type Aes128Ctr64Le = ctr::Ctr64LE<aes::Aes128>;

    use chacha20::{
        ChaCha20,
        cipher::{KeyIvInit, StreamCipher},
    };

    use crate::crypto::encryption::StreamCipherCryptoStream;

    #[test]
    fn test_chacha20_encrypt_decrypt() {
        let mut cipher = ChaCha20::new(&[10u8; 32].into(), &[0u8; 12].into());
        let mut ciphertext = b"Hello world!!".clone();
        cipher.apply_keystream(&mut ciphertext);

        let cipher = ChaCha20::new(&[10u8; 32].into(), &[0u8; 12].into());

        let crypto = StreamCipherCryptoStream::new(vec![Box::new(cipher)]);
        let mut reader = BitStreamReader::new(&ciphertext);
        reader.set_crypto(Some(Box::new(crypto)));

        assert_eq!(reader.read_byte().unwrap(), b'H');
        assert_eq!(reader.read_byte().unwrap(), b'e');
        assert_eq!(reader.read_byte().unwrap(), b'l');
        assert_eq!(reader.read_byte().unwrap(), b'l');
        assert_eq!(reader.read_byte().unwrap(), b'o');
        assert_eq!(reader.read_byte().unwrap(), b' ');
        assert_eq!(reader.read_bytes(7).unwrap(), b"world!!");
    }

    #[test]
    fn test_aes_ctr_encrypt_decrypt() {
        let key = b"0000000000000000";
        let mut cipher = Aes128Ctr64Le::new(key.into(), key.into());
        let mut ciphertext = b"Hello world!!".clone();
        cipher.apply_keystream(&mut ciphertext);

        assert_eq!("sTkTB3YLVdCMkX9WuA==", BASE64_STANDARD.encode(&ciphertext));

        let cipher = Aes128Ctr64Le::new(key.into(), key.into());

        let crypto = StreamCipherCryptoStream::new(vec![Box::new(cipher)]);
        let mut reader = BitStreamReader::new(&ciphertext);
        reader.set_crypto(Some(Box::new(crypto)));

        assert_eq!(reader.read_byte().unwrap(), b'H');
        assert_eq!(reader.read_byte().unwrap(), b'e');
        assert_eq!(reader.read_byte().unwrap(), b'l');
        assert_eq!(reader.read_byte().unwrap(), b'l');
        assert_eq!(reader.read_byte().unwrap(), b'o');
        assert_eq!(reader.read_byte().unwrap(), b' ');
        assert_eq!(reader.read_bytes(7).unwrap(), b"world!!");
    }

    #[test]
    fn test_aes_and_chacha_stream_decrypt() {
        // input encrypted with aes128-ctr, then with chacha20
        let input = BASE64_STANDARD.decode("5TFrPHSvv+jBTqDP2g==").unwrap();
        let key1 = b"00000000000000000000000000000000";
        let key2 = b"0000000000000000";
        let key3 = b"000000000000";

        let cipher1 = ChaCha20::new(key1.into(), key3.into());
        let cipher2 = Aes128Ctr64Le::new(key2.into(), key2.into());

        let mut reader = BitStreamReader::new(&input);
        let crypto = StreamCipherCryptoStream::new(vec![Box::new(cipher1), Box::new(cipher2)]);
        reader.set_crypto(Some(Box::new(crypto)));

        assert_eq!(reader.read_bytes(5).unwrap(), b"Hello");
        assert_eq!(reader.read_byte().unwrap(), b' ');
        assert_eq!(reader.read_bytes(7).unwrap(), b"world!!");
    }
}
