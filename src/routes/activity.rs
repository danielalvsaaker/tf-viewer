use actix_web::{Responder, web, HttpRequest, HttpResponse};
use actix_identity::Identity;
use askama_actix::{Template, TemplateIntoResponse};
use super::{UrlFor, FormatDuration,
           api::{DataRequest, DataResponse, ActivityData},
           error::ErrorTemplate
};
use crate::{Session, Lap};

#[derive(Template)]
#[template(path = "activity/activity.html")]
struct ActivityTemplate<'a> {
    url: UrlFor,
    id: Identity,
    user: &'a str,
    session: &'a Session,
    laps: &'a Vec<Lap>,
    coords: &'a Vec<(Option<f64>, Option<f64>)>,
    plot: &'a str,
    title: &'a str,
}

pub async fn activity(
    req: HttpRequest,
    data: web::Data<crate::Database>,
    id: Identity,
    web::Path((user, activity_id)): web::Path<(String, String)>,
    ) -> impl Responder {


    if !data.as_ref().activities.exists(&user, &activity_id).unwrap() {
        return ErrorTemplate::not_found(req, id).await
    }

    let activity = data.as_ref().activities.get_activity(&user, &activity_id).unwrap();

    let plot = {
        let record = activity.record.clone();
        web::block(move || super::utils::plot(&record)).await.unwrap()
    };

    ActivityTemplate {
        url: UrlFor::new(&id, req),
        id,
        user: &user,
        session: &activity.session,
        laps: &activity.lap,
        coords: &activity.record.lon.into_iter().zip(activity.record.lat).collect(),
        plot: &plot,
        title: "Activity",
    }.into_response()
}


#[derive(Template)]
#[template(path = "activity/activityindex.html")]
struct ActivityIndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    user: &'a str,
    title: &'a str,
}

pub async fn activityindex(
    req: HttpRequest,
    id: Identity,
    user: web::Path<String>
    ) -> impl Responder {

    ActivityIndexTemplate {
        url: UrlFor::new(&id, req),
        id,
        user: &user,
        title: "Activities",
    }.into_response()
}

pub async fn activityindex_post(
    request: web::Json<DataRequest>,
    data: web::Data<crate::Database>,
    user: web::Path<String>
    ) -> impl Responder {

    let iter = data.as_ref().activities.iter_session(&user.to_owned())
        .unwrap();

    let id = data.as_ref().activities.iter_id(&user).unwrap();
   
    let mut sessions: Vec<ActivityData> = iter
        .zip(id)
        .map(|(x,y)| -> ActivityData {  ActivityData {
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
        }})
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


    web::Json(
        DataResponse {
            draw: request.draw,
            recordsTotal: amount,
            recordsFiltered: amount,
            data: results,
        }
    )
}
