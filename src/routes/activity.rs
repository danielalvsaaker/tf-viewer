use super::{
    api::{ActivityData, DataRequest, DataResponse},
    UrlActivity, UrlFor,
};
use crate::{
    middleware::Restricted,
    models::{ActivityType, DisplayUnit, Duration, Lap, Session, Unit},
};
use actix_identity::Identity;
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use serde::Deserialize;
use std::str::FromStr;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{username}/activity")
            .name("activity_index")
            .route(web::get().to(activity_index))
            .route(web::post().to(activity_index_post)),
    )
    .service(
        web::resource("/{username}/activity/{activity}")
            .name("activity")
            .to(activity),
    )
    .service(
        web::resource("/{username}/activity/{activity}/settings")
            .name("activity_settings")
            .wrap(Restricted)
            .route(web::get().to(activity_settings))
            .route(web::post().to(activity_settings_post)),
    );
}

#[derive(Template)]
#[template(path = "activity/activity.html")]
struct ActivityTemplate<'a> {
    url: UrlFor,
    id: Identity,
    unit: &'a Unit,
    activity_url: &'a str,
    prev: Option<UrlActivity>,
    next: Option<UrlActivity>,
    username: &'a str,
    gear: Option<&'a str>,
    session: &'a Session,
    laps: &'a [Lap],
    coords: &'a [(f64, f64)],
    zones: Option<&'a [Duration]>,
    notes: Option<&'a str>,
    plot: &'a str,
    title: &'a str,
}

async fn activity(
    req: HttpRequest,
    data: web::Data<crate::Database>,
    id: Identity,
    web::Path((username, activity_id)): web::Path<(String, String)>,
    unit: web::Data<Unit>,
) -> impl Responder {
    let activity = data.activities.get_activity(&username, &activity_id)?;

    let plot = super::utils::plot(&activity.record, &unit)?;

    let zones = {
        let user = data.users.get_heartrate(&username)?;
        super::utils::zone_duration(&activity.record, &user)?
    };

    let set: std::collections::BTreeSet<String> =
        data.activities.username_iter_id(&username)?.collect();

    let prev = set
        .range(..activity_id.clone())
        .next_back()
        .map(|x| UrlActivity::new(&username, x, &req).ok())
        .flatten();

    let next = set
        .range((
            std::ops::Bound::Excluded(activity_id),
            std::ops::Bound::Unbounded,
        ))
        .next()
        .map(|x| UrlActivity::new(&username, x, &req).ok())
        .flatten();

    ActivityTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        unit: &unit,
        activity_url: &req.path(),
        prev,
        next,
        username: &username,
        gear: activity.gear_id.as_deref(),
        session: &activity.session,
        laps: &activity.lap,
        coords: &activity
            .record
            .lon
            .into_iter()
            .flatten()
            .zip(activity.record.lat.into_iter().flatten())
            .collect::<Vec<(f64, f64)>>(),
        zones: zones.as_deref(),
        plot: &plot,
        notes: activity.notes.as_deref(),
        title: &format!("Activity {}", &activity.session.start_time),
    }
    .into_response()
}

#[derive(Template)]
#[template(path = "activity/settings.html")]
struct ActivitySettingsTemplate<'a> {
    url: UrlFor,
    id: Identity,
    activity_type: &'a ActivityType,
    gears: &'a [String],
    notes: Option<&'a str>,
    title: &'a str,
    message: Option<&'a str>,
}

async fn activity_settings(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    web::Path((username, activity_id)): web::Path<(String, String)>,
) -> impl Responder {
    let activity = data.activities.get_activity(&username, &activity_id)?;

    let mut gears: Vec<String> = data.gear.iter(&username)?.map(|x| x.name).collect();
    gears.sort_by_key(|k| Some(k) != activity.gear_id.as_ref());

    ActivitySettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        activity_type: &activity.session.activity_type,
        gears: &gears,
        notes: activity.notes.as_deref(),
        title: "Settings",
        message: None,
    }
    .into_response()
}

#[derive(Deserialize)]
struct ActivitySettingsForm {
    pub activity_type: String,
    pub gear_id: Option<String>,
    pub notes: String,
}

