use super::{UrlActivity, UrlFor};
use crate::models::Session;
use actix_identity::Identity;
use actix_web::{web, HttpRequest, Responder};
use askama_actix::{Template, TemplateIntoResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").name("index").to(index));
}

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

async fn index(id: Identity, req: HttpRequest, data: web::Data<crate::Database>) -> impl Responder {
    let (username_iter, id_iter) = (data.activities.iter_username()?, data.activities.iter_id()?);

    let mut username_id: Vec<(String, String)> = username_iter.zip(id_iter).collect();

    username_id.sort_by_key(|(_, k)| k.parse::<u64>().unwrap());
    username_id.reverse();
    username_id.truncate(5);

    let sessions: Vec<Session> = username_id
        .iter()
        .flat_map(|(x, y)| data.activities.get_session(&x, &y))
        .collect();

    for (username, id) in username_id.iter() {
        let path = format!("static/img/activity/{}_{}.png", &username, &id);
        let path = std::path::PathBuf::from(&path);

        if !path.exists() {
            let record = data.activities.get_record(&username, &id)?;

            web::block(move || super::utils::generate_thumb(record, &path)).await?;
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
