//! Key exchange helpers
//!
//! This module provides a small, ergonomic wrapper around supported
//! key-exchange algorithms used by the Plabble protocol. It exposes a
//! `KeyExchange` type that can:
//!
//! - generate a key-exchange request (public key or encapsulation),
//! - process an incoming request and produce a response plus the
//!   responder's shared secret, and
//! - process a response from a peer to derive the initiator's shared
//!   secret.
//!
//! Implementations include X25519 (always available) and optional
//! post-quantum KEMs gated behind `pqc_lite`.

use crate::crypto::algorithm::{KeyExhangeRequest, KeyExhangeResponse};
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

/// Supported key-exchange algorithms.
///
/// - `X25519` is the standard Diffie-Hellman over Curve25519.
/// - `Kem512` / `Kem768` are optional post-quantum KEM algorithms
///   provided when the `pqc_lite` feature is enabled.
pub enum KeyExchangeAlgorithm {
    X25519,
    Kem512,
    Kem768,
}

/// A small stateful helper to run a key exchange for a chosen algorithm.
///
/// The struct stores the selected algorithm and, for initiators, the
/// secret material generated when creating a request. The secret is kept
/// as raw bytes so it can be used by the `process_response` method to
/// compute the final shared secret.
pub struct KeyExchange {
    algorithm: KeyExchangeAlgorithm,
    secret: Option<Vec<u8>>,
}

impl KeyExchange {
    pub fn new(algorithm: KeyExchangeAlgorithm) -> Self {
        Self {
            algorithm,
            secret: None,
        }
    }

    /// Generate a key-exchange request for the configured algorithm.
    ///
    /// For X25519 this generates a fresh private key (stored in
    /// `self.secret`) and returns the public key bytes to send to the
    /// remote peer. For KEMs (when enabled) the encapsulation key is
    /// returned and the decapsulation key is stored for later use.
    pub fn make_request(&mut self) -> Option<KeyExhangeRequest> {
        match self.algorithm {
            KeyExchangeAlgorithm::X25519 => {
                use x25519_dalek::PublicKey;

                let secret = StaticSecret::random_from_rng(&mut rand_core_064::OsRng);
                let public = PublicKey::from(&secret);
                self.secret = Some(secret.as_bytes().to_vec());

                Some(KeyExhangeRequest::X25519(public.to_bytes()))
            }
            #[cfg(feature = "pqc_lite")]
            KeyExchangeAlgorithm::Kem512 => {
                use ml_kem::{
                    EncodedSizeUser, KemCore, MlKem512, MlKem512Params,
                    kem::{DecapsulationKey, EncapsulationKey},
                };

                let (dc, ec): (
                    DecapsulationKey<MlKem512Params>,
                    EncapsulationKey<MlKem512Params>,
                ) = MlKem512::generate(&mut rand_core_064::OsRng);

                self.secret = Some(dc.as_bytes().to_vec());

                Some(KeyExhangeRequest::Kem512(ec.as_bytes().into()))
            }
            #[cfg(feature = "pqc_lite")]
            KeyExchangeAlgorithm::Kem768 => {
                use ml_kem::{
                    EncodedSizeUser, KemCore, MlKem768, MlKem768Params,
                    kem::{DecapsulationKey, EncapsulationKey},
                };

                let (dc, ec): (
                    DecapsulationKey<MlKem768Params>,
                    EncapsulationKey<MlKem768Params>,
                ) = MlKem768::generate(&mut rand_core_064::OsRng);

                self.secret = Some(dc.as_bytes().to_vec());

                Some(KeyExhangeRequest::Kem768(ec.as_bytes().into()))
            }
            #[cfg(not(feature = "pqc_lite"))]
            _ => None,
        }
    }

    /// Process an incoming request as a responder.
    ///
    /// Returns a tuple of (shared_secret_bytes, response) where
    /// `shared_secret_bytes` is the secret computed by the
    /// responder and `response` is the value that must be sent back
    /// to the original requester.
    pub fn process_request(
        &mut self,
        req: &KeyExhangeRequest,
    ) -> Option<([u8; 32], KeyExhangeResponse)> {
        match self.algorithm {
            KeyExchangeAlgorithm::X25519 => {
                if let KeyExhangeRequest::X25519(other_pub) = req {
                    let secret = EphemeralSecret::random_from_rng(&mut rand_core_064::OsRng);
                    let public = PublicKey::from(&secret);
                    let other_pub = PublicKey::from(*other_pub);

                    let ss = secret.diffie_hellman(&other_pub);
                    Some((ss.to_bytes(), KeyExhangeResponse::X25519(public.to_bytes())))
                } else {
                    None
                }
            }
            #[cfg(feature = "pqc_lite")]
            KeyExchangeAlgorithm::Kem512 => {
                if let KeyExhangeRequest::Kem512(encap_key) = req {
                    use ml_kem::{
                        Ciphertext, EncodedSizeUser, MlKem512, MlKem512Params, SharedKey,
                        kem::{Encapsulate, EncapsulationKey},
                    };
                    let encapsulation_key =
                        EncapsulationKey::<MlKem512Params>::from_bytes(encap_key.into());

                    let (es, ss): (Ciphertext<MlKem512>, SharedKey<MlKem512>) = encapsulation_key
                        .encapsulate(&mut rand_core_064::OsRng)
                        .ok()?;

                    Some((ss.into(), KeyExhangeResponse::Kem512(es.into())))
                } else {
                    None
                }
            }
            #[cfg(feature = "pqc_lite")]
            KeyExchangeAlgorithm::Kem768 => {
                if let KeyExhangeRequest::Kem768(encap_key) = req {
                    use ml_kem::{
                        Ciphertext, EncodedSizeUser, MlKem768, MlKem768Params, SharedKey,
                        kem::{Encapsulate, EncapsulationKey},
                    };
                    let encapsulation_key =
                        EncapsulationKey::<MlKem768Params>::from_bytes(encap_key.into());

                    let (es, ss): (Ciphertext<MlKem768>, SharedKey<MlKem768>) = encapsulation_key
                        .encapsulate(&mut rand_core_064::OsRng)
                        .ok()?;

                    Some((ss.into(), KeyExhangeResponse::Kem768(es.into())))
                } else {
                    None
                }
            }
            #[cfg(not(feature = "pqc_lite"))]
            _ => None,
        }
    }

