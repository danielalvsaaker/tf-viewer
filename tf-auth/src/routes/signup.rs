use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::templates::signup_template;
use super::UserForm;

pub async fn get_signup(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(signup_template(req.uri().query().unwrap_or_default()))
}

pub async fn post_signup(
    req: HttpRequest,
    web::Form(user): web::Form<UserForm>,
) -> impl Responder {
    dbg!(&user.username);
    HttpResponse::Found()
        .append_header((
            "Location",
            format!("{}?{}", "signin", req.uri().query().unwrap_or_default()),
        ))
        .finish()
}
