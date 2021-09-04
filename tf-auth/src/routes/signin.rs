use super::Callback;
use actix_identity::Identity;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::{error::Result, routes::UserForm, templates::signin_template, Database};

pub async fn get_signin(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(signin_template(req.uri().query().unwrap_or_default()))
}

pub async fn post_signin(
    id: Identity,
    req: HttpRequest,
    db: web::Data<Database>,
    query: Option<web::Query<Callback>>,
    web::Form(user): web::Form<UserForm>,
) -> Result<impl Responder> {
    if db.verify_hash(&user)? {
        id.remember(user.username);
    } else {
        return Ok(HttpResponse::Unauthorized()
            .body(signin_template(req.uri().query().unwrap_or_default())));
    }

    if let Some(q) = query {
        Ok(HttpResponse::Found()
            .append_header(("Location", q.into_inner().callback))
            .finish())
    } else {
        Ok(HttpResponse::Found()
            .append_header(("Location", "index"))
            .finish())
    }
}
