use crate::error::{Error, Result};
use axum::{
    extract::{Extension, Path, Query},
    http::{self, HeaderValue, StatusCode},
    response::{Headers, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use tf_database::{
    query::{ActivityQuery, UserQuery},
    Database,
};
use tf_macro::oauth;

pub fn router() -> Router {
    Router::new()
        .route("/:id", get(get_activity))
        .route("/:id/session", get(activity_session))
        .route("/:id/record", get(activity_record))
}

#[oauth("activity:read")]
pub async fn get_activity(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_activity(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth("activity:read")]
pub async fn activity_session(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_session(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth("activity:read")]
pub async fn activity_record(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_record(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth("activity:read")]
async fn activity_lap(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_lap(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth("activity:read")]
async fn get_activity_gear(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_gear(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth("activity:write")]
async fn put_activity_gear(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
    Json(gear_id): Json<String>,
) -> Result<impl IntoResponse> {
    db.activity
        .insert_gear(&query, Some(&gear_id))?
        .map(|x| {
            x.map(|_| StatusCode::NO_CONTENT)
                .unwrap_or(StatusCode::CREATED)
        })
        .ok_or(Error::NotFound)
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

#[oauth("activity:read")]
async fn get_activity_zones(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_record(&query)?
        .map(|x| super::utils::zone_duration(&x, 50, 205))
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth("activity:read")]
async fn get_activity_prev(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity.prev(&query)?.map(Json).ok_or(Error::NotFound)
}

#[oauth("activity:read")]
async fn get_activity_next(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity.next(&query)?.map(Json).ok_or(Error::NotFound)
}

#[oauth("activity:write")]
async fn delete_activity(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity.remove_activity(&query)?;

    Ok(StatusCode::NO_CONTENT)
}

#[oauth("activity:read")]
async fn get_activity_index(
    Extension(db): Extension<Database>,
    Path(query): Path<UserQuery<'_>>,
    Query(filters): Query<Filters>,
) -> Result<impl IntoResponse> {
    let sessions: Vec<_> = db
        .activity
        .username_iter_session(&query)?
        .skip(filters.skip)
        .take(filters.take)
        .collect();

    Ok(Json(sessions))
}

#[oauth("activity:write")]
async fn post_activity_index(
    Extension(db): Extension<Database>,
    Path(query): Path<UserQuery<'_>>,
    file: bytes::Bytes,
) -> Result<impl IntoResponse> {
    let gear = db.user.get_standard_gear(&query)?;

    let parsed = tf_parse::parse(&file, gear)?;
    db.activity.insert_activity(&query, &parsed)?;

    let activity_query = ActivityQuery::from((&query, parsed.id.as_str()));
    let url = format!(
        "/user/{}/activity/{}",
        activity_query.user_id, activity_query.id
    );

    Ok(Headers(vec![(
        http::header::LOCATION,
        HeaderValue::from_str(&url).unwrap(),
    )]))
}
