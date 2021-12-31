use crate::templates::index_template;
use actix_web::{HttpResponse, Responder};

pub async fn get_index() -> impl Responder {
    HttpResponse::Ok().body(index_template())
}
