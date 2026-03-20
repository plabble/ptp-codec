use crate::{core::PlabbleDateTime, packets::body::whisper::WhisperMetadata};

/// Slot history
pub struct SlotHistory {
    pub version: u32,
    pub last_update: PlabbleDateTime,
    pub deleted: bool,
}

impl SlotHistory {
    /// Check if the incoming message can be accepted based on version/timestamp/node ID
    pub fn can_accept(&self, incoming: &WhisperMetadata, self_id: &[u8; 16]) -> bool {
        // Do not accept clock skewed (> 2 sec future) message
        if incoming.timestamp.timestamp() > PlabbleDateTime::from_now(2).timestamp() {
            return false;
        }

        // Do not allow skewed versions (+10 versions in the future)
        if incoming.version > self.version + 10 {
            return false;
        }

        // Do not accept older versions
        if incoming.version < self.version {
            return false;
        }

        // If version is equal (conflict), try to resolve
        if incoming.version == self.version {
            // If the incoming message is newer, reject (first come first serve)
            if incoming.timestamp.timestamp() > self.last_update.timestamp() {
                return false;
            }

            if incoming.timestamp.timestamp() == self.last_update.timestamp() {
                // If the timestamps are equal, break ties by node ID (higher wins)
                if let Some(from) = &incoming.from {
                    if from < self_id {
                        return false;
                    }
                }
            }
        }

        true
    }
}