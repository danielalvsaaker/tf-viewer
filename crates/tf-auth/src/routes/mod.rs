use crate::primitives::scopes::Grant;
use axum::{response::IntoResponse, routing::get, Json, Router};
use axum_sessions::{async_session::MemoryStore, SameSite, SessionLayer};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub fn routes() -> Router {
    let session_layer = SessionLayer::new(MemoryStore::new(), nanoid::nanoid!(128).as_bytes())
        .with_cookie_name("tf_session")
        // TODO: Set based on config
        .with_secure(true)
        .with_save_unchanged(false)
        .with_cookie_path("/oauth/")
        .with_same_site_policy(SameSite::Lax);

    Router::new()
        .merge(oauth::routes())
        .nest("/client", client::routes())
        .nest("/signin", signin::routes())
        .nest("/signout", signout::routes())
        .nest("/signup", signup::routes())
        .route("/whoami", get(whoami))
        .layer(session_layer)
}

async fn whoami(grant: Grant<()>) -> impl IntoResponse {
    Json(grant.grant.owner_id)
}

mod client;
mod oauth;
mod signin;
mod signout;
mod signup;

#[derive(Default, Serialize, Deserialize)]
pub struct Callback<'a> {
    callback: Cow<'a, str>,
}

impl<'a> Callback<'a> {
    fn as_str(&self) -> &str {
        self.callback.as_ref()
    }

    fn from_str(callback: &'a str) -> Self {
        Self {
            callback: Cow::Borrowed(callback),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct UserForm {
    pub username: String,
    pub password: String,
}
