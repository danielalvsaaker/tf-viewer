use crate::{AuthServer, Consent, Extras};
use actix::Addr;
use actix_identity::Identity;
use actix_web::{web, HttpRequest};
use oxide_auth::endpoint::WebResponse;
use oxide_auth_actix::{Authorize, OAuthOperation, OAuthRequest, OAuthResponse, WebError};

pub async fn get_authorize(
    id: Identity,
    (oreq, state): (OAuthRequest, web::Data<Addr<AuthServer>>),
    req: HttpRequest,
) -> Result<OAuthResponse, WebError> {
    if let Some(username) = id.identity() {
        return state
            .send(Authorize(oreq).wrap(Extras::AuthGet { username }))
            .await?;
    }

    let mut response = OAuthResponse::ok();
    response.unauthorized("Bearer")?;

    let mut url = req.url_for_static("signin").unwrap();

    if let Some(x) = req.uri().path_and_query() {
        url.query_pairs_mut().append_pair("callback", x.as_str());
    }

    response.redirect(url)?;
    Ok(response)
}

pub async fn post_authorize(
    id: Identity,
    web::Query(consent): web::Query<Consent>,
    oreq: OAuthRequest,
    req: HttpRequest,
    state: web::Data<Addr<AuthServer>>,
) -> Result<OAuthResponse, WebError> {
    if let Some(username) = id.identity() {
        state
            .send(Authorize(oreq).wrap(Extras::AuthPost { consent, username }))
            .await?
    } else {
        let mut response = OAuthResponse::ok();
        response.unauthorized("Bearer")?;

        let mut url = req.url_for_static("signin").unwrap();

        if let Some(x) = req.uri().path_and_query() {
            url.query_pairs_mut().append_pair("callback", x.as_str());
        }
        response.redirect(url)?;
        Ok(response)
    }
}
