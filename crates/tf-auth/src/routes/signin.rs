use super::{Callback, UserForm};
use crate::{
    database::{
        resource::user::{User, Username},
        Database,
    },
    error::Result,
    session::Session,
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
    session: Session,
    Extension(db): Extension<Database>,
    query: Option<Query<Callback<'_>>>,
    Form(user): Form<UserForm>,
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

    let cookie = if authorized {
        let user_id = db
            .root::<Username>()?
            .traverse::<User>()?
            .key(&user.username)?
            .unwrap();
        session.remember(user_id).await
    } else {
        return Ok((
            StatusCode::UNAUTHORIZED,
            SignIn {
                query: query.unwrap_or_default(),
            },
        )
            .into_response());
    };

    if let Some(query) = query {
        Ok((cookie, Redirect::to(query)).into_response())
    } else {
        Ok((cookie, Redirect::to("/oauth/")).into_response())
    }
}
