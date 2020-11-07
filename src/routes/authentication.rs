use super::UrlFor;
use actix_identity::Identity;
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "auth/login.html")]
struct LoginTemplate<'a> {
    url: UrlFor,
    title: &'a str,
    message: Option<&'a str>,
    id: Identity,
}

pub async fn login(req: HttpRequest, id: Identity) -> impl Responder {
    LoginTemplate {
        url: UrlFor::new(&id, req)?,
        title: "Log in",
        message: None,
        id,
    }
    .into_response()
}

#[derive(Deserialize)]
pub struct Credentials {
    pub username: String,
    password: String,
}

pub async fn login_post(
    form: web::Form<Credentials>,
    data: web::Data<crate::Database>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {
    let username = form.username.clone();
    let password = form.password.clone();

    if data.get_ref().users.exists(&form.username).unwrap()
        && web::block(move || data.get_ref().users.verify_hash(&username, &password))
            .await
            .unwrap()
    {
        id.remember(form.username.to_owned());

        return Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish()
            .into_body());
    }

    LoginTemplate {
        url: UrlFor::new(&id, req)?,
        title: "Login",
        message: Some("Wrong username or password"),
        id,
    }
    .into_response()
}

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
    message: Option<&'a str>,
    id: Identity,
}

pub async fn register(req: HttpRequest, id: Identity) -> impl Responder {
    RegisterTemplate {
        url: UrlFor::new(&id, req)?,
        title: "Register",
        message: None,
        id,
    }
    .into_response()
}

pub async fn register_post(
    form: web::Form<Credentials>,
    data: web::Data<crate::Database>,
    id: Identity,
) -> impl Responder {
    if !data.get_ref().users.exists(&form.username).unwrap() {
        web::block(move || {
            data.get_ref()
                .users
                .insert(crate::User::new(), &form.username, &form.password)
        })
        .await;
    }

    HttpResponse::Found()
        .header(http::header::LOCATION, "/login")
        .finish()
        .into_body()
}
