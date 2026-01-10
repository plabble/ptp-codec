use binary_codec::CryptoStream;
use cipher::StreamCipher;

/// Stream cipher crypto stream
pub struct StreamCipherCryptoStream {
    plaintext: Vec<u8>,
    ciphers: Vec<Box<dyn StreamCipher>>,
}

impl StreamCipherCryptoStream {
    /// Create new stream cipher stream
    pub fn new(ciphers: Vec<Box<dyn StreamCipher>>) -> Self {
        Self {
            plaintext: Vec::new(),
            ciphers,
        }
    }
}

impl CryptoStream for StreamCipherCryptoStream {
    fn apply_keystream_byte(&mut self, b: u8) -> u8 {
        self.apply_keystream(&[b])[0]
    }

    fn apply_keystream(&mut self, slice: &[u8]) -> &[u8] {
        let offset = self.plaintext.len();
        self.plaintext.extend_from_slice(slice);
        let slice = &mut self.plaintext[offset..];
        for cipher in self.ciphers.iter_mut() {
            cipher.as_mut().apply_keystream(slice);
        }
        slice
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
    fn test_aes_and_chacha_decrypt() {
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
