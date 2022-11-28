use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {source}")]
    Database {
        #[from]
        source: tf_database::error::Error,
    },
    #[error("Parse error.")]
    Parse {
        #[from]
        source: tf_parse::error::Error,
    },
    #[error("Not found")]
    NotFound,

    #[error("{source}")]
    JoinError {
        #[from]
        source: tokio::task::JoinError,
    },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        #[cfg(debug_assertions)]
        {
            (status_code, self.to_string()).into_response()
        }

        #[cfg(not(debug_assertions))]
        status_code.into_response()
    }
}
