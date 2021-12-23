use crate::{templates::authorize_template, Consent, State};
use axum::{
    extract::{Extension, Query},
    routing::{get, post},
    Router,
};
use oxide_auth::{
    endpoint::{OwnerConsent, QueryParameter, Solicitation},
    frontends::simple::endpoint::FnSolicitor,
};
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
) -> Result<OAuthResponse, WebError> {
    let username = "daniel".to_string();

    state
        .endpoint()
        .with_solicitor(FnSolicitor(
            move |req: &mut OAuthRequest, pre_grant: Solicitation| {
                // This will display a page to the user asking for his permission to proceed. The submitted form
                // will then trigger the other authorization handler which actually completes the flow.
                OwnerConsent::InProgress(
                    OAuthResponse::default()
                        .content_type("text/html")
                        .unwrap()
                        .body(&authorize_template(req, pre_grant, &username)),
                )
            },
        ))
        .authorization_flow()
        .execute(req)
}

async fn post_authorize(
    req: OAuthRequest,
    Extension(state): Extension<State>,
    Query(consent): Query<Consent>,
) -> Result<OAuthResponse, WebError> {
    let username = "daniel".to_string();

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
