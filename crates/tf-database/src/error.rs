use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {source}")]
    SledError {
        #[from]
        source: sled::Error,
    },

    #[error("Transaction error")]
    TransactionError {
        #[from]
        source: sled::transaction::TransactionError,
    },

    #[error("Serialization error: {source}")]
    SerializeError {
        #[from]
        source: rmp_serde::encode::Error,
    },

    #[error("Deserialization error: {source}")]
    DeserializeError {
        #[from]
        source: rmp_serde::decode::Error,
    },
}
