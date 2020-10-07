use actix_web::{get, Responder, web, HttpRequest};
use actix_identity::Identity;
use askama_actix::{Template, TemplateIntoResponse};
use super::UrlFor;

#[derive(Template)]
#[template(path = "gear/gear.html")]
struct GearTemplate<'a> {
    url: UrlFor,
    id: Identity,
    user: &'a str,
    gear: &'a str,
    title: &'a str,
}

pub async fn gear(
    req: HttpRequest,
    id: Identity,
    web::Path((user, gear)): web::Path<(String, String)>
    ) -> impl Responder {

    GearTemplate {
        url: UrlFor::new(&id, req),
        id: id,
        user: &user,
        gear: &gear,
        title: &gear,
    }.into_response()
}

#[derive(Template)]
#[template(path = "gear/gearindex.html")]
struct GearIndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    user: &'a str,
    title: &'a str,
}

pub async fn gearindex(
    req: HttpRequest,
    id: Identity,
    user: web::Path<String>
    ) -> impl Responder {

    GearIndexTemplate {
        url: UrlFor::new(&id, req),
        id: id,
        user: &user,
        title: "Gear",
    }.into_response()
}

