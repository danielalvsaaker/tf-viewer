use crate::error::{Error, Result};
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use std::ops::Deref;
use tf_database::{
    query::{ActivityQuery, UserQuery},
    Database,
};
use tf_models::{
    frontend::{Activity, Lap, Record, Session},
    Unit,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{user_id}/activity/{id}")
            .name("activity")
            .route(web::get().to(get_activity))
            .route(web::delete().to(delete_activity)),
    )
    .service(
        web::resource("/{user_id}/activity/{id}/session")
            .name("activity_session")
            .route(web::get().to(activity_session)),
    )
    .service(
        web::resource("/{user_id}/activity/{id}/record")
            .name("activity_record")
            .route(web::get().to(activity_record)),
    )
    .service(
        web::resource("/{user_id}/activity/{id}/lap")
            .name("activity_lap")
            .route(web::get().to(activity_lap)),
    )
    .service(
        web::resource("/{user_id}/activity/{id}/gear")
            .name("activity_gear")
            .route(web::get().to(get_activity_gear))
            .route(web::put().to(put_activity_gear)),
    )
    .service(
        web::resource("/{user_id}/activity/{id}/zones")
            .name("activity_zones")
            .route(web::get().to(get_activity_zones)),
    )
    .service(
        web::resource("/{user_id}/activity/{id}/prev")
            .name("activity_prev")
            .route(web::get().to(get_activity_prev)),
    )
    .service(
        web::resource("/{user_id}/activity/{id}/next")
            .name("activity_next")
            .route(web::get().to(get_activity_next)),
    )
    .service(
        web::resource("/{user_id}/activity")
            .name("activity_index")
            .route(web::get().to(get_activity_index))
            .route(web::post().to(post_activity_index)),
    );
}

async fn activity_session(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
    unit: Option<web::Query<Unit>>,
) -> Result<impl Responder> {
    let session = db.activity.get_session(&query)?.ok_or(Error::NotFound)?;

    Ok(web::Json(Session::from_backend(
        session,
        unit.as_deref().unwrap_or_default(),
    )))
}

async fn activity_record(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
    unit: Option<web::Query<Unit>>,
) -> Result<impl Responder> {
    let record = db.activity.get_record(&query)?.ok_or(Error::NotFound)?;

    Ok(web::Json(Record::from_backend(
        record,
        unit.as_deref().unwrap_or_default(),
    )))
}

async fn activity_lap(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
    unit: Option<web::Query<Unit>>,
) -> Result<impl Responder> {
    let unit = unit.as_deref().unwrap_or_default();

    let lap: Vec<Lap> = db
        .activity
        .get_lap(&query)?
        .ok_or(Error::NotFound)?
        .into_iter()
        .map(|x| Lap::from_backend(x, unit))
        .collect();

    Ok(web::Json(lap))
}

async fn get_activity_gear(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
) -> Result<impl Responder> {
    Ok(web::Json(db.activity.get_gear(&query)?))
}

async fn put_activity_gear(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
    gear_id: web::Json<String>,
) -> Result<impl Responder> {
    Ok(match db.activity.insert_gear(&query, Some(gear_id.deref()))? {
        Some(_) => HttpResponse::NoContent(),
        None => HttpResponse::Created(),
    }
    .finish())
}

#[derive(Deserialize)]
#[serde(default)]
struct Filters {
    skip: usize,
    take: usize,
    //sort_by: Sort,
}

impl Default for Filters {
    fn default() -> Self {
        Self { skip: 0, take: 25 }
    }
}

async fn get_activity(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
    unit: Option<web::Query<Unit>>,
) -> Result<impl Responder> {
    let id = query.id.to_string();

    let gear_id = db.activity.get_gear(&query)?;

    let session = db.activity.get_session(&query)?.ok_or(Error::NotFound)?;
    let record = db.activity.get_record(&query)?.ok_or(Error::NotFound)?;
    let lap = db.activity.get_lap(&query)?.ok_or(Error::NotFound)?;

    let activity = Activity::from_backend(
        id,
        gear_id,
        session,
        record,
        lap,
        unit.as_deref().unwrap_or_default(),
    );

    Ok(web::Json(activity))
}

async fn get_activity_zones(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
) -> Result<impl Responder> {
    let record = db.activity.get_record(&query)?.ok_or(Error::NotFound)?;

    let zones = super::utils::zone_duration(&record, 50, 205);

    Ok(web::Json(zones))
}

async fn get_activity_prev(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
) -> Result<impl Responder> {
    Ok(web::Json(db.activity.prev(&query)?))
}

async fn get_activity_next(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
) -> Result<impl Responder> {
    Ok(web::Json(db.activity.next(&query)?))
}

async fn delete_activity(
    db: web::Data<Database>,
    query: web::Path<ActivityQuery<'_>>,
) -> Result<impl Responder> {
    db.activity.remove_activity(&query)?;

    Ok(HttpResponse::NoContent())
}

async fn get_activity_index(
    db: web::Data<Database>,
    query: web::Path<UserQuery<'_>>,
    unit: Option<web::Query<Unit>>,
    filters: web::Query<Filters>,
) -> Result<impl Responder> {
    let ids = db
        .activity
        .username_iter_session(&query)?
        .skip(filters.skip)
        .take(filters.take);

    Ok(web::Json(
        ids.map(|x| Session::from_backend(x, unit.as_deref().unwrap_or_default()))
            .collect::<Vec<Session>>(),
    ))
}

async fn post_activity_index(
    db: web::Data<Database>,
    query: web::Path<UserQuery<'_>>,
    file: web::Bytes,
    req: HttpRequest,
) -> Result<impl Responder> {
    let gear = db.user.get_standard_gear(&query)?;

    let parsed = tf_parse::parse(&file, gear)?;
    db.activity.insert_activity(&query, &parsed)?;

    let activity_query = ActivityQuery::from((query.deref(), parsed.id.as_str()));

    let url = req
        .url_for("activity", &[&activity_query.user_id, &activity_query.id])
        .unwrap();

    Ok(HttpResponse::Created()
        .insert_header((http::header::LOCATION, url.to_string()))
        .finish())
}
