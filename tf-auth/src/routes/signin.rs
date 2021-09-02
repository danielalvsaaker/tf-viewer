use super::Callback;
use actix_identity::Identity;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

use crate::templates::signin_template;

pub async fn get_signin(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(signin_template(req.uri().query().unwrap_or_default()))
}

pub async fn post_signin(id: Identity, query: Option<web::Query<Callback>>) -> impl Responder {
    id.remember("test-user".to_owned());

    if let Some(q) = query {
        HttpResponse::Found()
            .append_header(("Location", q.into_inner().callback))
            .finish()
    } else {
        HttpResponse::Found()
            .append_header(("Location", "index"))
            .finish()
    }
}
