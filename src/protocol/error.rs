use crate::{errors::{DeserializationError, SerializationError}, packets::body::error::PlabbleError};

pub enum PlabbleProtocolError {
    SerializationError(SerializationError),
    DeserializationError(DeserializationError),
    ProtocolError(PlabbleError),
    SenderError,
    ReceiverError,
    UnexpectedResponse,
    FailedToProcessResponse
}

#[repr(u16)]
#[derive(Debug)]
pub enum PlabbleStatusCode {
    Ok = 0,
    
    /* 1 - 255: Plabble protocol error codes */
    InternalServerError = 255,

    SerializationFailed = 256,
    DeserializationFailed = 257,
    EncryptionFailed = 258,
    DecryptionFailed = 259,
    IntegrityCheckFailed = 260,

    TransportError = 300,
    UnexpectedResponse = 301,
    FailedToProcessResponse = 302,
}

impl From<PlabbleProtocolError> for PlabbleStatusCode {
    fn from(value: PlabbleProtocolError) -> Self {
        match value {
            PlabbleProtocolError::SerializationError(s) => {
                match s {
                    SerializationError::EncryptionFailed | SerializationError::NoKeyAvailable =>
                        PlabbleStatusCode::EncryptionFailed,
                    _ => PlabbleStatusCode::SerializationFailed,
                }
            }
            PlabbleProtocolError::DeserializationError(d) => {
                match d {
                    DeserializationError::DecryptionFailed | DeserializationError::NoKeyAvailable =>
                        PlabbleStatusCode::DecryptionFailed,
                    DeserializationError::IntegrityFailed => PlabbleStatusCode::IntegrityCheckFailed,
                    _ => PlabbleStatusCode::DeserializationFailed,
                }
            }
            PlabbleProtocolError::ProtocolError(e) => {
                match e {
                    _ => PlabbleStatusCode::InternalServerError
                }
            },
            PlabbleProtocolError::SenderError | PlabbleProtocolError::ReceiverError => PlabbleStatusCode::TransportError,
            PlabbleProtocolError::UnexpectedResponse => PlabbleStatusCode::UnexpectedResponse,
            PlabbleProtocolError::FailedToProcessResponse => PlabbleStatusCode::FailedToProcessResponse,
        }
    }
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