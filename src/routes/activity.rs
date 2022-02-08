use crate::{
    cache::ThumbnailCache,
    error::{Error, Result},
};
use axum::{
    extract::{Extension, Path, Query, TypedHeader},
    headers::{ContentType, ETag, HeaderMapExt, IfNoneMatch},
    http::{self, HeaderMap, HeaderValue, StatusCode},
    response::{Headers, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::str::FromStr;
use tf_database::{
    query::{ActivityQuery, UserQuery, GearQuery, Key},
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
        .route("/:id/thumbnail", get(get_activity_thumbnail))
}

async fn get_activity_thumbnail(
    Extension(db): Extension<Database<'_>>,
    Extension(cache): Extension<ThumbnailCache>,
    Path(query): Path<ActivityQuery<'_>>,
    header: Option<TypedHeader<IfNoneMatch>>,
) -> Result<impl IntoResponse> {
    let record = db.activity.record.get(&query)?.ok_or(Error::NotFound)?;
    let thumbnail = cache.get(query.as_key(), record)
        .await
        .ok_or(Error::NotFound)?;

    let etag = ETag::from_str(&format!(r#""{:#x}""#, thumbnail.crc)).unwrap();

    if header
        .map(|TypedHeader(header)| !header.precondition_passes(&etag))
        .unwrap_or_default()
    {
        return Ok(StatusCode::NOT_MODIFIED.into_response());
    }

    let mut headers = HeaderMap::new();

    headers.typed_insert(etag);
    headers.typed_insert(ContentType::png());

    Ok((headers, thumbnail.data).into_response())
}

#[oauth(scopes = ["activity:read"])]
pub async fn get_activity(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .get(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
pub async fn activity_session(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .session
        .get(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
pub async fn activity_record(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .record
        .get(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
async fn activity_lap(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .lap
        .get(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
async fn get_activity_gear(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .gear
        .get(&query)?
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:write"])]
async fn put_activity_gear(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
    Json(gear_id): Json<String>,
) -> Result<impl IntoResponse> {
    let foreign_key = GearQuery {
        user_id: std::borrow::Cow::Borrowed(&query.user_id),
        id: gear_id.into(),
    };

    db.activity
        .gear
        .insert(&query, &foreign_key)?
        .map(|x| StatusCode::NO_CONTENT)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:write"])]
async fn delete_activity_gear(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    //db.activity.insert_gear(&query, None)?;

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
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.activity
        .record
        .get(&query)?
        .map(|x| super::utils::zone_duration(&x, 50, 205))
        .map(Json)
        .ok_or(Error::NotFound)
}

#[oauth(scopes = ["activity:read"])]
async fn get_activity_prev(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    //db.activity.prev(&query)?.map(Json).ok_or(Error::NotFound)
    Ok("")
}

#[oauth(scopes = ["activity:read"])]
async fn get_activity_next(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    //db.activity.next(&query)?.map(Json).ok_or(Error::NotFound)
    Ok("")
}

#[oauth(scopes = ["activity:write"])]
async fn delete_activity(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<ActivityQuery<'_>>,
) -> Result<impl IntoResponse> {
    //db.activity.remove_activity(&query)?;

    Ok(StatusCode::NO_CONTENT)
}

#[oauth(scopes = ["activity:read"])]
async fn get_activity_index(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<UserQuery>,
    Query(filters): Query<Filters>,
) -> Result<impl IntoResponse> {
    /*
    let sessions: Vec<_> = db
        .activity
        .username_iter_session(&query)?
        .skip(filters.skip)
        .take(filters.take)
        .collect();

    Ok(Json(sessions))
    */
    Ok("")
}

#[oauth(scopes = ["activity:write"])]
async fn post_activity_index(
    Extension(db): Extension<Database<'_>>,
    Path(query): Path<UserQuery>,
    file: bytes::Bytes,
) -> Result<impl IntoResponse> {
    //let gear = db.user.get_standard_gear(&query)?;

    let parsed = tf_parse::parse(&file, Default::default())?;
    db.activity.insert(&parsed)?;

    let activity_query = ActivityQuery::from(&parsed);

    let url = format!(
        "/user/{}/activity/{}",
        activity_query.user_id, activity_query.id
    );

    Ok(Headers(vec![(
        http::header::LOCATION,
        HeaderValue::from_str(&url).unwrap(),
    )]))
}
