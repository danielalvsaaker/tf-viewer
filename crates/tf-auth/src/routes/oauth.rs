use super::Callback;
use crate::{
    database::{
        resource::user::{Authorization, AuthorizationQuery, User},
        Database,
    },
    error::{Error, Result},
    solicitor::Solicitor,
    Consent, State,
};
use axum::{
    extract::{Extension, OriginalUri, Query},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use axum_sessions::extractors::ReadableSession;
use oxide_auth::{
    endpoint::{OwnerConsent, QueryParameter, Solicitation},
    frontends::simple::endpoint::FnSolicitor,
};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};
use tf_database::{primitives::Key, query::UserQuery};

pub fn routes() -> Router {
    Router::new()
        .route("/authorize", get(get_authorize).post(post_authorize))
        .route("/refresh", get(refresh))
        .route("/token", post(token))
}

async fn get_authorize(
    req: OAuthRequest,
    Extension(state): Extension<State>,
    Extension(db): Extension<Database>,
    session: ReadableSession,
    OriginalUri(uri): OriginalUri,
) -> Result<impl IntoResponse> {
    let user = if let Some(user) = session.get::<UserQuery>("user") {
        user
    } else {
        let callback =
            Callback::from_str(uri.path_and_query().map(|x| x.as_str()).unwrap_or_default());

        let uri = format!(
            "/oauth/signin?{}",
            serde_urlencoded::to_string(&callback).unwrap()
        );
        return Ok(Redirect::to(&uri).into_response());
    };

    state
        .endpoint(db.clone())
        .await
        .with_solicitor(Solicitor::new(db, user))
        .authorization_flow()
        .execute(req)
        .await
        .map(IntoResponse::into_response)
        .map_err(Into::into)
}

async fn post_authorize(
    req: OAuthRequest,
    Extension(state): Extension<State>,
    Extension(db): Extension<Database>,
    Query(consent): Query<Consent>,
    session: ReadableSession,
) -> Result<impl IntoResponse> {
    let user = match session.get::<UserQuery>("user") {
        Some(user) => user,
        _ => return Ok(Redirect::to("/oauth/signin").into_response()),
    };

    state
        .endpoint(db.clone())
        .await
        .with_solicitor(FnSolicitor(
            move |_: &mut OAuthRequest, solicitation: Solicitation| {
                if let Consent::Allow = consent {
                    tokio::task::spawn_blocking({
                        let db = db.clone();
                        let solicitation = solicitation.into_owned();

                        move || {
                            let query = AuthorizationQuery {
                                user,
                                client: solicitation.pre_grant().client_id.parse()?,
                            };

                            let collection = db.root::<User>()?.traverse::<Authorization>()?;

                            let authorization = collection.get(&query)?;
                            let scope = solicitation.pre_grant().scope.clone();

                            if authorization.is_none()
                                || authorization
                                    .map(|authorization| authorization.scope < scope)
                                    .unwrap_or_default()
                            {
                                collection.insert(&query, &Authorization { scope }, &user)?;
                            }

                            Ok::<_, Error>(())
                        }
                    });

                    OwnerConsent::Authorized(user.as_string())
                } else {
                    OwnerConsent::Denied
                }
            },
        ))
        .authorization_flow()
        .execute(req)
        .await
        .map(IntoResponse::into_response)
        .map_err(Into::into)
}

async fn token(
    req: OAuthRequest,
    Extension(state): Extension<State>,
    Extension(db): Extension<Database>,
) -> Result<OAuthResponse, WebError> {
    let grant_type = req
        .body()
        .and_then(|x| x.unique_value("grant_type"))
        .unwrap_or_default();

    match &*grant_type {
        "refresh_token" => refresh(req, Extension(state), Extension(db)).await,
        _ => {
            state
                .endpoint(db)
                .await
                .access_token_flow()
                .execute(req)
                .await
        }
    }
}

async fn refresh(
    req: OAuthRequest,
    Extension(state): Extension<State>,
    Extension(db): Extension<Database>,
) -> Result<OAuthResponse, WebError> {
    state.endpoint(db).await.refresh_flow().execute(req).await
}
