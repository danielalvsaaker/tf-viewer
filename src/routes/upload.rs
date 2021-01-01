use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use futures::{StreamExt, TryStreamExt};

use super::UrlFor;

#[derive(Template)]
#[template(path = "upload.html")]
struct UploadTemplate<'a> {
    url: UrlFor,
    id: Identity,
    title: &'a str,
}

pub async fn upload(id: Identity, req: HttpRequest) -> impl Responder {
    UploadTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        title: "Upload",
    }
    .into_response()
}

pub async fn upload_post(
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

    let parsed = crate::parser::parse(&f, gear);

    match parsed {
        Ok(x) => {
            let id = id.identity().unwrap();
            data.activities.insert(x, &id)
                .map(|_| HttpResponse::Ok().finish().into_body())
        }
        Err(x) => Ok(HttpResponse::BadRequest().body(x.to_string())),
    }
}
