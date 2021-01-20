use actix_web::{dev::HttpResponseBuilder, http::StatusCode, web, HttpResponse, ResponseError};
use askama_actix::Template;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.app_data(web::FormConfig::default().error_handler(|_, _| {
        Error::BadRequest(ErrorKind::BadRequest, "Failed to parse form").into()
    }))
    .app_data(web::PathConfig::default().error_handler(|_, _| {
        Error::BadRequest(ErrorKind::BadRequest, "Failed to parse the requested path").into()
    }));
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate<'a> {
    title: &'a str,
    text: &'a Error,
}

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
    BadRequest(ErrorKind, &'static str),
}

#[derive(Debug)]
pub enum ErrorKind {
    BadRequest,
    Forbidden,
    NotFound,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        if let Self::BadRequest(kind, _) = &self {
            match kind {
                ErrorKind::BadRequest => StatusCode::BAD_REQUEST,
                ErrorKind::Forbidden => StatusCode::FORBIDDEN,
                ErrorKind::NotFound => StatusCode::NOT_FOUND,
            }
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    // Ugly result, and the trait should optimally serve HttpRequest as a parameter
    // for this method
    fn error_response(&self) -> HttpResponse {
        let title = match self.status_code() {
            StatusCode::BAD_REQUEST => "Bad request",
            StatusCode::FORBIDDEN => "Forbidden",
            StatusCode::NOT_FOUND => "Not found",
            _ => "Internal server error",
        };

        let response = ErrorTemplate {
            title: &title,
            text: &self,
        }
        .render()
        .expect("Failed to render error template");

        HttpResponseBuilder::new(self.status_code())
            .content_type("text/html")
            .body(response)
    }
}
