use actix_web::{http::StatusCode, ResponseError};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
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
    #[error("Web error.")]
    Web {
        #[from]
        source: actix_web::error::Error,
    },
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
