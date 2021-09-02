use crate::{AuthServer, Extras};
use actix::Addr;
use actix_web::web;
use oxide_auth_actix::{OAuthOperation, OAuthRequest, OAuthResponse, Refresh, WebError};

pub async fn post_refresh(
    req: OAuthRequest,
    state: web::Data<Addr<AuthServer>>,
) -> Result<OAuthResponse, WebError> {
    state.send(Refresh(req).wrap(Extras::Nothing)).await?
}
