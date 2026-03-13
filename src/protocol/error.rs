use std::fmt;

use crate::{
    errors::{DeserializationError, SerializationError},
    packets::body::error::PlabbleError,
};

#[cfg_attr(feature = "ffi", derive(uniffi::Error))]
#[cfg_attr(feature = "ffi", uniffi(flat_error))]
#[derive(Debug)]
pub enum PlabbleProtocolError {
    SerializationError(SerializationError),
    DeserializationError(DeserializationError),
    ProtocolError(PlabbleError),
    SenderError,
    ReceiverError,
    UnexpectedResponse,
    FailedToProcessResponse,
    InputParsingFailed,
    OutputSerializationFailed,
}

impl From<SerializationError> for PlabbleProtocolError {
    fn from(value: SerializationError) -> Self {
        PlabbleProtocolError::SerializationError(value)
    }
}

impl From<DeserializationError> for PlabbleProtocolError {
    fn from(value: DeserializationError) -> Self {
        PlabbleProtocolError::DeserializationError(value)
    }
}

impl From<PlabbleError> for PlabbleProtocolError {
    fn from(value: PlabbleError) -> Self {
        PlabbleProtocolError::ProtocolError(value)
    }
}

impl fmt::Display for PlabbleProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SerializationError(e) => write!(f, "Serialization error: {:?}", e),
            Self::DeserializationError(e) => write!(f, "Deserialization error: {:?}", e),
            Self::ProtocolError(e) => write!(f, "Protocol error: {:?}", e),
            Self::SenderError => write!(f, "Sender error"),
            Self::ReceiverError => write!(f, "Receiver error"),
            Self::UnexpectedResponse => write!(f, "Unexpected response"),
            Self::FailedToProcessResponse => write!(f, "Failed to process response"),
            Self::InputParsingFailed => write!(f, "Input parsing failed"),
            Self::OutputSerializationFailed => write!(f, "Output serialization failed"),
        }
    }
}

impl std::error::Error for PlabbleProtocolError {}
