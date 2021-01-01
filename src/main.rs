mod database;
mod error;
mod middleware;
mod models;
pub mod parser;
mod routes;

#[cfg(feature = "jemalloc")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

pub use database::Database;
pub use models::{
    Activity, ActivityType, Duration, Gear, GearType, Lap, Record, Session, TimeStamp, UserTotals,
};
pub use parser::*;

use actix_files::Files;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer, ResponseError};

use error::{Error, ErrorKind};
use middleware::{AuthType, CheckLogin, Restricted};
use routes::{
    activity::{
        activity, activity_index, activity_index_post, activity_settings, activity_settings_post,
    },
    authentication::{signin, signin_post, signout, signup, signup_post},
    gear::{gear_add, gear_add_post, gear_index, gear_settings, gear_settings_post},
    index::index,
    upload::{upload, upload_post},
    user::{user, user_avatar, user_avatar_post, user_index, user_settings, user_settings_post},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = Database::load_or_create().expect("Failed to load");

    println!("Running at 127.0.0.1:2000");

    HttpServer::new(move || {
        App::new()
            .default_service(
                web::route().to(|| {
                    Error::BadRequest(ErrorKind::NotFound, "Page not found").error_response()
                }),
            )
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("tf-viewer")
                    .secure(false),
            ))
            .data(data.clone())
            .service(Files::new("/static", "static"))
            .service(web::resource("/static").name("static"))
            .service(
                web::resource("/signin")
                    .name("signin")
                    .wrap(CheckLogin::new(AuthType::Public))
                    .route(web::get().to(signin))
                    .route(web::post().to(signin_post)),
            )
            .service(
                web::resource("/signup")
                    .name("signup")
                    .wrap(CheckLogin::new(AuthType::Public))
                    .route(web::get().to(signup))
                    .route(web::post().to(signup_post)),
            )
            .service(
                web::scope("")
                    .wrap(CheckLogin::new(AuthType::Restricted))
                    .service(web::resource("/").name("index").to(index))
                    .service(web::resource("/signout").name("signout").to(signout))
                    .service(
                        web::resource("/upload")
                            .name("upload")
                            .route(web::get().to(upload))
                            .route(web::post().to(upload_post)),
                    )
                    .service(
                        web::scope("user")
                            .service(
                                web::resource("/")
                                    .name("user_index")
                                    .route(web::get().to(user_index)),
                            )
                            .service(web::resource("/{username}").name("user").to(user))
                            .service(
                                web::resource("/{username}/activity")
                                    .name("activity_index")
                                    .route(web::get().to(activity_index))
                                    .route(web::post().to(activity_index_post)),
                            )
                            .service(
                                web::resource("/{username}/activity/{activity}/settings")
                                    .name("activity_settings")
                                    .wrap(Restricted)
                                    .route(web::get().to(activity_settings))
                                    .route(web::post().to(activity_settings_post)),
                            )
                            .service(
                                web::resource("/{username}/settings")
                                    .name("user_settings")
                                    .wrap(Restricted)
                                    .route(web::get().to(user_settings))
                                    .route(web::post().to(user_settings_post)),
                            )
                            .service(
                                web::resource("/{username}/settings/avatar")
                                    .name("user_avatar")
                                    .wrap(Restricted)
                                    .route(web::get().to(user_avatar))
                                    .route(web::post().to(user_avatar_post)),
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
                                    .wrap(Restricted)
                                    .route(web::get().to(gear_add))
                                    .route(web::post().to(gear_add_post)),
                            )
                            .service(
                                web::resource("/{username}/gear/{gear}")
                                    .name("gear_settings")
                                    .wrap(Restricted)
                                    .route(web::get().to(gear_settings))
                                    .route(web::post().to(gear_settings_post)),
                            ),
                    ),
            )
    })
    .bind("0.0.0.0:2000")?
    .run()
    .await
}
