use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};
use serde_with::hex::Hex;
use serde_with::formats::Lowercase;
use serde_with::serde_as;

/// Block proof, which is a cryptographic proof that a block is valid and can be added to the blockchain
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum BlockProof {
    /// Proof of work (the parameters that are the answer to solve the target)
    #[toggled_by = "proof_of_work"]
    ProofOfWork {
        /// Nonce value used in the proof of work algorithm
        #[dyn_int]
        nonce: u64,

        /// Mining target
        target: Difficulty,
    },
}

/// Mining target
#[serde_as]
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Difficulty {
    /// Exponent, which is a single byte value that defines the difficulty of the mining target
    exponent: u8,

    /// Coefficient, which is a 3-byte value that, together with the exponent, defines the mining target
    #[serde_as(as = "Hex<Lowercase>")]
    coefficient: [u8; 3],
}