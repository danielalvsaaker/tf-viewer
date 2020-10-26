use actix_identity::Identity;
use askama_actix::{Template, TemplateIntoResponse};
use actix_web::{HttpRequest, Responder, dev, http, Result, FromRequest, dev::ServiceResponse, http::HeaderValue, HttpResponse};
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use super::UrlFor;
use futures::future::{ok, Either, Ready};

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    url: UrlFor,
    id: Identity,
    title: &'a str,
    text: &'a str,
}

impl<'a> ErrorTemplate<'a> {
    pub async fn not_found(req: HttpRequest, id: Identity) -> Result<actix_web::HttpResponse> {
        let body = ErrorTemplate {
                    url: UrlFor::new(&id, req),
                    id,
                    title: "404 Not found",
                    text: "Page not found",
                }.render().unwrap();
        Ok(HttpResponse::NotFound().content_type("text/html").body(body))
    }
}