async fn activity_settings_post(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    web::Path((username, activity_id)): web::Path<(String, String)>,
    form: web::Form<ActivitySettingsForm>,
) -> impl Responder {
    let form = form.into_inner();

    let mut activity = data.activities.get_activity(&username, &activity_id)?;

    let mut gears: Vec<String> = data.gear.iter(&username)?.map(|x| x.name).collect();
    gears.sort_by_key(|k| Some(k) != form.gear_id.as_ref());

    let result = {
        if !gears.iter().any(|y| Some(y) == form.gear_id.as_ref()) && !gears.is_empty() {
            Some("The specified gear does not exist.")
        } else {
            None
        }
    };

    if result.is_none() {
        activity.session.activity_type =
            ActivityType::from_str(&form.activity_type).unwrap_or_default();
        activity.gear_id = form.gear_id;
        activity.notes = match form.notes.is_empty() {
            true => None,
            false => Some(form.notes),
        };

        data.activities.insert_or_overwrite(activity, &username)?;

        let url: UrlActivity = UrlActivity::new(&username, &activity_id, &req)?;

        return Ok(HttpResponse::Found()
            .header(http::header::LOCATION, url.url.as_str())
            .finish()
            .into_body());
    }

    ActivitySettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        activity_type: &activity.session.activity_type,
        gears: &gears,
        notes: activity.notes.as_deref(),
        title: "Settings",
        message: result,
    }
    .into_response()
}

#[derive(Template)]
#[template(path = "activity/index.html")]
struct ActivityIndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    username: &'a str,
    title: &'a str,
}

async fn activity_index(
    req: HttpRequest,
    id: Identity,
    username: web::Path<String>,
    data: web::Data<crate::Database>,
) -> impl Responder {
    data.users.exists(&username)?;

    ActivityIndexTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        username: &username,
        title: "Activities",
    }
    .into_response()
}

async fn activity_index_post(
    request: web::Json<DataRequest>,
    username: web::Path<String>,
    data: web::Data<crate::Database>,
    unit: web::Data<Unit>,
) -> impl Responder {
    let mut sessions: Vec<(Session, Option<String>)> = data
        .activities
        .username_iter_session(&username)
        .unwrap()
        .zip(data.activities.username_iter_gear(&username).unwrap())
        .collect();

    let amount = sessions.len();

    match request.column {
        0 => sessions.sort_by_key(|(k, _)| std::cmp::Reverse(k.start_time.0)),
        1 => sessions.sort_by_key(|(k, _)| k.activity_type.to_owned()),
        2 => sessions
            .sort_by(|(a, _), (b, _)| a.duration_active.partial_cmp(&b.duration_active).unwrap()),
        3 => sessions.sort_by(|(a, _), (b, _)| a.distance.partial_cmp(&b.distance).unwrap()),
        4 => sessions.sort_by_key(|(k, _)| k.calories),
        5 => sessions.sort_by_key(|(k, _)| k.cadence_avg),
        6 => sessions.sort_by_key(|(k, _)| k.heartrate_avg),
        7 => sessions.sort_by_key(|(k, _)| k.heartrate_max),
        8 => sessions.sort_by(|(a, _), (b, _)| a.speed_avg.partial_cmp(&b.speed_avg).unwrap()),
        9 => sessions.sort_by(|(a, _), (b, _)| a.speed_max.partial_cmp(&b.speed_max).unwrap()),
        10 => sessions.sort_by_key(|(k, _)| k.ascent),
        11 => sessions.sort_by_key(|(k, _)| k.descent),
        _ => (),
    };

    if request.dir.as_str() == "asc" {
        sessions.reverse();
    }

    let results: Vec<ActivityData> = sessions
        .iter()
        .skip(request.start)
        .take(request.length)
        .map(|(x, gear)| ActivityData {
            date: x.start_time.0,
            activity_type: x.activity_type.to_string(),
            duration: x.duration_active.to_string(),
            distance: x.distance.map(|x| x.display_km_mi(&unit)),
            calories: x.calories,
            cadence_avg: x.cadence_avg,
            heartrate_avg: x.heartrate_avg,
            heartrate_max: x.heartrate_max,
            speed_avg: x.speed_avg.map(|x| x.display_km_mi(&unit)),
            speed_max: x.speed_max.map(|x| x.display_km_mi(&unit)),
            ascent: x.ascent.map(|x| x.display_m_ft(&unit)),
            descent: x.descent.map(|x| x.display_m_ft(&unit)),
            gear: gear.to_owned(),
            id: x.start_time.0.format("%Y%m%d%H%M").to_string(),
        })
        .collect();

    web::Json(DataResponse {
        draw: request.draw,
        records_total: amount,
        records_filtered: amount,
        data: results,
    })
}
