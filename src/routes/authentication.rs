use super::{utils, UrlFor};
use crate::middleware::{AuthType, CheckLogin};
use actix_identity::Identity;
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/signin")
            .name("signin")
            .wrap(CheckLogin::new(AuthType::Public))
            .route(web::get().to(signin))
            .route(web::post().to(signin_post)),
    )
    .service(
        web::resource("/signup")
            .name("signup")
            .wrap(CheckLogin::new(AuthType::Public))
            .route(web::get().to(signup))
            .route(web::post().to(signup_post)),
    )
    .service(
        web::resource("/signout")
            .name("signout")
            .wrap(CheckLogin::new(AuthType::Restricted))
            .to(signout),
    );
}

#[derive(Template)]
#[template(path = "auth/signin.html")]
struct SigninTemplate<'a> {
    url: UrlFor,
    title: &'a str,
    message: Option<&'a str>,
    id: Identity,
}

async fn signin(req: HttpRequest, id: Identity) -> impl Responder {
    SigninTemplate {
        url: UrlFor::new(&id, &req)?,
        title: "Sign in",
        message: None,
        id,
    }
    .into_response()
}

#[derive(Deserialize)]
pub struct AuthForm {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub confirm_password: String,
}

async fn signin_post(
    form: web::Form<AuthForm>,
    data: web::Data<crate::Database>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {
    if data.users.exists(&form.username).is_ok()
        && data.users.verify_hash(&form.username, &form.password).is_ok()
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
        message: Some("Wrong username or password."),
        id,
    }
    .into_response()
}

async fn signout(id: Identity) -> impl Responder {
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
    message: &'a Option<crate::error::Error>,
    id: Identity,
}

async fn signup(req: HttpRequest, id: Identity) -> impl Responder {
    SignupTemplate {
        url: UrlFor::new(&id, &req)?,
        title: "Sign up",
        message: &None,
        id,
    }
    .into_response()
}

async fn signup_post(
    form: web::Form<AuthForm>,
    data: web::Data<crate::Database>,
    req: HttpRequest,
    id: Identity,
) -> impl Responder {
    let validation = utils::validate_form(&super::PasswordEnum::Signup(&form), &data);

    if validation.is_ok() {
        data.users.insert(&form.username, &form.password)?;

        return Ok(HttpResponse::Found()
            .header(http::header::LOCATION, "/signin")
            .finish()
            .into_body());
    }

    SignupTemplate {
        url: UrlFor::new(&id, &req)?,
        title: "Sign up",
        message: &validation.err(),
        id,
    }
    .into_response()
}
