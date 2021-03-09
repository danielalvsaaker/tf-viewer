use super::UrlFor;
use crate::models::{DisplayUnit, Duration, Gear, GearType, GetWithUnit, Unit};
use actix_identity::Identity;
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use serde::Deserialize;
use std::str::FromStr;
use uom::si::{
    f64::Length,
    length::{kilometer, mile},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{username}/gear")
            .name("gear_index")
            .to(gear_index),
    )
    .service(
        web::resource("/{username}/gear/add")
            .name("gear_add")
            .wrap(crate::middleware::Restricted)
            .route(web::get().to(gear_add))
            .route(web::post().to(gear_add_post)),
    )
    .service(
        web::resource("/{username}/gear/{gear}")
            .name("gear_settings")
            .wrap(crate::middleware::Restricted)
            .route(web::get().to(gear_settings))
            .route(web::post().to(gear_settings_post)),
    );
}

#[derive(Template)]
#[template(path = "gear/settings.html")]
struct GearSettingsTemplate<'a> {
    url: UrlFor,
    id: Identity,
    unit: &'a Unit,
    gear: &'a Gear,
    title: &'a str,
    message: Option<&'a str>,
}

async fn gear_settings(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    web::Path((username, gear_name)): web::Path<(String, String)>,
    unit: web::Data<Unit>,
) -> impl Responder {
    let gear = data.gear.get(&username, &gear_name)?;

    GearSettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        unit: &unit,
        gear: &gear,
        title: &gear.name,
        message: None,
    }
    .into_response()
}

#[derive(Deserialize)]
struct GearForm {
    pub name: String,
    pub gear_type: String,
    #[serde(default)]
    pub fixed_distance: f64,
    #[serde(default)]
    pub standard: bool,
}

async fn gear_settings_post(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    web::Path((username, gear_name)): web::Path<(String, String)>,
    form: web::Form<GearForm>,
    unit: web::Data<Unit>,
) -> impl Responder {
    let gear_form = form.into_inner();
    let gear_type = GearType::from_str(&gear_form.gear_type);

    let result = {
        if gear_type.is_err() {
            Some("Wrong gear type specified.")
        } else if gear_name != gear_form.name {
            Some("Wrong gear name specified.")
        } else {
            None
        }
    };

    let fixed_distance = match unit.as_ref() {
        Unit::Metric => Length::new::<kilometer>(gear_form.fixed_distance),
        Unit::Imperial => Length::new::<mile>(gear_form.fixed_distance),
    };

    if result.is_none() {
        let gear = Gear {
            name: gear_form.name.clone(),
            gear_type: gear_type.unwrap(),
            fixed_distance,
        };

        if gear_form.standard {
            data.users.set_standard_gear(&username, &gear_form.name)?;
        }
        data.gear.insert(gear, &username)?;

        let url: UrlFor = UrlFor::new(&id, &req)?;
        return Ok(HttpResponse::Found()
            .header(http::header::LOCATION, url.gear_index.as_str())
            .finish()
            .into_body());
    }

    let gear = data.gear.get(&username, &gear_name)?;

    GearSettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        unit: &unit,
        gear: &gear,
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
    unit: &'a Unit,
    gears: &'a [(Length, Duration, Gear)],
    standard_gear: Option<&'a str>,
    username: &'a str,
    title: &'a str,
}

async fn gear_index(
    req: HttpRequest,
    id: Identity,
    username: web::Path<String>,
    data: web::Data<crate::Database>,
    unit: web::Data<Unit>,
) -> impl Responder {
    data.users.exists(&username)?;

    let gear_iter = data.gear.iter(&username)?;
    let standard_gear = data.users.get_standard_gear(&username)?;

    let gears: Vec<(Length, Duration, Gear)> = gear_iter
        .map(|x| {
            (
                data.activities
                    .gear_totals(&username, &x.name)
                    .unwrap_or_default(),
                x,
            )
        })
        .map(|((x, y), z)| (x, y, z))
        .collect();

    GearIndexTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        unit: &unit,
        gears: &gears,
        standard_gear: standard_gear.as_deref(),
        username: &username,
        title: "Gear",
    }
    .into_response()
}

#[derive(Template)]
#[template(path = "gear/add.html")]
struct GearAddTemplate<'a> {
    url: UrlFor,
    id: Identity,
    unit: &'a Unit,
    title: &'a str,
    message: Option<&'a str>,
}

async fn gear_add(req: HttpRequest, id: Identity, unit: web::Data<Unit>) -> impl Responder {
    GearAddTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        unit: &unit,
        title: "Add new gear",
        message: None,
    }
    .into_response()
}

async fn gear_add_post(
    req: HttpRequest,
    id: Identity,
    username: web::Path<String>,
    data: web::Data<crate::Database>,
    form: web::Form<GearForm>,
    unit: web::Data<Unit>,
) -> impl Responder {
    let username = username.into_inner();
    let gear_form = form.into_inner();
    let gear_type = GearType::from_str(&gear_form.gear_type);

    let result = {
        if gear_type.is_err() {
            Some("Wrong gear type specified.")
        } else if gear_form.name.is_empty() {
            Some("Gear name can not be empty.")
        } else if data.gear.exists(&username, &gear_form.name)? {
            Some("A gear with this name already exists.")
        } else {
            None
        }
    };

    if result.is_none() {
        let fixed_distance = match unit.as_ref() {
            Unit::Metric => Length::new::<kilometer>(gear_form.fixed_distance),
            Unit::Imperial => Length::new::<mile>(gear_form.fixed_distance),
        };

        let gear = Gear {
            name: gear_form.name.clone(),
            gear_type: gear_type?,
            fixed_distance,
        };

        if gear_form.standard {
            data.users.set_standard_gear(&username, &gear_form.name)?;
        }
        data.gear.insert(gear, &username)?;

        let url: UrlFor = UrlFor::new(&id, &req)?;

        return Ok(HttpResponse::Found()
            .header(http::header::LOCATION, url.gear_index.as_str())
            .finish()
            .into_body());
    }

    GearAddTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        unit: &unit,
        title: "Add new gear",
        message: result,
    }
    .into_response()
}
