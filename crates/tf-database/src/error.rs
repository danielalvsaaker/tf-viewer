use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal error")]
    InternalError {
        #[from]
        source: nebari::Error,
    },

    #[error("Foreign key constraint error")]
    ForeignKeyConstraint,

    #[error("Invalid key")]
    InvalidKey {
        #[from]
        source: tf_models::InvalidLengthError,
    },

    #[error("Serialization error: {source}")]
    SerializeError {
        #[from]
        source: flexbuffers::SerializationError,
    },

    #[error("Deserialization error: {source}")]
    DeserializeError {
        #[from]
        source: flexbuffers::DeserializationError,
    },
}

impl From<nebari::AbortError<Self>> for Error {
    fn from(e: nebari::AbortError<Self>) -> Self {
        match e {
            nebari::AbortError::Nebari(source) => Self::InternalError { source },
            nebari::AbortError::Other(inner) => inner,
        }
    }
}
