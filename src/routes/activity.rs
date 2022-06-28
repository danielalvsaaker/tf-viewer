use crate::{
    cache::ThumbnailCache,
    error::{Error, Result},
};
use axum::{
    extract::{Extension, Path, TypedHeader},
    headers::{ContentType, ETag, HeaderMapExt, IfNoneMatch},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use std::str::FromStr;
use tf_auth::scopes::{Activity, Grant, Read, Write};
use tf_database::{
    primitives::Key,
    query::{ActivityQuery, UserQuery},
    Database,
};
use tf_models::{
    activity::{Lap, Record, Session},
    user::User,
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
    _grant: Grant<Write<Activity>>,
    Extension(db): Extension<Database>,
    Path(query): Path<UserQuery>,
    file: bytes::Bytes,
) -> Result<impl IntoResponse> {
    let task = async move {
        let (send, recv) = tokio::sync::oneshot::channel();

        rayon::spawn(move || {
            let _ = send.send(tf_parse::parse(query.user_id, &file));
        });

        recv.await
    };

    let parsed = task.await.unwrap()?;

    let activity_query = ActivityQuery {
        user_id: query.user_id,
        id: parsed.id,
    };

    tokio::task::spawn_blocking(move || {
        let root = db.root::<tf_models::user::User>()?;

        if !root.contains_key(&query)? {
            root.insert(
                &query,
                &tf_models::user::User {
                    name: query.user_id.as_str().into(),
                    ..Default::default()
                },
            )?;
        }

        root.traverse::<Session>()?
            .insert(&activity_query, &parsed.session, &query)?;

        root.traverse::<Record>()?
            .insert(&activity_query, &parsed.record, &query)?;

        root.traverse::<Vec<Lap>>()?
            .insert(&activity_query, &parsed.lap, &query)?;

        Ok::<_, tf_database::error::Error>(())
    })
    .await??;

    Ok(Json(activity_query))
}
