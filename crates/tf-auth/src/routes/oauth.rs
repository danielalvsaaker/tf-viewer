use super::Callback;
use crate::{
    templates::Authorize,
    Consent, State,
};
use axum::{
    extract::{Extension, OriginalUri, Query},
    http::Uri,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Router,
};
use oxide_auth::{
    endpoint::{OwnerConsent, QueryParameter, Solicitation},
    frontends::simple::endpoint::FnSolicitor,
};
use askama::Template;
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};

pub fn routes() -> Router {
    Router::new()
        .route("/authorize", get(get_authorize).post(post_authorize))
        .route("/refresh", get(refresh))
        .route("/token", post(token))
}

async fn get_authorize(
    req: OAuthRequest,
    Extension(state): Extension<State>,
    session: crate::session::Session,
    OriginalUri(uri): OriginalUri,
) -> Result<impl IntoResponse, WebError> {
    if session.id().is_none() {
        let callback =
            Callback::from_str(uri.path_and_query().map(|x| x.as_str()).unwrap_or_default());

        let uri = format!(
            "/oauth/signin?{}",
            serde_urlencoded::to_string(&callback).unwrap()
        );
        return Ok(Redirect::to(uri.parse().unwrap_or_default()).into_response());
    }

    let username = session.id().unwrap();

    state
        .endpoint()
        .with_solicitor(FnSolicitor(
            move |req: &mut OAuthRequest, pre_grant: Solicitation| {
                let body = Authorize::new(req, pre_grant, &username);

                match body.render() {
                    Ok(inner) => OwnerConsent::InProgress(
                        OAuthResponse::default()
                            .content_type("text/html")
                            .unwrap()
                            .body(&inner),
                    ),
                    Err(inner) => OwnerConsent::Error(WebError::InternalError(Some(inner.to_string()))),
                }
            },
        ))
        .authorization_flow()
        .execute(req)
        .map(IntoResponse::into_response)
}

async fn post_authorize(
    req: OAuthRequest,
    Extension(state): Extension<State>,
    Query(consent): Query<Consent>,
    session: crate::session::Session,
) -> Result<impl IntoResponse, WebError> {
    let username = match session.id() {
        Some(username) => username,
        _ => return Ok(Redirect::to(Uri::from_static("/oauth/signin")).into_response()),
    };

    state
        .endpoint()
        .with_solicitor(FnSolicitor(
            move |_: &mut OAuthRequest, _: Solicitation| match consent {
                Consent::Allow => OwnerConsent::Authorized(username.to_owned()),
                Consent::Deny => OwnerConsent::Denied,
            },
        ))
        .authorization_flow()
        .execute(req)
        .map(IntoResponse::into_response)
}

async fn token(
    req: OAuthRequest,
    Extension(state): Extension<State>,
) -> Result<OAuthResponse, WebError> {
    let grant_type = req
        .body()
        .and_then(|x| x.unique_value("grant_type"))
        .unwrap_or_default();

    match &*grant_type {
        "refresh_token" => refresh(req, Extension(state)).await,
        _ => state.endpoint().access_token_flow().execute(req),
    }
}

async fn refresh(
    req: OAuthRequest,
    Extension(state): Extension<State>,
) -> Result<OAuthResponse, WebError> {
    state.endpoint().refresh_flow().execute(req)
}
