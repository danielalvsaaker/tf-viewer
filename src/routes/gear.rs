use super::UrlFor;
use crate::Gear;
use actix_identity::Identity;
use actix_web::{web, web::block, HttpRequest, Responder};
use askama_actix::{Template, TemplateIntoResponse};

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
    web::Path((user, gear)): web::Path<(String, String)>,
) -> impl Responder {
    GearTemplate {
        url: UrlFor::new(&id, req)?,
        id,
        user: &user,
        gear: &gear,
        title: &gear,
    }
    .into_response()
}

#[derive(Template)]
#[template(path = "gear/index.html")]
struct GearIndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    gears: Vec<((f64, f64), Gear)>,
    user: &'a str,
    title: &'a str,
}

pub async fn gear_index(
    req: HttpRequest,
    id: Identity,
    user: web::Path<String>,
    data: web::Data<crate::Database>,
) -> impl Responder {
    let gears: Vec<((f64, f64), Gear)> = {
        let user = user.to_owned();
        let data = data.clone();

        block(move || data.as_ref().gear.iter(&user))
    }
    .await?
    .map(|x| (data.as_ref().activities.gear_totals(&user, &x.name), x))
    .collect();

    GearIndexTemplate {
        url: UrlFor::new(&id, req)?,
        id,
        gears,
        user: &user,
        title: "Gear",
    }
    .into_response()
}

#[derive(Template)]
#[template(path = "gear/add.html")]
struct GearAddTemplate<'a> {
    url: UrlFor,
    id: Identity,
    user: &'a str,
    title: &'a str,
}

pub async fn gear_add(req: HttpRequest, id: Identity, user: web::Path<String>) -> impl Responder {
    GearAddTemplate {
        url: UrlFor::new(&id, req)?,
        id,
        user: &user,
        title: "Add new gear",
    }
    .into_response()
}
