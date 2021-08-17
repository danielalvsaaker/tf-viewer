use actix_identity::Identity;
use actix_web::{HttpRequest, HttpResponse, Responder};

/*
pub async fn get_signin(req: HttpRequest, id: Identity) -> impl Responder {
    const SIGNIN_TEMPLATE: &'static str = include_str!("../templates/signin.html");
    SIGNIN_TEMPLATE
}

pub async fn get_signup(req: HttpRequest, id: Identity) -> impl Responder {
    const SIGNUP_TEMPLATE: &'static str = include_str!("../templates/signup.html");
    SIGNUP_TEMPLATE
}

pub async fn get_stylesheet() -> impl Responder {
    const STYLESHEET: &'static [u8] = include_bytes!("../static/spectre.min.css");
    "test"
}
*/

pub async fn get_signin(id: Identity) -> impl Responder {
    id.remember("test-user".to_owned());
    HttpResponse::Ok()
}
