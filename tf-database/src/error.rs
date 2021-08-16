use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error.")]
    SledError {
        #[from]
        source: sled::Error,
    },

    #[error("Serialization error.")]
    SerializeError {
        #[from]
        source: rmp_serde::encode::Error,
    },

    #[error("Deserialization error.")]
    DeserializeError {
        #[from]
        source: rmp_serde::decode::Error,
    },
}
