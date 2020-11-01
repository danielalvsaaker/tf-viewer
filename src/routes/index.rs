use askama_actix::{Template, TemplateIntoResponse};
use actix_web::{Responder, HttpRequest, web};
use actix_identity::Identity;
use crate::Session;
use url::Url;

use super::{UrlFor, UrlActivity, FormatDuration};

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
    data: web::Data<crate::Database>
    ) -> impl Responder {

    //let sessions: Vec<Session> = data.as_ref().activities.iter_session_all().unwrap().take(5).collect();
    
    let usernames = data.as_ref().activities.iter_username_all().unwrap();
    let ids = data.as_ref().activities.iter_id_all().unwrap();
    let mut username_id: Vec<(String, String)> = usernames
        .zip(ids)
        .collect();

    username_id.sort_by_key(|(a,k)| k.parse::<u64>().unwrap());
    username_id.reverse();
    username_id.truncate(5);

    let mut session: Vec<Session> = Vec::new();
    for (username, id) in &username_id {
        session.push(data.as_ref().activities.get_session(&username, &id).unwrap());
    }

    for (username, id) in &username_id {
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

    // This is necessary because Askama does not allow zipping iterators inside a template
    let template_data: Vec<TemplateData> = session.into_iter()
        .zip(username_id)
        .map(|(x, (y, z))| 
             TemplateData {
                 session: x,
                 url: UrlActivity::new(&y, &z, &req),
                 username: y,
                 id: z,
             }
        )
        .collect();

    IndexTemplate {
        url: UrlFor::new(&id, req),
        id,
        template_data: &template_data,
        title: "Index",
    }.into_response()
}
