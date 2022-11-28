use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Database error.")]
    Database {
        #[from]
        source: tf_database::error::Error,
    },
    #[error("Not found")]
    NotFound,

    #[error("Invalid key")]
    InvalidKey {
        #[from]
        source: tf_models::InvalidLengthError,
    },

    #[error("Invalid hash in database")]
    Hash {
        #[from]
        source: argon2::password_hash::Error,
    },

    #[error("{source}")]
    OAuth {
        #[from]
        source: oxide_auth_axum::WebError,
    },

    #[error("{source}")]
    JoinError {
        #[from]
        source: tokio::task::JoinError,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        if let Self::OAuth { source } = self {
            source.into_response()
        } else {
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
