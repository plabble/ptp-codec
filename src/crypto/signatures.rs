use crate::crypto::algorithm::{CryptoSignature, SigningKey, VerificationKey};

impl SigningKey {
    /// Sign data using this signing key. Returns None if failed, CryptoSignature if succeeded
    pub fn sign(&self, data: &[u8]) -> Option<CryptoSignature> {
        match self {
            SigningKey::Ed25519(key) => {
                use ed25519_dalek::{SigningKey, ed25519::signature::Signer};

                let key = SigningKey::from_bytes(key);
                let signature = key.try_sign(data).ok()?.to_bytes();
                Some(CryptoSignature::Ed25519(signature))
            }
            SigningKey::Ed448(key) => {
                use ed448_goldilocks::{SigningKey, signature::Signer};

                let key = SigningKey::try_from(&key[..]).expect("Invalid Ed448 signing key");
                let signature = key.try_sign(data).ok()?.to_bytes();
                Some(CryptoSignature::Ed448(signature))
            }
            #[cfg(feature = "pqc-lite")]
            SigningKey::Dsa44(key) => {
                use ml_dsa::{MlDsa44, Signature, SigningKey, signature::Signer};

                let key = SigningKey::<MlDsa44>::from_seed(key.into());

                let signature: Signature<MlDsa44> = key.try_sign(data).ok()?;
                Some(CryptoSignature::Dsa44(signature.encode().into()))
            }
            #[cfg(feature = "pqc-lite")]
            SigningKey::Dsa65(key) => {
                use ml_dsa::{MlDsa65, Signature, SigningKey, signature::Signer};

                let key = SigningKey::<MlDsa65>::from_seed(key.into());
                let signature: Signature<MlDsa65> = key.try_sign(data).ok()?;
                Some(CryptoSignature::Dsa65(signature.encode().into()))
            }
            // SigningKey::Falcon(_) => todo!(),
            // SigningKey::SlhDsaSha128s(_) => todo!(),
            _ => None,
        }
    }
}

