use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    SledError {
        #[from]
        source: sled::Error,
    },

    #[error("Serialization error: {0}")]
    SerializeError {
        #[from]
        source: rmp_serde::encode::Error,
    },

    #[error("Deserialization error: {0}")]
    DeserializeError {
        #[from]
        source: rmp_serde::decode::Error,
    },
}
