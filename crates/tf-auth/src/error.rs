use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error.")]
    Database {
        #[from]
        source: sled::Error,
    },
    #[error("Not found")]
    NotFound,
    #[error("Invalid hash in database")]
    Hash {
        #[from]
        source: argon2::password_hash::Error,
    },
}

impl actix_web::ResponseError for Error {}
