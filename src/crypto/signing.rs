use crate::crypto::algorithm::{CryptoSignature, SigningKey};

impl SigningKey {
    pub fn sign(&self, data: &[u8]) -> Option<CryptoSignature> {
        match self {
            SigningKey::Ed25519(key) => {
                use ed25519_dalek::{SigningKey, ed25519::signature::Signer};

                let key = SigningKey::from_bytes(key);
                let signature = key.try_sign(data).ok()?.to_bytes();
                Some(CryptoSignature::Ed25519(signature))
            },
            #[cfg(feature = "pqc_lite")]
            SigningKey::Dsa44(key) => {
                use ml_dsa::{MlDsa44, SigningKey, signature::Signer};

                let key = SigningKey::<MlDsa44>::decode(key.into());
                let signature = key.try_sign(data).ok()?.encode();
                Some(CryptoSignature::Dsa44(signature.into()))
            },
            #[cfg(feature = "pqc_lite")]
            SigningKey::Dsa65(key) => {
                use ml_dsa::{MlDsa65, SigningKey, signature::Signer};

                let key = SigningKey::<MlDsa65>::decode(key.into());
                let signature = key.try_sign(data).ok()?.encode();
                Some(CryptoSignature::Dsa65(signature.into()))
            },
            SigningKey::Falcon(_) => todo!(),
            SigningKey::SlhDsaSha128s(_) => todo!(),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::algorithm::SigningKey;

    #[test]
    fn try_sign_dsa44() {
        let key = SigningKey::Dsa44([0u8; 2560]);
        let sig = key.sign(&vec![1,2,3,4][..]);

        println!("{:?}", sig);
    }
}