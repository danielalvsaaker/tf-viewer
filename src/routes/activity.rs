use super::{
    api::{ActivityData, DataRequest, DataResponse},
    error::ErrorTemplate,
    UrlActivity, UrlFor,
};
use crate::{ActivityType, Lap, Session};
use actix_identity::Identity;
use actix_web::{http, web, web::block, HttpRequest, HttpResponse, Responder};
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
    plot: &'a str,
    title: &'a str,
}

pub async fn activity(
    req: HttpRequest,
    data: web::Data<crate::Database>,
    id: Identity,
    web::Path((username, activity_id)): web::Path<(String, String)>,
) -> impl Responder {
    let activity = {
        let username = username.clone();
        web::block(move || {
            data.as_ref()
                .activities
                .get_activity(&username, &activity_id)
        })
    }
    .await?;

    let plot = {
        let record = activity.record.clone();
        web::block(move || super::utils::plot(&record)).await?
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
    username: &'a str,
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
    let activity = {
        let (username, data) = (username.clone(), data.clone());
        web::block(move || {
            data.as_ref()
                .activities
                .get_activity(&username, &activity_id)
        })
    }
    .await?;

    let gear_iter = {
        let (username, data) = (username.clone(), data.clone());
        web::block(move || data.as_ref().gear.iter(&username))
    }
    .await?
    .map(|x| x.name);

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
        username: &username,
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
    pub gear_id: String,
}

pub async fn activity_settings_post(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    web::Path((username, activity_id)): web::Path<(String, String)>,
    form: web::Form<ActivitySettingsForm>,
) -> impl Responder {
    let form = form.into_inner();

    let mut activity = {
        let (username, activity_id, data) = (username.clone(), activity_id.clone(), data.clone());
        web::block(move || {
            data.as_ref()
                .activities
                .get_activity(&username, &activity_id)
        })
    }
    .await?;

    let gear_iter = {
        let (username, data) = (username.clone(), data.clone());
        web::block(move || data.as_ref().gear.iter(&username))
    }
    .await?
    .map(|x| x.name);

    let mut gears: Vec<String> = Vec::new();
    if let Some(gear_id) = activity.gear_id {
        gears = gear_iter.filter(|x| x != &gear_id).collect();
        gears.insert(0, gear_id);
    } else {
        gears = gear_iter.collect();
    }

    let result = {
        if !gears.iter().any(|x| x == &form.gear_id) {
            Some("The specified gear does not exist.")
        } else {
            None
        }
    };

    if result.is_none() {
        activity.session.activity_type =
            ActivityType::from_str(&form.activity_type).unwrap_or_default();
        activity.gear_id = Some(form.gear_id);

        {
            let username = username.clone();
            web::block(move || data.as_ref().activities.insert(activity, &username))
        }
        .await?;

        let url: UrlActivity = UrlActivity::new(&username, &activity_id, &req)?;

        return Ok(HttpResponse::Found()
            .header(http::header::LOCATION, url.url.as_str())
            .finish()
            .into_body());
    }

    ActivitySettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        username: &username,
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
    user: &'a str,
    title: &'a str,
}

pub async fn activity_index(
    req: HttpRequest,
    id: Identity,
    user: web::Path<String>,
) -> impl Responder {
    ActivityIndexTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        user: &user,
        title: "Activities",
    }
    .into_response()
}

pub async fn activity_index_post(
    request: web::Json<DataRequest>,
    data: web::Data<crate::Database>,
    user: web::Path<String>,
) -> impl Responder {
    let iter = {
        let data = data.clone();
        let user = user.to_owned();

        web::block(move || data.as_ref().activities.username_iter_session(&user))
    }
    .await
    .unwrap();

    let id = web::block(move || data.as_ref().activities.username_iter_id(&user))
        .await
        .unwrap();

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
