use super::UrlFor;
use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use futures::{StreamExt, TryStreamExt};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/upload")
            .name("upload")
            .route(web::get().to(upload))
            .route(web::post().to(upload_post)),
    );
}

#[derive(Template)]
#[template(path = "upload.html")]
struct UploadTemplate<'a> {
    url: UrlFor,
    id: Identity,
    title: &'a str,
}

async fn upload(id: Identity, req: HttpRequest) -> impl Responder {
    UploadTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        title: "Upload",
    }
    .into_response()
}

async fn upload_post(
    data: web::Data<crate::Database>,
    id: Identity,
    mut payload: Multipart,
) -> impl Responder {
    let mut f: Vec<u8> = Vec::new();

    while let Ok(Some(mut field)) = payload.try_next().await {
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            f.extend_from_slice(chunk.as_ref());
        }
    }

    let gear = data
        .users
        .get_standard_gear(id.identity().unwrap().as_str())?;

    if let Ok(x) = crate::parser::fit::parse(&f, gear.clone()) {
        let id = id.identity().unwrap();
        data.activities
            .insert(x, &id)
            .map(|_| HttpResponse::Ok().finish().into_body())
    } else if let Ok(x) = crate::parser::gpxp::parse(&f, gear.clone()) {
        let id = id.identity().unwrap();
        data.activities
            .insert(x, &id)
            .map(|_| HttpResponse::Ok().finish().into_body())
    } else {

    // match parsed {
    //     Ok(x) => {
    //         let id = id.identity().unwrap();
    //         data.activities
    //             .insert(x, &id)
    //             .map(|_| HttpResponse::Ok().finish().into_body())
    //     }
        Ok(HttpResponse::BadRequest().body("FIT or GPX invalid".to_string()))
    }
}
