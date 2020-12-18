use super::UrlFor;
use actix_identity::Identity;
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use serde::Deserialize;

#[derive(Template)]
#[template(path = "auth/signin.html")]
struct SigninTemplate<'a> {
    url: UrlFor,
    title: &'a str,
    message: Option<&'a str>,
    id: Identity,
}

pub async fn signin(req: HttpRequest, id: Identity) -> impl Responder {
    SigninTemplate {
        url: UrlFor::new(&id, &req)?,
        title: "Sign in",
        message: None,
        id,
    }
    .into_response()
}

#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}

pub async fn signin_post(
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

    SigninTemplate {
        url: UrlFor::new(&id, &req)?,
        title: "Sign in",
        message: Some("Wrong username or password"),
        id,
    }
    .into_response()
}

pub async fn signout(id: Identity) -> impl Responder {
    id.forget();

    HttpResponse::Found()
        .header(http::header::LOCATION, "/")
        .finish()
        .into_body()
}

#[derive(Template)]
#[template(path = "auth/signup.html")]
struct SignupTemplate<'a> {
    url: UrlFor,
    title: &'a str,
    message: Option<&'a str>,
    id: Identity,
}

pub async fn signup(req: HttpRequest, id: Identity) -> impl Responder {
    SignupTemplate {
        url: UrlFor::new(&id, &req)?,
        title: "Sign up",
        message: None,
        id,
    }
    .into_response()
}

pub async fn signup_post(
    form: web::Form<Credentials>,
    data: web::Data<crate::Database>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {
    let result = || {
        let regex = regex::Regex::new(r#"^[a-zA-Z0-9]+$"#).unwrap();
        println!("{:#?}", regex.is_match(&form.username));
        if !regex.is_match(&form.username) {
            return Some("Invalid username supplied.");
        }
        if data.get_ref().users.exists(&form.username).unwrap() {
            return Some("Username is already taken.");
        }
        None
    };

    if result().is_none() {
        web::block(move || {
            data.get_ref()
                .users
                .insert(crate::User::new(), &form.username, &form.password)
        })
        .await;

        return Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/signin")
            .finish()
            .into_body());
    }

    SignupTemplate {
        url: UrlFor::new(&id, &req)?,
        title: "Sign up",
        message: result(),
        id,
    }
    .into_response()
}
