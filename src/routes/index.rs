use askama_actix::{Template, TemplateIntoResponse};
use actix_web::{Responder, HttpRequest};
use actix_identity::Identity;

use super::UrlFor;

#[derive(Template)]
#[template(path = "base.html")]
struct IndexTemplate<'a> {
    url: UrlFor,
    user: Identity,
    title: &'a str,
}


pub async fn index(id: Identity, req: HttpRequest) -> impl Responder {
    IndexTemplate {
        url: UrlFor::new(&id, req),
        user: id,
        title: "Index",
    }.into_response()
}
