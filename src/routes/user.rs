use crate::error::{Error, Result};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use tf_database::{query::UserQuery, Database};
use tf_macro::oauth;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_authenticated_user))
        .route("/:user_id", get(get_user))
}

#[oauth("user:read")]
async fn get_authenticated_user() -> impl IntoResponse {
    Json(grant.owner_id)
}

#[oauth("user:read")]
async fn get_user(
    Extension(db): Extension<Database>,
    Path(query): Path<UserQuery<'_>>,
) -> Result<impl IntoResponse> {
    db.user.get_user(&query)?.map(Json).ok_or(Error::NotFound)
}

/*
#[protect]
pub async fn post_user(
    db: web::Data<Database>,
    user: web::Json<User>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let id = nanoid::nanoid!(10);
    let query = UserQuery::from(id.as_str());

    db.user.insert_user(&query, user.deref())?;

    let url = req.url_for("user", &[query.user_id]).unwrap();

    Ok(HttpResponse::Created()
        .insert_header((http::header::LOCATION, url.to_string()))
        .finish())
}

#[protect]
async fn put_user(
    db: web::Data<Database>,
    query: web::Path<UserQuery<'_>>,
    user: web::Json<User>,
) -> Result<impl Responder> {
    Ok(match db.user.insert_user(&query, user.deref())? {
        Some(_) => HttpResponse::NoContent(),
        None => HttpResponse::Created(),
    }
    .finish())
}
*/
