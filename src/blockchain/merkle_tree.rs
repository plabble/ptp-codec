use crate::crypto::hash_192;

#[derive(Debug, Clone)]
pub struct MerkleNode {
    pub hash: [u8; 24],
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>,
}

pub struct MerkleTree {
    pub root: MerkleNode,
    blake3: bool,
}

impl MerkleNode {
    pub fn new(hash: [u8; 24]) -> Self {
        MerkleNode {
            hash,
            left: None,
            right: None,
        }
    }
}

impl MerkleTree {
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

    // TODO: make proof, verify proof

    /// Get the Merkle root hash of the tree.
    pub fn root_hash(&self) -> [u8; 24] {
        self.root.hash
    }
}
