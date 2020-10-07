use actix_web::{get, post, Responder, web, HttpRequest, HttpResponse};
use actix_identity::Identity;
use askama_actix::{Template, TemplateIntoResponse};
use serde::Deserialize;
use super::UrlFor;

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate<'a> {
    url: UrlFor,
    title: &'a str,
}


#[get("/login")]
pub async fn login(
    req: HttpRequest,
    id: Identity
    ) -> impl Responder {

    LoginTemplate {
        url: UrlFor::new(&id, req),
        title: "Log in",
    }.into_response()
}

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    password: String,
}

#[post("/login")]
pub async fn login_post(
    form: web::Form<Credentials>,
    id: Identity
    ) -> impl Responder {

    id.remember(form.username.to_owned());

    HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body()
}


#[get("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.forget();

    HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body()
}

#[derive(Template)]
#[template(path = "auth/register.html")]
struct RegisterTemplate<'a> {
    url: UrlFor,
    title: &'a str,
}

pub async fn register(
    req: HttpRequest,
    id: Identity
    ) -> impl Responder {

    RegisterTemplate {
        url: UrlFor::new(&id, req),
        title: "Register",
    }.into_response()
}

pub async fn register_post(
    form: web::Form<Credentials>,
    id: Identity
    ) -> impl Responder {

    HttpResponse::Found()
        .header(http::header::LOCATION, "/login")
        .finish()
        .into_body()
}
