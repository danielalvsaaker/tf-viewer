use crate::Session;
use actix_identity::Identity;
use actix_web::{web, HttpRequest, Responder};
use askama_actix::{Template, TemplateIntoResponse};

use super::{UrlActivity, UrlFor};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    template_data: &'a Vec<TemplateData>,
    title: &'a str,
}

struct TemplateData {
    session: Session,
    url: UrlActivity,
    username: String,
    id: String,
}

pub async fn index(
    id: Identity,
    req: HttpRequest,
    data: web::Data<crate::Database>,
) -> impl Responder {
    let (username_iter, id_iter) = (
        data.activities.iter_username().unwrap(),
        data.activities.iter_id().unwrap(),
    );

    let mut username_id: Vec<(String, String)> = username_iter.zip(id_iter).collect();

    username_id.sort_by_key(|(_, k)| k.parse::<u64>().unwrap());
    username_id.reverse();
    username_id.truncate(5);

    let mut sessions: Vec<Session> = Vec::new();
    for (username, id) in username_id.iter() {
        let session = data.activities.get_session(&username, &id).unwrap();
        sessions.push(session);
    }

    for (username, id) in username_id.iter() {
        let path = format!("static/img/activity/{}_{}.png", &username, &id);
        let path = std::path::PathBuf::from(&path);

        if !path.exists() {
            let record = data.activities.get_record(&username, &id).unwrap();

            std::thread::spawn(move || {
                // Creating file prematurely, preventing more processes from spawning
                // and performing the same task
                std::fs::File::create(&path);

                super::utils::generate_thumb(record, path);
            });
        }
    }

    // This is necessary because Askama does not allow zipping iterators inside a template
    let template_data: Vec<TemplateData> = sessions
        .into_iter()
        .zip(username_id)
        .map(|(x, (y, z))| TemplateData {
            session: x,
            url: UrlActivity::new(&y, &z, &req).unwrap(),
            username: y,
            id: z,
        })
        .collect();

    IndexTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        template_data: &template_data,
        title: "Recent activities",
    }
    .into_response()
}
