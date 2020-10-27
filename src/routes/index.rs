use askama_actix::{Template, TemplateIntoResponse};
use actix_web::{Responder, HttpRequest, web};
use actix_identity::Identity;
use crate::Session;

use super::{UrlFor, FormatDuration};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    session_id_username: &'a Vec<(crate::Session, String, String)>,
    title: &'a str,
}

pub async fn index(
    id: Identity,
    req: HttpRequest,
    data: web::Data<crate::Database>
    ) -> impl Responder {

    let sessions: Vec<Session> = data.as_ref().activities.iter_session_all().unwrap().take(5).collect();
    let ids: Vec<String> = data.as_ref().activities.iter_id_all().unwrap().take(5).collect();
    let usernames: Vec<String> = data.as_ref().activities.iter_username_all().unwrap().take(5).collect();

    // This is necessary because Askama does not allow zipping iterators inside a template
    let session_id_username: Vec<(crate::Session, String, String)> = sessions.into_iter()
        .zip(ids.clone())
        .zip(usernames.clone())
        .map(|((x, y), z)| (x, y, z))
        .collect();

    for (id, username) in ids.into_iter().zip(usernames) {
        let path = format!("static/img/activity/{}.png", id);
        let path = std::path::PathBuf::from(&path);
        
        if !path.exists() {
            let record = data.as_ref().activities.get_record(&username, &id).unwrap();
                
            // Creating file prematurely, preventing more processes from spawning
            // and performing the same task
            std::fs::File::create(&path)?;

            std::thread::spawn(move || super::utils::generate_thumb(record, path));
        }
    }

    IndexTemplate {
        url: UrlFor::new(&id, req),
        id,
        session_id_username: &session_id_username,
        title: "Index",
    }.into_response()
}