impl VerificationKey {
    /// Verify signature with data and this key. Returns Some if succeeded, true if valid, false if not valid.
    /// Returns None if failed (algorithm not supported or key invalid)
    pub fn verify(&self, data: &[u8], signature: &CryptoSignature) -> Option<bool> {
        match self {
            VerificationKey::Ed25519(key) => {
                if let CryptoSignature::Ed25519(signature) = signature {
                    use ed25519_dalek::{Verifier, VerifyingKey, ed25519::Signature};

                    let key = VerifyingKey::from_bytes(key).ok()?;
                    let signature = Signature::from_bytes(signature);
                    Some(key.verify(data, &signature).is_ok())
                } else {
                    None
                }
            }
            VerificationKey::Ed448(key) => {
                if let CryptoSignature::Ed448(signature) = signature {
                    use ed448_goldilocks::{Signature, VerifyingKey};

                    let key = VerifyingKey::from_bytes(key).ok()?;
                    let signature = Signature::from_bytes(signature);
                    Some(key.verify_raw(&signature, data).is_ok())
                } else {
                    None
                }
            }
            #[cfg(feature = "pqc-lite")]
            VerificationKey::Dsa44(key) => {
                if let CryptoSignature::Dsa44(signature) = signature {
                    use ml_dsa::{MlDsa44, Signature, VerifyingKey, signature::Verifier};
                    let key = VerifyingKey::<MlDsa44>::decode(key.into());
                    let signature = Signature::<MlDsa44>::decode(signature.into())?;

                    Some(key.verify(data, &signature).is_ok())
                } else {
                    None
                }
            }
            #[cfg(feature = "pqc-lite")]
            VerificationKey::Dsa65(key) => {
                if let CryptoSignature::Dsa65(signature) = signature {
                    use ml_dsa::{MlDsa65, Signature, VerifyingKey, signature::Verifier};
                    let key = VerifyingKey::<MlDsa65>::decode(key.into());
                    let signature = Signature::<MlDsa65>::decode(signature.into())?;

                    Some(key.verify(data, &signature).is_ok())
                } else {
                    None
                }
            }
            // VerificationKey::Falcon(_) => todo!(),
            // VerificationKey::SlhDsaSha128s(_) => todo!(),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::algorithm::{CryptoSignature, SigningKey, VerificationKey};

    #[test]
    fn can_sign_and_verify_ed25519() {
        use ed25519_dalek::SigningKey as SK;

        let data = [0u8; 16];
        let sk = SK::from_bytes(&[0u8; 32]);
        let vk = sk.verifying_key();

        let sig = SigningKey::Ed25519([0u8; 32]);

        let signature = sig.sign(&data).unwrap();
        assert!(matches!(signature, CryptoSignature::Ed25519(_)));

        let ver = VerificationKey::Ed25519(vk.to_bytes());
        let inv = VerificationKey::Ed25519([0u8; 32]);

        assert_eq!(Some(true), ver.verify(&data, &signature)); // Valid
        assert_eq!(Some(false), ver.verify(&[0u8; 15], &signature)); // Invalid data
        assert_eq!(Some(false), inv.verify(&data, &signature)); // Invalid key
    }

    #[test]
    fn can_sign_and_verify_ed448() {
        use ed448_goldilocks::SigningKey as SK;

        let data = [0u8; 16];
        let sk = SK::try_from(&[0u8; 57][..]).expect("Invalid Ed448 signing key");
        let vk = sk.verifying_key();

        let sig = SigningKey::Ed448([0u8; 57]);

        let signature = sig.sign(&data).unwrap();
        assert!(matches!(signature, CryptoSignature::Ed448(_)));

        let ver = VerificationKey::Ed448(vk.to_bytes());
        let inv = VerificationKey::Ed448([0u8; 57]);

        assert_eq!(Some(true), ver.verify(&data, &signature)); // Valid
        assert_eq!(Some(false), ver.verify(&[0u8; 15], &signature)); // Invalid data
        assert_eq!(None, inv.verify(&data, &signature)); // Invalid key
    }

    #[cfg(feature = "pqc-lite")]
    #[test]
    fn can_sign_and_verify_dsa44() {
        use ml_dsa::{KeyGen, KeyPair, MlDsa44};

        let data = [0u8; 16];
        let kp: KeyPair<MlDsa44> = MlDsa44::key_gen(&mut rand::rng());

        let sig = SigningKey::Dsa44(kp.to_seed().into());

        let signature = sig.sign(&data).unwrap();
        assert!(matches!(signature, CryptoSignature::Dsa44(_)));

        let ver = VerificationKey::Dsa44(kp.verifying_key().encode().into());
        let inv = VerificationKey::Dsa44([0u8; 1312]);

        assert_eq!(Some(true), ver.verify(&data, &signature)); // Valid
        assert_eq!(Some(false), ver.verify(&[0u8; 15], &signature)); // Invalid data
        assert_eq!(Some(false), inv.verify(&data, &signature)); // Invalid key
    }

    #[cfg(feature = "pqc-lite")]
    #[test]
    fn can_sign_and_verify_dsa65() {
        use ml_dsa::{KeyGen, KeyPair, MlDsa65};

        let data = [0u8; 16];
        let kp: KeyPair<MlDsa65> = MlDsa65::key_gen(&mut rand::rng());

        let sig = SigningKey::Dsa65(kp.to_seed().into());

        let signature = sig.sign(&data).unwrap();
        assert!(matches!(signature, CryptoSignature::Dsa65(_)));

        let ver = VerificationKey::Dsa65(kp.verifying_key().encode().into());
        let inv = VerificationKey::Dsa65([0u8; 1952]);

        assert_eq!(Some(true), ver.verify(&data, &signature)); // Valid
        assert_eq!(Some(false), ver.verify(&[0u8; 15], &signature)); // Invalid data
        assert_eq!(Some(false), inv.verify(&data, &signature)); // Invalid key
    }
}
