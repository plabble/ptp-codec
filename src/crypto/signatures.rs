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
            },
            #[cfg(feature = "pqc_lite")]
            SigningKey::Dsa44(key) => {
                use ml_dsa::{MlDsa44, Signature, SigningKey, signature::Signer};

                let key = SigningKey::<MlDsa44>::decode(key.into());
                
                let signature: Signature<MlDsa44> = key.try_sign(data).ok()?;
                Some(CryptoSignature::Dsa44(signature.encode().into()))
            },
            #[cfg(feature = "pqc_lite")]
            SigningKey::Dsa65(key) => {
                use ml_dsa::{MlDsa65, Signature, SigningKey, signature::Signer};

                let key = SigningKey::<MlDsa65>::decode(key.into());
                let signature: Signature<MlDsa65> = key.try_sign(data).ok()?;
                Some(CryptoSignature::Dsa65(signature.encode().into()))
            },
            // SigningKey::Falcon(_) => todo!(),
            // SigningKey::SlhDsaSha128s(_) => todo!(),
            _ => None
        }
    }
}

impl VerificationKey {
    /// Verify signature with data and this key. Returns Some if succeeded, true if valid, false if not valid.
    /// Returns None if failed (algorithm not supported or key invalid)
    pub fn verify(&self, data: &[u8], signature: CryptoSignature) -> Option<bool> {
        match self {
            VerificationKey::Ed25519(key) => {
                if let CryptoSignature::Ed25519(ref signature) = signature {
                    use ed25519_dalek::{VerifyingKey, Verifier, ed25519::Signature};

                    let key = VerifyingKey::from_bytes(key).expect("a valid ed25519 key");
                    let signature = Signature::from_bytes(signature);
                    Some(key.verify(data, &signature).is_ok())
                } else {
                    None
                }
            },
            #[cfg(feature = "pqc_lite")]
            VerificationKey::Dsa44(key) => {
                if let CryptoSignature::Dsa44(ref signature) = signature {
                    use ml_dsa::{MlDsa44, Signature, VerifyingKey, signature::Verifier};
                    let key = VerifyingKey::<MlDsa44>::decode(key.into());
                    let signature = Signature::<MlDsa44>::decode(signature.into())?;

                    Some(key.verify(data, &signature).is_ok())
                } else {
                    None
                }
            },
            #[cfg(feature = "pqc_lite")]
            VerificationKey::Dsa65(key) => {
                if let CryptoSignature::Dsa65(ref signature) = signature {
                    use ml_dsa::{MlDsa65, Signature, VerifyingKey, signature::Verifier};
                    let key = VerifyingKey::<MlDsa65>::decode(key.into());
                    let signature = Signature::<MlDsa65>::decode(signature.into())?;

                    Some(key.verify(data, &signature).is_ok())
                } else {
                    None
                }
            },
            // VerificationKey::Falcon(_) => todo!(),
            // VerificationKey::SlhDsaSha128s(_) => todo!(),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    // TODO
}