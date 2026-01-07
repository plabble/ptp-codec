use crate::crypto::algorithm::{KeyExhangeRequest, KeyExhangeResponse};
use x25519_dalek::{EphemeralSecret, PublicKey, StaticSecret};

#[derive(Debug)]
pub enum KeyExchangeAlgorithm {
    X25519,
    Kem512,
    Kem768,
}

#[derive(Debug)]
pub struct KeyExchange {
    algorithm: KeyExchangeAlgorithm,
    secret: Option<Vec<u8>>
}

impl KeyExchange {
    pub fn new(algorithm: KeyExchangeAlgorithm) -> Self {
        Self {
            algorithm,
            secret: None
        }
    }

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
            _ => None,
        }
    }

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
            _ => None,
        }
    }

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
