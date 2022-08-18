use super::{Callback, UserForm};
use crate::{
    database::{
        resource::user::{User, Username},
        Database,
    },
    error::Result,
    templates::SignIn,
};

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{Extension, Form, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_sessions::extractors::WritableSession;

pub fn routes() -> Router {
    Router::new().route("/", get(get_signin).post(post_signin))
}

async fn get_signin(query: Option<Query<Callback<'_>>>) -> impl IntoResponse {
    let query = &query
        .as_ref()
        .map(|Query(x)| serde_urlencoded::to_string(x).unwrap())
        .unwrap_or_default();
    SignIn { query }.into_response()
}

async fn post_signin(
    Extension(db): Extension<Database>,
    query: Option<Query<Callback<'_>>>,
    Form(user): Form<UserForm>,
    mut session: WritableSession,
) -> Result<impl IntoResponse> {
    let query = query.as_ref().map(|x| x.as_str());

    let hash = db
        .root::<Username>()?
        .traverse::<User>()?
        .get(&user.username)?
        .map(|x| x.password)
        .unwrap_or_default();

    let authorized = PasswordHash::new(&hash)
        .map(|x| {
            Argon2::default()
                .verify_password(user.password.as_bytes(), &x)
                .is_ok()
        })
        .unwrap_or_default();

    if authorized {
        let user_id = db
            .root::<Username>()?
            .traverse::<User>()?
            .key(&user.username)?
            .unwrap();
        session.insert("id", user_id).unwrap();
    } else {
        return Ok((
            StatusCode::UNAUTHORIZED,
            SignIn {
                query: query.unwrap_or_default(),
            },
        )
            .into_response());
    }

    if let Some(query) = query {
        Ok(Redirect::to(query).into_response())
    } else {
        Ok(Redirect::to("/oauth/").into_response())
    }
}
