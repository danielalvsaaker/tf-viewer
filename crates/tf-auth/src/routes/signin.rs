use super::{Callback, UserForm};
use crate::error::Result;
use crate::templates::SignIn;

use axum::{
    extract::{Extension, Form, Query},
    http::{StatusCode, Uri},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/", get(get_signin).post(post_signin))
}

async fn get_signin(query: Option<Query<Callback<'_>>>) -> impl IntoResponse {
    let query = query.as_ref().map(|x| x.as_str()).unwrap_or_default();
    SignIn { query }.into_response()
}

async fn post_signin(
    session: crate::session::Session,
    Extension(_db): Extension<crate::database::Database>,
    query: Option<Query<Callback<'_>>>,
    Form(user): Form<UserForm>,
) -> Result<impl IntoResponse> {
    let query = query.as_ref().map(|x| x.as_str());
    /*
    let authorized = db.get(&user.username)?
        .as_deref()
        .map(PasswordHash::new)
        .transpose()?
        .map(|x| {
            Argon2::default()
             .verify_password(user.password.as_bytes(), &x)
             .is_ok()
        })
        .unwrap_or_default();
        */

    let cookie = if true {
        session.remember(user.username).await
    } else {
        return Ok((
            StatusCode::UNAUTHORIZED,
            SignIn { query: query.unwrap_or_default() }
        )
            .into_response());
    };

    if let Some(query) = query {
        Ok((cookie, Redirect::to(query.parse().unwrap_or_default())).into_response())
    } else {
        Ok((cookie, Redirect::to(Uri::from_static("/oauth/"))).into_response())
    }
}
