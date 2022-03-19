use sled::transaction::TransactionError;
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal error")]
    InternalError {
        #[from]
        source: nebari::Error,
        //source: sled::Error,
    },

    #[error("Foreign key constraint error")]
    ForeignKeyConstraint,

    #[error("Invalid key")]
    InvalidKey {
        #[from]
        source: tf_models::InvalidLengthError,
    },

    #[error("Serialization/deserialization error: {source}")]
    PotError {
        #[from]
        source: pot::Error,
    },
}

/*
impl From<TransactionError<Self>> for Error {
    fn from(e: TransactionError<Self>) -> Self {
        match e {
            TransactionError::Abort(inner) => inner,
            TransactionError::Storage(source) => Self::InternalError { source },
        }
    }
}
*/
