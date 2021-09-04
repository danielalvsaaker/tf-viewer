use super::UserForm;
use crate::templates::signup_template;
use crate::{error::Result, Database};
use actix_web::{web, HttpRequest, HttpResponse, Responder};

pub async fn get_signup(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(signup_template(req.uri().query().unwrap_or_default()))
}

pub async fn post_signup(
    req: HttpRequest,
    db: web::Data<Database>,
    web::Form(user): web::Form<UserForm>,
) -> Result<impl Responder> {
    db.insert(&user)?;

    Ok(HttpResponse::Found()
        .append_header((
            "Location",
            format!("{}?{}", "signin", req.uri().query().unwrap_or_default()),
        ))
        .finish())
}
