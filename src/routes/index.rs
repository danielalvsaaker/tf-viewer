use askama_actix::{Template, TemplateIntoResponse};
use actix_web::{Responder, HttpRequest, web};
use actix_identity::Identity;

use super::{UrlFor, FormatDuration};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    activities: &'a Vec<(crate::Session, String)>,
    title: &'a str,
}

pub async fn index(
    id: Identity,
    req: HttpRequest,
    data: web::Data<crate::Database>
    ) -> impl Responder {

    let sessions = data.as_ref().activities.iter_session(5).unwrap();
    let records = data.as_ref().activities.iter_record(5).unwrap();
    let ids: Vec<String> = data.as_ref().activities.iter_all_id(5).unwrap().collect();
    let activities: Vec<(crate::Session, String)> = sessions.zip(ids.clone()).collect();

    {
        for (record, id) in records.zip(ids) {
            std::thread::spawn(move || super::utils::generate_thumb(record, &id));
        }
    }


    IndexTemplate {
        url: UrlFor::new(&id, req),
        id,
        activities: &activities,
        title: "Index",
    }.into_response()
}
