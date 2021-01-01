use super::{
    api::{ActivityData, DataRequest, DataResponse},
    UrlActivity, UrlFor,
};
use crate::{ActivityType, Lap, Session};
use actix_identity::Identity;
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Template)]
#[template(path = "activity/activity.html")]
struct ActivityTemplate<'a> {
    url: UrlFor,
    id: Identity,
    activity_url: &'a str,
    username: &'a str,
    gear: Option<String>,
    session: &'a Session,
    laps: &'a Vec<Lap>,
    coords: &'a Vec<(f64, f64)>,
    zones: &'a Option<Vec<crate::Duration>>,
    plot: &'a str,
    title: &'a str,
}

pub async fn activity(
    req: HttpRequest,
    data: web::Data<crate::Database>,
    id: Identity,
    web::Path((username, activity_id)): web::Path<(String, String)>,
) -> impl Responder {
    let activity = data
        .activities
        .get_activity(&username, &activity_id)?;

    let plot = super::utils::plot(&activity.record)?;

    let zones = {
        let user = data.users.get_heartrate(&username)?;
        super::utils::zone_duration(&activity.record, &user)?
    };

    ActivityTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        activity_url: &req.path(),
        username: &username,
        gear: activity.gear_id,
        session: &activity.session,
        laps: &activity.lap,
        coords: &activity
            .record
            .lon
            .into_iter()
            .flatten()
            .zip(activity.record.lat.into_iter().flatten())
            .collect(),
        zones: &zones,
        plot: &plot,
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
    gears: &'a Vec<String>,
    title: &'a str,
    message: Option<&'a str>,
}

pub async fn activity_settings(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    web::Path((username, activity_id)): web::Path<(String, String)>,
) -> impl Responder {
    let activity = data
        .activities
        .get_activity(&username, &activity_id)?;

    let gear_iter = data.gear.iter(&username)?.map(|x| x.name);

    #[allow(unused_assignments)]
    let mut gears: Vec<String> = Vec::new();
    if let Some(gear_id) = activity.gear_id {
        gears = gear_iter.filter(|x| x != &gear_id).collect();
        gears.insert(0, gear_id);
    } else {
        gears = gear_iter.collect();
    }

    ActivitySettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        activity_type: &activity.session.activity_type,
        gears: &gears,
        title: "Settings",
        message: None,
    }
    .into_response()
}

#[derive(Deserialize)]
pub struct ActivitySettingsForm {
    pub activity_type: String,
    pub gear_id: Option<String>,
}

pub async fn activity_settings_post(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    web::Path((username, activity_id)): web::Path<(String, String)>,
    form: web::Form<ActivitySettingsForm>,
) -> impl Responder {
    let form = form.into_inner();

    let mut activity = data
        .activities
        .get_activity(&username, &activity_id)?;

    let gear_iter = data.gear.iter(&username)?.map(|x| x.name);

    #[allow(unused_assignments)]
    let mut gears: Vec<String> = Vec::new();
    if let Some(gear_id) = activity.gear_id {
        gears = gear_iter.filter(|x| x != &gear_id).collect();
        gears.insert(0, gear_id);
    } else {
        gears = gear_iter.collect();
    }

    let result = {
        if let Some(ref x) = form.gear_id {
            if !gears.iter().any(|y| y == x) {
                Some("The specified gear does not exist.")
            } 
            else {
                None
            }
        } else {
            None
        }
    };

    if result.is_none() {
        activity.session.activity_type =
            ActivityType::from_str(&form.activity_type).unwrap_or_default();
        activity.gear_id = form.gear_id;

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

pub async fn activity_index(
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

pub async fn activity_index_post(
    request: web::Json<DataRequest>,
    username: web::Path<String>,
    data: web::Data<crate::Database>,
) -> impl Responder {
    let iter = data.activities.username_iter_session(&username).unwrap();
    let id = data.activities.username_iter_id(&username).unwrap();

    let mut sessions: Vec<ActivityData> = iter
        .zip(id)
        .map(|(x, y)| -> ActivityData {
            ActivityData {
                date: x.start_time.0,
                activity_type: x.activity_type,
                duration: x.duration_active.to_string(),
                distance: x.distance,
                calories: x.calories,
                cadence_avg: x.cadence_avg,
                heartrate_avg: x.heartrate_avg,
                heartrate_max: x.heartrate_max,
                speed_avg: x.speed_avg,
                speed_max: x.speed_max,
                ascent: x.ascent,
                descent: x.descent,
                id: y,
            }
        })
        .collect();

    let amount = sessions.len();

    match request.column {
        0 => sessions.sort_by_key(|k| std::cmp::Reverse(k.date)),
        2 => sessions.sort_by(|a, b| a.duration.partial_cmp(&b.duration).unwrap()),
        3 => sessions.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap()),
        4 => sessions.sort_by_key(|k| k.calories),
        5 => sessions.sort_by_key(|k| k.cadence_avg),
        6 => sessions.sort_by_key(|k| k.heartrate_avg),
        7 => sessions.sort_by_key(|k| k.heartrate_max),
        8 => sessions.sort_by(|a, b| a.speed_avg.partial_cmp(&b.speed_avg).unwrap()),
        9 => sessions.sort_by(|a, b| a.speed_max.partial_cmp(&b.speed_max).unwrap()),
        10 => sessions.sort_by_key(|k| k.ascent),
        11 => sessions.sort_by_key(|k| k.descent),
        _ => (),
    };

    if request.dir.as_str() == "asc" {
        sessions.reverse();
    }

    let results: Vec<ActivityData> = sessions
        .into_iter()
        .skip(request.start)
        .take(request.length)
        .collect();

    web::Json(DataResponse {
        draw: request.draw,
        records_total: amount,
        records_filtered: amount,
        data: results,
    })
}
