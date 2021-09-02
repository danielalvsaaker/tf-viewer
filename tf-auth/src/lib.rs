use actix_web::web;
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/oauth")
            .wrap(actix_identity::IdentityService::new(
                actix_identity::CookieIdentityPolicy::new(&[3; 32]) // <- create cookie identity policy
                    .name("auth-cookie")
                    .secure(false),
            ))
            .configure(routes::configure)
            .route("/user", web::get().to(routes::get_user)),
    );
}

mod middleware;
mod routes;
mod server;
pub mod templates;

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
