use crate::{AuthServer, Extras};
use actix::Addr;
use actix_web::web;
use oxide_auth_actix::{OAuthOperation, OAuthRequest, OAuthResponse, Token, WebError};

pub async fn post_token(
    req: OAuthRequest,
    state: web::Data<Addr<AuthServer>>,
) -> Result<OAuthResponse, WebError> {
    state.send(Token(req).wrap(Extras::Nothing)).await?
}
