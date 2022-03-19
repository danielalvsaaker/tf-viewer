use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error.")]
    Database {
        #[from]
        source: tf_database::error::Error,
    },
    #[error("Not found")]
    NotFound,
    #[error("Invalid hash in database")]
    Hash {
        #[from]
        source: argon2::password_hash::Error,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
