use binary_codec::DeserializationError as BinaryDeserializationError;
use binary_codec::SerializationError as BinarySerializationError;

/// Plabble Serialization Error (based on binary_codec errors)
#[derive(Debug, PartialEq)]
pub enum SerializationError {
    /// Value is out of bounds (value, min, max)
    ValueOutOfBounds(i32, i32, i32),

    // Unexpected size (expected, actual)
    UnexpectedLength(usize, usize),

    /// Missing runtime length key
    MissingLengthByKey(String),

    /// Validation did fail for the data
    InvalidData(String),

    /// Encryption failed
    EncryptionFailed,
}

impl From<BinarySerializationError> for SerializationError {
    fn from(err: BinarySerializationError) -> Self {
        match err {
            BinarySerializationError::ValueOutOfBounds(a, b, c) => {
                SerializationError::ValueOutOfBounds(a, b, c)
            }
            BinarySerializationError::UnexpectedLength(a, b) => {
                SerializationError::UnexpectedLength(a, b)
            }
            BinarySerializationError::MissingLengthByKey(a) => {
                SerializationError::MissingLengthByKey(a)
            }
            BinarySerializationError::InvalidData(a) => SerializationError::InvalidData(a),
        }
    }
}

/// Plabble Deserialization Error (based on binary_codec errors)
#[derive(Debug, PartialEq)]
pub enum DeserializationError {
    /// Not enough bytes (bytes missing)
    NotEnoughBytes(usize),

    // Unexpected size (expected, actual)
    UnexpectedLength(usize, usize),

    /// Unknown enum discriminator
    UnknownDiscriminant(u8),

    /// Missing runtime length key
    MissingLengthByKey(String),

    /// Validation did fail for the data
    InvalidData(String),

    /// Decryption failed (e.g. due to wrong key or corrupted data)
    DecryptionFailed,

    /// MAC verification failed (e.g. due to wrong key or corrupted data)
    IntegrityFailed,
}

impl From<BinaryDeserializationError> for DeserializationError {
    fn from(err: BinaryDeserializationError) -> Self {
        match err {
            BinaryDeserializationError::NotEnoughBytes(a) => {
                DeserializationError::NotEnoughBytes(a)
            }
            BinaryDeserializationError::UnexpectedLength(a, b) => {
                DeserializationError::UnexpectedLength(a, b)
            }
            BinaryDeserializationError::UnknownDiscriminant(a) => {
                DeserializationError::UnknownDiscriminant(a)
            }
            BinaryDeserializationError::MissingLengthByKey(a) => {
                DeserializationError::MissingLengthByKey(a)
            }
            BinaryDeserializationError::InvalidData(a) => DeserializationError::InvalidData(a),
        }
    }
}
