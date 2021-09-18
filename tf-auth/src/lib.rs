use actix_cors::Cors;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, FromRequest, HttpRequest};
use oxide_auth::primitives::grant::Grant;
use oxide_auth_actix::{OAuthOperation, OAuthResponse, WebError};
use serde::Deserialize;

pub fn config(db: &web::Data<Database>) -> impl Fn(&mut web::ServiceConfig) + '_ {
    move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/oauth")
                .app_data(db.clone())
                .wrap(Cors::permissive())
                .wrap(IdentityService::new(
                    CookieIdentityPolicy::new(&[3; 32]) // <- create cookie identity policy
                        .name("auth-cookie")
                        .secure(false),
                ))
                .configure(routes::config)
                .route("/user", web::get().to(routes::get_user)),
        );
    }
}

mod database;
mod error;
mod middleware;
mod routes;
mod server;
pub mod templates;

pub use database::Database;
pub use server::AuthServer;

pub enum Extras {
    AuthGet { username: String },
    AuthPost { consent: Consent, username: String },
    Nothing,
}

#[derive(Deserialize)]
#[serde(tag = "consent", rename_all = "lowercase")]
pub enum Consent {
    Allow,
    Deny,
}

pub async fn authorize(
    state: &web::Data<actix::Addr<AuthServer>>,
    req: &HttpRequest,
) -> Result<Grant, Result<OAuthResponse, WebError>> {
    state
        .send(
            oxide_auth_actix::Resource(oxide_auth_actix::OAuthRequest::extract(req).await.unwrap())
                .wrap(Extras::Nothing),
        )
        .await
        .map_err(|_| Err(WebError::Mailbox))?
}
