use askama_actix::{Template, TemplateIntoResponse};
use actix_web::{Responder, HttpRequest, web};
use actix_identity::Identity;

use super::UrlFor;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    activities: Vec<(crate::Session, String)>,
    title: &'a str,
}

pub async fn index(
    id: Identity,
    req: HttpRequest,
    data: web::Data<crate::Database>
    ) -> impl Responder {

    let sessions = data.as_ref().activities.iter_session(5).unwrap();
    let ids = data.as_ref().activities.iter_all_id(5).unwrap();
    let activities: Vec<(crate::Session, String)> = sessions.zip(ids).collect();

    IndexTemplate {
        url: UrlFor::new(&id, req),
        id: id,
        activities: activities,
        title: "Index",
    }.into_response()
}
