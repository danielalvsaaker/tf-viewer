use crate::{database::Database, primitives::scopes::Grant};
use axum::{extract::FromRef, response::IntoResponse, routing::get, Json, Router};
use axum_sessions::{async_session::MemoryStore, PersistencePolicy, SameSite, SessionLayer};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod session {
    use super::Callback;
    use axum::{extract::FromRequestParts, http::request::Parts, response::Redirect};
    use axum_sessions::extractors::ReadableSession;
    use tf_models::query::UserQuery;

    pub struct Session {
        pub user: UserQuery,
    }

    #[axum::async_trait]
    impl<S> FromRequestParts<S> for Session
    where
        S: Send + Sync + 'static,
    {
        type Rejection = Redirect;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
            let session = ReadableSession::from_request_parts(parts, state)
                .await
                .ok()
                .and_then(|session| session.get("user"));

            if let Some(user) = session {
                Ok(Self { user })
            } else {
                let callback = Callback::from_str(
                    parts
                        .uri
                        .path_and_query()
                        .map(|x| x.as_str())
                        .map(|x| x.trim_start_matches('/'))
                        .unwrap_or_default(),
                );

                let uri = format!(
                    "/oauth/signin?{}",
                    serde_urlencoded::to_string(&callback).unwrap()
                );

                Err(Redirect::to(&uri))
            }
        }
    }
}

pub fn routes<S>() -> Router<S>
where
    crate::State: FromRef<S>,
    Database: FromRef<S>,
    S: Send + Sync + 'static + Clone,
{
    let session_layer = SessionLayer::new(MemoryStore::new(), nanoid::nanoid!(128).as_bytes())
        .with_cookie_name("tf_session")
        // TODO: Set based on config
        .with_secure(true)
        .with_persistence_policy(PersistencePolicy::ChangedOnly)
        .with_cookie_path("/oauth/")
        .with_same_site_policy(SameSite::Lax);

    Router::new()
        .merge(oauth::routes())
        .merge(index::routes())
        .nest("/client", client::routes())
        .nest("/signin", signin::routes())
        .nest("/signout", signout::routes().with_state(()))
        .nest("/signup", signup::routes())
        .route("/whoami", get(whoami))
        .layer(session_layer)
}

async fn whoami(grant: Grant<()>) -> impl IntoResponse {
    Json(grant.grant.owner_id)
}

mod client;
mod index;
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