    /// Process a response received from a peer (initiator role).
    ///
    /// This consumes the previously stored secret (generated by
    /// `make_request`) and returns the final shared secret bytes if
    /// the response is compatible with the configured algorithm.
    pub fn process_response(&self, res: &KeyExhangeResponse) -> Option<[u8; 32]> {
        match self.algorithm {
            KeyExchangeAlgorithm::X25519 => {
                if let KeyExhangeResponse::X25519(other_pub) = res {
                    let secret: &[u8; 32] = self.secret.as_ref().unwrap()[..].try_into().unwrap();
                    let secret = StaticSecret::from(*secret);
                    let other_pub = PublicKey::from(*other_pub);

                    let ss = secret.diffie_hellman(&other_pub);
                    Some(ss.to_bytes())
                } else {
                    None
                }
            }
            #[cfg(feature = "pqc_lite")]
            KeyExchangeAlgorithm::Kem512 => {
                if let KeyExhangeResponse::Kem512(ek) = res {
                    use ml_kem::{
                        Ciphertext, EncodedSizeUser, MlKem512, MlKem512Params, SharedKey,
                        kem::{Decapsulate, DecapsulationKey},
                    };

                    let ek: Ciphertext<MlKem512> = (*ek).into();
                    let secret: &[u8; 1632] = self.secret.as_ref().unwrap()[..].try_into().unwrap();

                    let dc = DecapsulationKey::<MlKem512Params>::from_bytes(secret.into());
                    let ss: SharedKey<MlKem512> = dc.decapsulate(&ek).ok()?;

                    Some(ss.into())
                } else {
                    None
                }
            }
            #[cfg(feature = "pqc_lite")]
            KeyExchangeAlgorithm::Kem768 => {
                if let KeyExhangeResponse::Kem768(ek) = res {
                    use ml_kem::{
                        Ciphertext, EncodedSizeUser, MlKem768, MlKem768Params, SharedKey,
                        kem::{Decapsulate, DecapsulationKey},
                    };

                    let ek: Ciphertext<MlKem768> = (*ek).into();
                    let secret: &[u8; 2400] = self.secret.as_ref().unwrap()[..].try_into().unwrap();

                    let dc = DecapsulationKey::<MlKem768Params>::from_bytes(secret.into());
                    let ss: SharedKey<MlKem768> = dc.decapsulate(&ek).ok()?;

                    Some(ss.into())
                } else {
                    None
                }
            }
            #[cfg(not(feature = "pqc_lite"))]
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::key_exchange::{KeyExchange, KeyExchangeAlgorithm};

    #[test]
    fn can_create_a_shared_secret_with_x25519() {
        let mut alice = KeyExchange::new(KeyExchangeAlgorithm::X25519);
        let mut bob = KeyExchange::new(KeyExchangeAlgorithm::X25519);

        let req = alice.make_request().unwrap();
        let (ss_bob, res) = bob.process_request(&req).unwrap();
        let ss_alice = alice.process_response(&res).unwrap();
        assert_eq!(ss_alice, ss_bob);
    }

    #[test]
    fn can_create_a_shared_secret_with_ml_kem_512() {
        let mut alice = KeyExchange::new(KeyExchangeAlgorithm::Kem512);
        let mut bob = KeyExchange::new(KeyExchangeAlgorithm::Kem512);

        let req = alice.make_request().unwrap();
        let (ss_bob, res) = bob.process_request(&req).unwrap();
        let ss_alice = alice.process_response(&res).unwrap();
        assert_eq!(ss_alice, ss_bob);
    }

    #[test]
    fn can_create_a_shared_secret_with_ml_kem_768() {
        let mut alice = KeyExchange::new(KeyExchangeAlgorithm::Kem768);
        let mut bob = KeyExchange::new(KeyExchangeAlgorithm::Kem768);

        let req = alice.make_request().unwrap();
        let (ss_bob, res) = bob.process_request(&req).unwrap();
        let ss_alice = alice.process_response(&res).unwrap();
        assert_eq!(ss_alice, ss_bob);
    }

    #[test]
    fn cannot_create_a_shared_secret_with_incompatible_algorithms() {
        let mut alice = KeyExchange::new(KeyExchangeAlgorithm::X25519);
        let mut bob = KeyExchange::new(KeyExchangeAlgorithm::Kem768);

        let req = alice.make_request().unwrap();
        let res = bob.process_request(&req);
        assert_eq!(res, None);
    }
}
