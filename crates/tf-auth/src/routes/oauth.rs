use crate::{
    database::{
        resource::user::{Authorization, AuthorizationQuery, User},
        Database,
    },
    error::{Error, Result},
    routes::session::Session,
    solicitor::Solicitor,
    Consent,
};
use axum::{
    extract::{FromRef, Query, State},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use oxide_auth::{
    endpoint::{OwnerConsent, PreGrant, QueryParameter, Solicitation},
    frontends::simple::endpoint::FnSolicitor,
};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};
use tf_database::primitives::Key;

pub fn routes<S>() -> Router<S>
where
    S: Send + Sync + 'static + Clone,
    Database: FromRef<S>,
    crate::State: FromRef<S>,
{
    Router::new()
        .route("/authorize", get(get_authorize).post(post_authorize))
        .route("/refresh", get(refresh))
        .route("/token", post(token))
}

async fn get_authorize(
    State(state): State<crate::State>,
    State(db): State<Database>,
    Session { user }: Session,
    request: OAuthRequest,
) -> Result<impl IntoResponse> {
    state
        .endpoint()
        .await
        .with_solicitor(Solicitor::new(db, user))
        .authorization_flow()
        .execute(request)
        .await
        .map(IntoResponse::into_response)
        .map_err(Into::into)
}

async fn post_authorize(
    State(state): State<crate::State>,
    State(db): State<Database>,
    Query(consent): Query<Consent>,
    Session { user }: Session,
    request: OAuthRequest,
) -> Result<impl IntoResponse> {
    state
        .endpoint()
        .await
        .with_solicitor(FnSolicitor(
            move |_: &mut OAuthRequest, solicitation: Solicitation| {
                if let Consent::Allow = consent {
                    tokio::task::spawn_blocking({
                        let PreGrant {
                            client_id, scope, ..
                        } = solicitation.pre_grant().clone();
                        let db = db.clone();

                        move || {
                            let query = AuthorizationQuery {
                                user,
                                client: client_id.parse()?,
                            };

                            let collection = db.root::<User>()?.traverse::<Authorization>()?;

                            let authorization = collection.get(&query)?;

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
        .execute(request)
        .await
        .map(IntoResponse::into_response)
        .map_err(Into::into)
}

async fn token(
    State(state): State<crate::State>,
    request: OAuthRequest,
) -> Result<OAuthResponse, WebError> {
    let grant_type = request
        .body()
        .and_then(|x| x.unique_value("grant_type"))
        .unwrap_or_default();

    match &*grant_type {
        "refresh_token" => refresh(State(state), request).await,
        _ => {
            state
                .endpoint()
                .await
                .access_token_flow()
                .execute(request)
                .await
        }
    }
}

async fn refresh(
    State(state): State<crate::State>,
    request: OAuthRequest,
) -> Result<OAuthResponse, WebError> {
    state.endpoint().await.refresh_flow().execute(request).await
}
