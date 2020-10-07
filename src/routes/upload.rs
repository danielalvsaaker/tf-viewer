use askama_actix::{Template, TemplateIntoResponse};
use actix_web::{Responder, HttpRequest, get, HttpResponse};
use actix_identity::Identity;
use actix_multipart::{Multipart, Field};
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
        url: UrlFor::new(&id, req),
        id: id,
        title: "Upload",
    }.into_response()
}

pub async fn upload_post(
    req: HttpRequest,
    id: Identity,
    mut payload: Multipart
    ) -> impl Responder {


    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .ok_or_else(|| actix_web::error::ParseError::Incomplete).expect("Fail");

        let mut f: Vec<u8> = Vec::new();
            
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            f.extend_from_slice(data.as_ref());
        }

        let activity = crate::parser::parse(f.as_slice());
        println!("{}", activity.session.duration.unwrap());
    }


    
    HttpResponse::Found()
        .header(http::header::LOCATION, "/upload")
        .finish()
        .into_body()
}
