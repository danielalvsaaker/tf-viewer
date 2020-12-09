mod database;
mod middleware;
mod models;
pub mod parser;
mod routes;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub use database::Database;
pub use models::{Activity, Duration, Gear, Lap, Record, Session, TimeStamp, User};
pub use parser::*;

use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{guard, web, App, HttpServer};

use routes::{
    activity::{activity, activity_index, activity_index_post},
    authentication::{login, login_post, logout, register, register_post},
    gear::{gear, gear_add, gear_index},
    index::index,
    upload::{upload, upload_post},
    user::{user, user_index, user_index_post},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(Database::load_or_create().expect("Failed to load"));

    println!("Running at 127.0.0.1:2000");

    HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("tf-viewer")
                    .secure(false),
            ))
            .app_data(data.clone())
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(routes::error::ErrorTemplate::not_found),
            )
            .service(Files::new("/static", "static"))
            .service(web::resource("/static").name("static"))
            .service(
                web::resource("/login")
                    .name("login")
                    .route(web::get().to(login))
                    .route(web::post().to(login_post)),
            )
            .service(web::resource("/logout").name("logout").to(logout))
            .service(
                web::resource("/register")
                    .name("register")
                    .route(web::get().to(register))
                    .route(web::post().to(register_post)),
            )
            .service(
                web::resource("/")
                    .name("index")
                    .wrap(middleware::CheckLogin)
                    .to(index),
            )
            .service(
                web::resource("/upload")
                    .name("upload")
                    .wrap(middleware::CheckLogin)
                    .route(web::get().to(upload))
                    .route(web::post().to(upload_post)),
            )
            .service(
                web::scope("/user")
                    .wrap(middleware::CheckLogin)
                    .service(
                        web::resource("/")
                            .name("user_index")
                            .route(web::get().to(user_index))
                            .route(web::post().to(user_index_post)),
                    )
                    .service(web::resource("/{username}").name("user").to(user))
                    .service(
                        web::resource("/{username}/activity")
                            .name("activity_index")
                            .route(web::get().to(activity_index))
                            .route(web::post().to(activity_index_post)),
                    )
                    .service(
                        web::resource("/{username}/activity/{activity}")
                            .name("activity")
                            .to(activity),
                    )
                    .service(
                        web::resource("/{username}/gear")
                            .name("gear_index")
                            .to(gear_index),
                    )
                    .service(
                        web::resource("/{username}/gear/add")
                            .name("gear_add")
                            .to(gear_add),
                    ),
            )
    })
    .bind("0.0.0.0:2000")?
    .run()
    .await
}
