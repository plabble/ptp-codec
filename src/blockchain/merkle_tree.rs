use binary_codec::{FromBytes, ToBytes};
use serde::{Deserialize, Serialize};

use crate::crypto::hash_192;

/// Merkle node for a Merkle tree
#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: [u8; 24],
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

/// Merkle tree
#[derive(Debug)]
pub struct MerkleTree {
    pub root: MerkleNode,
    blake3: bool,
}

/// Merkle Proof
#[derive(FromBytes, ToBytes, Serialize, Deserialize, Debug, PartialEq)]
pub struct MerkleProof {
    /// Length of the path (number of sibling hashes)
    #[length_for = "hashes"]
    pub length: u8,

    /// Hashes of sibling nodes
    #[length_by = "hashes"]
    pub hashes: Vec<[u8; 24]>,

    /// Path in the merkle tree
    #[length_by = "hashes"]
    pub path: Vec<bool>,
}

impl MerkleNode {
    pub fn new(hash: [u8; 24]) -> Self {
        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl MerkleTree {
    /// Create new merkle tree from single node (root node)
    pub fn new(root: MerkleNode, blake3: bool) -> Self {
        MerkleTree { root, blake3 }
    }

    /// New Merkle tree from a list of leaf hashes. The leaf hashes should be 24 bytes (192 bits) each.
    /// The hashes MUST be sorted
    pub fn new_from_hashes(blake3: bool, leaf_hashes: Vec<[u8; 24]>) -> Self {
        let nodes = leaf_hashes.into_iter().map(MerkleNode::new).collect();
        MerkleTree::new_tree(blake3, nodes)
    }

    /// Make new Merkle tree from a list of leaf hashes.
    fn new_tree(blake3: bool, mut nodes: Vec<MerkleNode>) -> MerkleTree {
        while nodes.len() > 1 {
            if nodes.len() % 2 != 0 {
                nodes.push(nodes.last().unwrap().clone()); // duplicate last node if odd
            }

            let mut new_level = Vec::new();
            for pair in nodes.chunks(2) {
                let hash = hash_192(blake3, vec![&pair[0].hash, &pair[1].hash]);
                let parent = MerkleNode {
                    hash: hash.into(),
                    left: Some(Box::new(pair[0].clone())),
                    right: Some(Box::new(pair[1].clone())),
                };
                new_level.push(parent);
            }
            nodes = new_level;
        }

        MerkleTree::new(nodes.remove(0), blake3)
    }

    /// Generate a Merkle inclusion proof for a given target hash.
    /// Returns `Some(MerkleProof)` if the hash is found, otherwise `None`.
    pub fn make_proof(&self, target_hash: [u8; 24]) -> Option<MerkleProof> {
        fn recurse(
            node: &MerkleNode,
            target_hash: [u8; 24],
            path: &mut Vec<bool>,
            hashes: &mut Vec<[u8; 24]>,
        ) -> bool {
            if node.is_leaf() && node.hash == target_hash {
                return true;
            }

            if let Some(left) = &node.left {
                if recurse(left, target_hash, path, hashes) {
                    path.push(false); // Left child
                    hashes.push(node.right.as_ref().unwrap().hash);
                    return true;
                }
            }

            if let Some(right) = &node.right {
                if recurse(right, target_hash, path, hashes) {
                    path.push(true); // Right child
                    hashes.push(node.left.as_ref().unwrap().hash);
                    return true;
                }
            }

            false
        }

        let mut path = Vec::new();
        let mut hashes = Vec::new();
        if recurse(&self.root, target_hash, &mut path, &mut hashes) {
            Some(MerkleProof {
                length: hashes.len() as u8,
                hashes,
                path,
            })
        } else {
            None
        }
    }

    /// Verify a Merkle inclusion proof.
    /// Returns `true` if the proof is valid and the hash is part of the tree.
    pub fn verify_proof(&self, target_hash: [u8; 24], proof: &MerkleProof) -> bool {
        let mut computed_hash = target_hash;
        for (i, sibling_hash) in proof.hashes.iter().enumerate() {
            computed_hash = if proof.path[i] {
                hash_192(self.blake3, vec![sibling_hash, &computed_hash])
            } else {
                hash_192(self.blake3, vec![&computed_hash, sibling_hash])
            };
        }
        computed_hash == self.root_hash()
    }

    /// Get the Merkle root hash of the tree.
    pub fn root_hash(&self) -> [u8; 24] {
        self.root.hash
    }
}

#[cfg(test)]
mod tests {
    use binary_codec::{BinaryDeserializer, BinarySerializer, SerializerConfig};

    use super::*;

    fn h(data: &[u8]) -> [u8; 24] {
        hash_192(false, vec![data])
    }

    fn sample_hashes() -> Vec<[u8; 24]> {
        vec![h(b"a"), h(b"b"), h(b"c"), h(b"d")]
    }

    #[test]
    fn merkle_root_is_deterministic() {
        let leaves = sample_hashes();

        let tree1 = MerkleTree::new_from_hashes(false, leaves.clone());
        let tree2 = MerkleTree::new_from_hashes(false, leaves);

        assert_eq!(tree1.root_hash(), tree2.root_hash());
    }

    #[test]
    fn proof_generation_and_verification() {
        let leaves = sample_hashes();
        let tree = MerkleTree::new_from_hashes(false, leaves.clone());
        let second_tree = MerkleTree::new(MerkleNode::new(tree.root_hash()), false);

        for leaf in leaves {
            let proof = tree.make_proof(leaf).unwrap();
            assert!(tree.verify_proof(leaf, &proof));
            assert!(second_tree.verify_proof(leaf, &proof));
        }
    }

    #[test]
    fn proof_fails_for_wrong_leaf() {
        let leaves = sample_hashes();
        let tree = MerkleTree::new_from_hashes(false, leaves.clone());

        let proof = tree.make_proof(leaves[0]).unwrap();

        let fake_leaf = h(b"evil");
        assert!(!tree.verify_proof(fake_leaf, &proof));
    }

    #[test]
    fn proof_fails_if_hash_modified() {
        let leaves = sample_hashes();
        let tree = MerkleTree::new_from_hashes(false, leaves.clone());

        let mut proof = tree.make_proof(leaves[0]).unwrap();

        proof.hashes[0][0] ^= 1;

        assert!(!tree.verify_proof(leaves[0], &proof));
    }

    #[test]
    fn proof_fails_if_path_modified() {
        let leaves = sample_hashes();
        let tree = MerkleTree::new_from_hashes(false, leaves.clone());

        let mut proof = tree.make_proof(leaves[0]).unwrap();

        proof.path[0] = !proof.path[0];

        assert!(!tree.verify_proof(leaves[0], &proof));
    }

    #[test]
    fn proof_for_missing_leaf_returns_none() {
        let leaves = sample_hashes();
        let tree = MerkleTree::new_from_hashes(false, leaves);

        let missing = h(b"not_present");

        assert!(tree.make_proof(missing).is_none());
    }

    #[test]
    fn works_with_odd_number_of_leaves() {
        let leaves = vec![h(b"a"), h(b"b"), h(b"c")];

        let tree = MerkleTree::new_from_hashes(false, leaves.clone());

        for leaf in leaves {
            let proof = tree.make_proof(leaf).unwrap();
            assert!(tree.verify_proof(leaf, &proof));
        }
    }

    #[test]
    fn single_leaf_tree() {
        let leaf = h(b"a");

        let tree = MerkleTree::new_from_hashes(false, vec![leaf]);

        let proof = tree.make_proof(leaf).unwrap();

        assert!(tree.verify_proof(leaf, &proof));
        assert!(proof.hashes.is_empty());
        assert!(proof.path.is_empty());
    }

    #[test]
    fn can_serialize_and_deserialize_proof() {
        let leaves = vec![h(b"a"), h(b"b"), h(b"c"), h(b"d"), h(b"e"), h(b"f")];
        let tree = MerkleTree::new_from_hashes(false, leaves.clone());

        let proof = tree.make_proof(leaves[1]).unwrap();

        let config: Option<&mut SerializerConfig> = None;
        let bytes = proof.to_bytes(config).unwrap();

        print!("{}", hex::encode(&bytes));

        let config: Option<&mut SerializerConfig> = None;
        let deserialized = MerkleProof::from_bytes(&bytes, config).unwrap();

        assert_eq!(proof, deserialized);

        let tree = MerkleTree::new(MerkleNode::new(tree.root_hash()), false);
        assert!(tree.verify_proof(leaves[1], &proof));
    }
}
