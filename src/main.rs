mod database;
mod routes;
mod models;
mod middleware;
pub mod parser;

pub use database::Database;
pub use models::{Activity, Session, Record, Lap, TimeStamp, User, Gear};
pub use parser::*;

use dotenv::dotenv;

use actix_web::{App, HttpServer, web, guard, HttpResponse, middleware::normalize::{NormalizePath, TrailingSlash}, http};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_files::Files;

use routes::{
    upload::{upload, upload_post},
    index::index, 
    authentication::{login, login_post, logout, register, register_post}, 
    user::{user, userindex, userindex_post},
    activity::{activity, activityindex, activityindex_post},
    gear::{gear, gearindex},
};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

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
        //.wrap(NormalizePath::new(TrailingSlash::MergeOnly))
        .app_data(data.clone())
        .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(routes::error::ErrorTemplate::not_found)
        )
        .service(Files::new("/static", "static"))
        .service(
            web::resource("/static")
            .name("static")
        )
        .service(
            web::resource("/login")
            .name("login")
            .route(web::get().to(login))
            .route(web::post().to(login_post))
        )
        .service(
            web::resource("/logout")
            .name("logout")
            .to(logout)
        )
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
            .route(web::get().to(upload))
            .route(web::post().to(upload_post))
        )
        .service(
            web::scope("/user")
                .wrap(middleware::CheckLogin)
                .service(
                    web::resource("/")
                        .name("userindex")
                        .route(web::get().to(userindex))
                        .route(web::post().to(userindex_post))
                )
                .service(
                    web::resource("/{user}")
                        .name("user")
                        .to(user)
                )
                .service(
                    web::resource("/{user}/activity")
                    .name("activityindex")
                    .route(web::get().to(activityindex))
                    .route(web::post().to(activityindex_post))
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
    .bind("0.0.0.0:2000")?
    .run()
    .await

}
