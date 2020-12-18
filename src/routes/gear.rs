use super::{error::ErrorTemplate, UrlFor};
use crate::{Duration, Gear};
use actix_identity::Identity;
use actix_web::http;
use actix_web::{web, web::block, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};

#[derive(Template)]
#[template(path = "gear/settings.html")]
struct GearSettingsTemplate<'a> {
    url: UrlFor,
    id: Identity,
    gear: &'a Gear,
    user: &'a str,
    title: &'a str,
    message: Option<&'a str>,
}

pub async fn gear_settings(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    web::Path((username, gear_name)): web::Path<(String, String)>,
) -> impl Responder {
    let gear = {
        let username = username.clone();
        block(move || data.as_ref().gear.get(&username, &gear_name))
    }
    .await?;

    GearSettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        gear: &gear,
        user: &username,
        title: &gear.name,
        message: None,
    }
    .into_response()
}

pub async fn gear_settings_post(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    web::Path((username, gear_name)): web::Path<(String, String)>,
    form: web::Form<Gear>,
) -> impl Responder {
    let gear_form = form.into_inner();

    let result = {
        let gear_kind = vec![
            "road bike",
            "hybrid bike",
            "time trial bike",
            "offroad bike",
            "running shoes",
        ];

        if !gear_kind.iter().any(|x| x == &gear_form.kind) {
            Some("Wrong gear kind specified.")
        } else if gear_name != gear_form.name {
            Some("Wrong gear name specified.")
        } else {
            None
        }
    };

    if result.is_none() {
        let username = username.clone();
        block(move || {
            if gear_form.standard {
                data.as_ref()
                    .users
                    .set_standard_gear(&username, &gear_form.name);
            }
            data.as_ref().gear.insert(gear_form, &username)
        })
        .await;

        let url: UrlFor = UrlFor::new(&id, &req)?;
        return Ok(HttpResponse::Found()
            .header(http::header::LOCATION, url.gear_index.as_str())
            .finish()
            .into_body());
    }

    let gear = {
        let username = username.clone();
        block(move || data.as_ref().gear.get(&username, &gear_name))
    }
    .await?;

    GearSettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        gear: &gear,
        user: &username,
        title: &gear.name,
        message: result,
    }
    .into_response()
}

#[derive(Template)]
#[template(path = "gear/index.html")]
struct GearIndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    gears: Vec<((f64, Duration), Gear)>,
    standard_gear: Option<String>,
    user: &'a str,
    title: &'a str,
}

pub async fn gear_index(
    req: HttpRequest,
    id: Identity,
    user: web::Path<String>,
    data: web::Data<crate::Database>,
) -> impl Responder {
    let gear_iter = {
        let (user, data) = (user.clone(), data.clone());

        block(move || data.as_ref().gear.iter(&user))
    }
    .await?;

    let standard_gear = {
        let (user, data) = (user.clone(), data.clone());

        block(move || data.as_ref().users.get_standard_gear(&user))
    }
    .await?;

    let gears: Vec<((f64, Duration), Gear)> = {
        let user = user.clone();
        block::<_, _, actix_web::error::BlockingError<std::io::Error>>(move || {
            Ok(gear_iter
                .map(|x| (data.as_ref().activities.gear_totals(&user, &x.name), x))
                .collect::<Vec<((f64, Duration), Gear)>>())
        })
        .await?
    };

    GearIndexTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        gears,
        standard_gear,
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
    message: Option<&'a str>,
}

pub async fn gear_add(req: HttpRequest, id: Identity, user: web::Path<String>) -> impl Responder {
    GearAddTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        user: &user,
        title: "Add new gear",
        message: None,
    }
    .into_response()
}

pub async fn gear_add_post(
    req: HttpRequest,
    id: Identity,
    user: web::Path<String>,
    data: web::Data<crate::Database>,
    form: web::Form<Gear>,
) -> impl Responder {
    let user = user.into_inner();

    let result = {
        let gear_kind = vec![
            "road bike",
            "hybrid bike",
            "time trial bike",
            "offroad bike",
            "running shoes",
        ];

        if !gear_kind.iter().any(|x| x == &form.kind) {
            Some("Wrong gear kind specified.")
        } else {
            None
        }
    };

    if result.is_none() {
        let gear = form.into_inner();
        web::block(move || {
            if gear.standard {
                data.as_ref().users.set_standard_gear(&user, &gear.name);
            }
            data.as_ref().gear.insert(gear, &user)
        })
        .await;

        let url: UrlFor = UrlFor::new(&id, &req)?;

        return Ok(HttpResponse::Found()
            .header(http::header::LOCATION, url.gear_index.as_str())
            .finish()
            .into_body());
    }

    GearAddTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        user: &user,
        title: "Add new gear",
        message: result,
    }
    .into_response()
}
