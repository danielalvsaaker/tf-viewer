use serde::Deserialize;

use super::{middleware::Redirect, AuthServer, Extras};
use actix::Addr;
use actix_web::web;
use oxide_auth_actix::{OAuthOperation, OAuthRequest, OAuthResponse, Resource, WebError};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/index")
            .wrap(Redirect::Private) .route(web::get().to(index::get_index)),
    )
    .service(
        web::resource("/signin")
            .name("signin")
            .wrap(Redirect::Public)
            .route(web::get().to(signin::get_signin))
            .route(web::post().to(signin::post_signin)),
    )
    .service(
        web::resource("/signup")
            .wrap(Redirect::Public)
            .route(web::get().to(signup::get_signup))
            .route(web::post().to(signup::post_signup)),
    )
    .service(
        web::resource("/signout")
            .wrap(Redirect::Private)
            .route(web::get().to(signout::get_signout)),
    )
    // OAuth2 related routes
    .service(
        web::resource("/authorize")
            .route(web::get().to(authorize::get_authorize))
            .route(web::post().to(authorize::post_authorize)),
    )
    .route("/refresh", web::post().to(refresh::post_refresh))
    .route("/token", web::post().to(token::post_token));
}

mod authorize;
mod index;
mod refresh;
mod signin;
mod signout;
mod signup;
mod token;

#[derive(Deserialize)]
pub struct Callback {
    pub callback: String,
}

#[derive(Deserialize)]
pub struct UserForm {
    pub username: String,
    pub password: String,
}

pub async fn get_user(
    req: OAuthRequest,
    state: web::Data<Addr<AuthServer>>,
) -> Result<OAuthResponse, WebError> {
    match state.send(Resource(req).wrap(Extras::Nothing)).await? {
        Ok(grant) => Ok(OAuthResponse::ok()
            .content_type("application/json")?
            .body(&format!(r#"{{"username":"{}"}}"#, grant.owner_id))),
        Err(Ok(e)) => Ok(e),
        Err(Err(e)) => Err(e),
    }
}
