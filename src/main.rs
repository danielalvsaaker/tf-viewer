mod config;
mod database;
mod error;
mod middleware;
mod models;
mod parser;
mod routes;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer, ResponseError};
use database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Database::load_or_create().expect("Failed to load");

    let config = config::config();
    let (cookie_key, secure_cookies) = (config.get_cookie_key(), config.secure_cookies);

    println!("Running at {}:{}", config.address, config.port);

    HttpServer::new(move || {
        App::new()
            .data(data.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_key)
                    .name("tf-viewer")
                    .http_only(true)
                    .secure(secure_cookies),
            ))
            .default_service(web::route().to(|| {
                error::Error::BadRequest(error::ErrorKind::NotFound, "Page not found")
                    .error_response()
            }))
            .service(actix_files::Files::new("/static", "static"))
            .service(web::resource("/static").name("static"))
            .configure(error::config)
            .configure(routes::authentication::config)
            .service(
                web::scope("")
                    .wrap(middleware::CheckLogin::new(
                        middleware::AuthType::Restricted,
                    ))
                    .configure(routes::index::config)
                    .configure(routes::upload::config)
                    .service(
                        web::scope("user")
                            .configure(routes::activity::config)
                            .configure(routes::user::config)
                            .configure(routes::gear::config),
                    ),
            )
    })
    .bind((config.address, config.port))?
    .run()
    .await
}
