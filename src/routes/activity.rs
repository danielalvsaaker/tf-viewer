use crate::{
    cache::ThumbnailCache,
    error::{Error, Result},
    state::AppState,
};
use axum::{
    extract::{Path, State, TypedHeader},
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
    resource::index::DefaultGear,
    Database,
};
use tf_models::{
    activity::{Lap, Record, Session},
    gear::Gear,
    user::User,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(post_activity_index))
        .route("/:id/thumbnail", get(get_activity_thumbnail))
}

async fn get_activity_thumbnail(
    _: Grant<Read<Activity>>,
    State(db): State<Database>,
    State(cache): State<ThumbnailCache>,
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
    State(db): State<Database>,
    Path(query): Path<UserQuery>,
    file: bytes::Bytes,
) -> Result<impl IntoResponse> {
    let task = async move {
        let (send, recv) = tokio::sync::oneshot::channel();

        rayon::spawn(move || {
            let _ = send.send(tf_parse::parse(&file));
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

        if let Some(default_gear) = root.traverse::<DefaultGear>()?.key(&query)? {
            root.traverse::<Session>()?
                .traverse::<Gear>(&activity_query)?
                .link(&activity_query, &default_gear)?;
        }

        Ok::<_, tf_database::error::Error>(())
    })
    .await??;

    Ok(Json(activity_query))
}
