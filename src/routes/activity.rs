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
        .route("/", get(get_activity_index).post(post_activity_index))
        .route("/:id", get(get_activity).delete(delete_activity))
        .route("/:id/session", get(activity_session))
        .route("/:id/record", get(activity_record))
        .route("/:id/lap", get(activity_lap))
        .route(
            "/:id/gear",
            get(get_activity_gear)
                .put(put_activity_gear)
                .delete(delete_activity_gear),
        )
        .route("/:id/zones", get(get_activity_zones))
        .route("/:id/prev", get(get_activity_prev))
        .route("/:id/next", get(get_activity_next))
}

#[oauth(scopes = ["activity:read"])]
pub async fn get_activity(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_activity(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
pub async fn activity_session(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_session(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
pub async fn activity_record(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_record(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
async fn activity_lap(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_lap(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
async fn get_activity_gear(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get_gear(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:write"])]
async fn put_activity_gear(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
    Json(gear_id): Json<String>,
) -> Result<impl IntoResponse> {
    if !db.gear.contains_gear(&(&query, &gear_id).into())? {
        return Ok(StatusCode::UNPROCESSABLE_ENTITY);
    }

    db.activity
        .insert_gear(&query, Some(&gear_id))?
        .map(|x| {
            x.map(|_| StatusCode::NO_CONTENT)
                .unwrap_or(StatusCode::CREATED)
        })
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:write"])]
async fn delete_activity_gear(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity.insert_gear(&query, None)?;

    Ok(StatusCode::NO_CONTENT)
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

#[oauth(scopes = ["activity:read"])]
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

#[oauth(scopes = ["activity:read"])]
async fn get_activity_prev(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity.prev(&query)?.map(Json).ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
async fn get_activity_next(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity.next(&query)?.map(Json).ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:write"])]
async fn delete_activity(
    Extension(db): Extension<Database>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity.remove_activity(&query)?;

    Ok(StatusCode::NO_CONTENT)
}

#[oauth(scopes = ["activity:read"])]
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

#[oauth(scopes = ["activity:write"])]
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
