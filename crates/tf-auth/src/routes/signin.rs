use super::{Callback, UserForm};
use crate::{
    database::{
        resource::user::{User, Username},
        Database,
    },
    error::Error,
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
        .and_then(|Query(x)| serde_urlencoded::to_string(x).ok())
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

    let authorized = tokio::task::spawn_blocking({
        let db = db.clone();
        let user = user.clone();
        move || {
            let collection = db.root::<Username>()?.traverse::<User>()?;

            Ok::<_, Error>(
                collection
                    .get(&user.username)?
                    .ok_or(argon2::password_hash::Error::Password)
                    .map_err(Error::from)
                    .and_then(|User { password, .. }| {
                        let hash = PasswordHash::new(&password)?;
                        Ok(Argon2::default().verify_password(user.password.as_bytes(), &hash)?)
                    })
                    .and_then(|_| {
                        let user = collection.key(&user.username)?.ok_or(Error::NotFound)?;

                        session.insert("user", user).unwrap();
                        Ok(())
                    })
                    .is_ok(),
            )
        }
    })
    .await??;

    if !authorized {
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
