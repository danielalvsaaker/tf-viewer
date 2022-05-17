use super::Callback;
use crate::{database::Database, error::Result, templates::Authorize, Consent, State, session::Session};
use askama::Template;
use axum::{
    extract::{Extension, OriginalUri, Query},
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use oxide_auth::{
    endpoint::{OwnerConsent, QueryParameter, Solicitation},
    frontends::simple::endpoint::FnSolicitor,
};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};
use tf_database::primitives::Key;

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
    session: Session,
    OriginalUri(uri): OriginalUri,
) -> Result<impl IntoResponse> {
    if session.id().is_none() {
        let callback =
            Callback::from_str(uri.path_and_query().map(|x| x.as_str()).unwrap_or_default());

        let uri = format!(
            "/oauth/signin?{}",
            serde_urlencoded::to_string(&callback).unwrap()
        );
        return Ok(Redirect::to(&uri).into_response());
    }

    let id = session.id().unwrap();

    Ok(state
        .endpoint(db)
        .with_solicitor(FnSolicitor(
            move |req: &mut OAuthRequest, pre_grant: Solicitation| {
                let body = Authorize::new(req, pre_grant, &id);

                match body.render() {
                    Ok(inner) => OwnerConsent::InProgress(
                        OAuthResponse::default()
                            .content_type("text/html")
                            .unwrap()
                            .body(&inner),
                    ),
                    Err(inner) => {
                        OwnerConsent::Error(WebError::InternalError(Some(inner.to_string())))
                    }
                }
            },
        ))
        .authorization_flow()
        .execute(req)
        .map(IntoResponse::into_response)?)
}

async fn post_authorize(
    req: OAuthRequest,
    Extension(state): Extension<State>,
    Extension(db): Extension<Database>,
    Query(consent): Query<Consent>,
    session: Session,
) -> Result<impl IntoResponse> {
    let user_id = match session.id() {
        Some(username) => username,
        _ => return Ok(Redirect::to("/oauth/signin").into_response()),
    };

    Ok(state
        .endpoint(db)
        .with_solicitor(FnSolicitor(
            move |_: &mut OAuthRequest, _: Solicitation| match consent {
                Consent::Allow => OwnerConsent::Authorized(user_id.as_string()),
                Consent::Deny => OwnerConsent::Denied,
            },
        ))
        .authorization_flow()
        .execute(req)
        .map(IntoResponse::into_response)?)
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
        _ => state.endpoint(db).access_token_flow().execute(req),
    }
}

async fn refresh(
    req: OAuthRequest,
    Extension(state): Extension<State>,
    Extension(db): Extension<Database>,
) -> Result<OAuthResponse, WebError> {
    state.endpoint(db).refresh_flow().execute(req)
}
