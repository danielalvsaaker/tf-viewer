use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::web;
use serde::Deserialize;

pub fn config(db: &web::Data<Database>) -> impl Fn(&mut web::ServiceConfig) + '_ {
    move |cfg: &mut web::ServiceConfig| {
        cfg.service(
            web::scope("/oauth")
                .app_data(db.clone())
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
