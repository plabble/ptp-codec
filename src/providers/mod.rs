#[cfg(feature = "blockchain")]
use crate::blockchain::transaction::TransactionLock;

use crate::protocol::error::PlabbleProtocolError;

// Key/storage provider for Plabble Connection
pub trait KeyProvider: Send + Sync {
    /// Given a bucket ID serialized as bytes, return the 32-byte bucket key, or None.
    fn get_bucket_key(&self, bucket_id: &[u8; 16]) -> Option<[u8; 32]>;

    /// Given a 12-byte PSK ID, return the 64-byte pre-shared key, or None.
    fn get_psk(&self, psk_id: &[u8; 12]) -> Option<[u8; 64]>;

    /// Store a pre-shared key with the given PSK ID and optional expiration time (as a UNIX timestamp).
    fn store_psk(&self, psk_id: [u8; 12], psk: [u8; 64], expiration: Option<u32>);
}

/// Plabble bucket provider, an interface for interacting with buckets on current or another server
pub trait PlabbleBucketProvider: Send + Sync {
    /// Connect to a Plabble server at the given address (e.g. "example.com:1234").
    fn connect(&mut self, address: &str) -> Result<(), PlabbleProtocolError>;

    /// Select a bucket by its ID (16 bytes).
    fn select_bucket(&mut self, bucket_id: &[u8; 16]) -> Result<(), PlabbleProtocolError>;

    /// Read numeric slot from selected bucket
    fn read(&self, slot: u32) -> Result<Vec<u8>, PlabbleProtocolError>;

    /// Write numeric slot in selected bucket
    fn write(&self, slot: u32, data: Vec<u8>) -> Result<(), PlabbleProtocolError>;

    /// Append data to selected bucket
    fn append(&self, data: Vec<u8>) -> Result<(), PlabbleProtocolError>;

    /// Delete numeric slot from selected bucket
    fn delete(&self, slot: u32) -> Result<(), PlabbleProtocolError>;
}

/// Plabble provider for interacting with Plabble blockchain
#[cfg(feature = "blockchain")]
pub trait BlockchainProvider: Send + Sync {
    /// Get the current block height of the blockchain.
    fn get_block_height(&self) -> Result<u64, PlabbleProtocolError>;

    /// Select transaction
    fn select_transaction(&mut self, transaction_id: &[u8; 24]) -> Result<(), PlabbleProtocolError>;

    /// Select block in blockchain
    fn select_block(&self, block_id: &[u8; 24]) -> Result<(), PlabbleProtocolError>;

    // TODO: what about other types than transactions?

    /// Get current transaction ID
    fn get_current_txid(&self) -> Result<[u8; 24], PlabbleProtocolError>;

    /// Get current transaction lock, if in the context of a transaction
    fn get_current_lock(&self) -> Result<TransactionLock, PlabbleProtocolError>;
}