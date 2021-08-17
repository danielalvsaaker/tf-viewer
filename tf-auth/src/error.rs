use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use thiserror::Error;

pub type Result = std::result::Result<OwnerConsent, OwnerConsent>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error.")]
    SledError {
        #[from]
        source: sled::Error,
    },
    #[error("Serialization error.")]
    RmpsError {
        #[from]
        source: rmp_serde::encode::Error,
    },
    #[error("{0}.")]
    BadServerResponse(&'static str),
    #[error("{1}.")]
    BadRequest(StatusCode, &'static str),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        if let Self::BadRequest(code, _) = &self {
            *code
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::NotFound()
            .finish()
    }
}
