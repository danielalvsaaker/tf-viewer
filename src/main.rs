mod database;
mod routes;
mod error;
mod models;
mod middleware;
pub mod parser;
use std::fs;

pub use database::Database;
pub use models::{Activity, User, Gear};
pub use parser::*;
pub use error::{Error, Result};


use actix_web::{App, HttpServer, web};
use actix_identity::{CookieIdentityPolicy, IdentityService, Identity};
use actix_files::Files;

use routes::{
    index::index, 
    authentication::{login, login_post, logout, register, register_post}, 
    user::{user, userindex},
    activity::{activity, activityindex},
    gear::{gear, gearindex},
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
                    .secure(false)
                )
        )
        .app_data(data.clone())
        .service(Files::new("/static", "static/"))
        .service(
            web::resource("/static")
            .name("static")
        )
        .service(login)
        .service(login_post)
        .service(logout)
        .service(
            web::resource("/register")
            .name("register")
            .route(web::get().to(register))
            .route(web::post().to(register_post))
        )
        .service(
            web::resource("/")
            .name("index")
            .wrap(middleware::CheckLogin)
            .to(index)
        )
        .service(
            web::resource("/upload")
            .name("upload")
            .wrap(middleware::CheckLogin)
            .to(index)
        )
        .service(
            web::scope("/user")
                .wrap(middleware::CheckLogin)
                .service(
                    web::resource("/")
                        .name("userindex")
                        .to(userindex)
                )
                .service(
                    web::resource("/{user}")
                        .name("user")
                        .to(user)
                )
                .service(
                    web::resource("/{user}/activity")
                    .name("activityindex")
                    .to(activityindex)
                )
                .service(
                    web::resource("/{user}/activity/{activity}")
                    .name("activity")
                    .to(activity)
                )
                .service(
                    web::resource("/{user}/gear")
                    .name("gearindex")
                    .to(gearindex)
                )
    )})
    .bind("127.0.0.1:2000")?
    .run()
    .await

}
