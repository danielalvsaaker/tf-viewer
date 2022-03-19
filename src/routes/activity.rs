use crate::{
    cache::ThumbnailCache,
    error::{Error, Result},
};
use axum::{
    extract::{Extension, Path, TypedHeader},
    headers::{ContentType, ETag, HeaderMapExt, IfNoneMatch},
    http::{self, HeaderMap, HeaderValue, StatusCode},
    response::{Headers, IntoResponse},
    routing::{get, post},
    Router,
};
use std::str::FromStr;
use tf_auth::scopes::{Activity, Grant, Read, Write};
use tf_database::{
    Database, primitives::Key, query::{ActivityQuery, UserQuery},
};
use tf_models::{
    user::User, activity::{Session, Record, Lap},
};

pub fn router() -> Router {
    Router::new()
        .route("/", post(post_activity_index))
        .route("/:id/thumbnail", get(get_activity_thumbnail))
}

async fn get_activity_thumbnail(
    _: Grant<Read<Activity>>,
    Extension(db): Extension<Database>,
    Extension(cache): Extension<ThumbnailCache>,
    Path(query): Path<ActivityQuery>,
    header: Option<TypedHeader<IfNoneMatch>>,
) -> Result<impl IntoResponse> {
    let record = db
        .root::<User>()?
        .traverse::<Record>()?
        .get(&query)?
        .ok_or(Error::NotFound)?;
    let thumbnail = cache
        .get(query.as_key(), record)
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

async fn post_activity_index(
    //_grant: Grant<Write<Activity>>,
    Extension(db): Extension<Database>,
    Path(query): Path<UserQuery>,
    file: bytes::Bytes,
) -> Result<impl IntoResponse> {

    let parsed = tf_parse::parse(query.user_id, &file)?;

    let activity_query = ActivityQuery {
        user_id: query.user_id,
        id: parsed.id,
    };

    let root = db
        .root()?;

    root.insert(&query, &tf_models::user::User { heartrate_rest: 50, heartrate_max: 205, name: "daniel".into(), })?;

    root.traverse::<Session>()?
        .insert(&activity_query, &parsed.session, &query)?;

    root.traverse::<Record>()?
        .insert(&activity_query, &parsed.record, &query)?;

    root.traverse::<Vec<Lap>>()?
        .insert(&activity_query, &parsed.lap, &query)?;

    let url = format!("/user/{}/activity/{}", activity_query.user_id, activity_query.id,);

    Ok(Headers(vec![(
        http::header::LOCATION,
        HeaderValue::from_str(&url).unwrap(),
    )]))
}
