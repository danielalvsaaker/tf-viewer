use actix_web::{
    http::StatusCode,
    ResponseError,
};
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
    BincodeError {
        #[from]
        source: bincode::Error,
    },
    #[error("{0}")]
    BadServerResponse(&'static str),
    #[error("{1}")]
    BadRequest(ErrorKind, &'static str),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        if let Self::BadRequest(kind, _) = &self {
            match kind {
                ErrorKind::BadRequest => StatusCode::BAD_REQUEST,
                ErrorKind::Forbidden => StatusCode::FORBIDDEN,
                ErrorKind::NotFound => StatusCode::NOT_FOUND,
            }
        }
        else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}


#[derive(Debug)]
pub enum ErrorKind {
    BadRequest,
    Forbidden,
    NotFound,
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            ErrorKind::BadRequest => "Bad request",
            ErrorKind::Forbidden => "Forbidden",
            ErrorKind::NotFound => "Not found",
        };
        write!(f, "{}", kind)
    }
}
