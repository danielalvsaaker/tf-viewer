use actix_web::{get, Responder, web, HttpRequest};
use actix_identity::Identity;
use askama_actix::{Template, TemplateIntoResponse};
use super::UrlFor;

#[derive(Template)]
#[template(path = "user/user.html")]
struct UserTemplate<'a> {
    url: UrlFor,
    id: Identity,
    user: &'a str,
    title: &'a str,
}

pub async fn user(
    req: HttpRequest,
    id: Identity,
    user: web::Path<String>
    ) -> impl Responder {

    UserTemplate {
        url: UrlFor::new(&id, req),
        id: id,
        user: &user,
        title: &user,
    }.into_response()
}

#[derive(Template)]
#[template(path = "user/userindex.html")]
struct UserIndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    title: &'a str,
}

pub async fn userindex(
    req: HttpRequest,
    id: Identity
    ) -> impl Responder {

    UserIndexTemplate {
        url: UrlFor::new(&id, req),
        id: id,
        title: "Users",
    }.into_response()
}
