use crate::{AuthServer, Extras};
use actix::Addr;
use actix_web::web;
use oxide_auth_actix::{OAuthOperation, OAuthRequest, OAuthResponse, Token, WebError, Refresh};
use oxide_auth::endpoint::QueryParameter;

pub async fn post_token(
    req: OAuthRequest,
    state: web::Data<Addr<AuthServer>>,
) -> Result<OAuthResponse, WebError> {
    let grant_type = req.body().and_then(|x| x.unique_value("grant_type")).unwrap_or_default();

    match &*grant_type {
        "refresh_token" => state.send(Refresh(req).wrap(Extras::Nothing)).await?,
        _ => state.send(Token(req).wrap(Extras::Nothing)).await?,
    }
}
