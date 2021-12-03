mod error;
mod routes;

use actix::Actor;
use actix_cors::Cors;
use actix_web::{middleware::Compress, web, App, HttpServer};
use tf_database::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = web::Data::new(Database::load_or_create().unwrap());
    let auth_database = web::Data::new(tf_auth::Database::load_or_create().unwrap());
    let state = web::Data::new(tf_auth::AuthServer::preconfigured().start());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive().expose_headers(["Location"]))
            .wrap(Compress::default())
            .app_data(database.clone())
            .app_data(state.clone())
            .app_data(web::PayloadConfig::new(1024 * 1024 * 15))
            .configure(tf_auth::config(&auth_database))
            .service(
                web::resource("/user")
                    .name("user_index")
                    .route(web::post().to(routes::user::post_user)),
            )
            .service(
                web::scope("/user")
                    .configure(routes::activity::config)
                    .configure(routes::gear::config)
                    .configure(routes::user::config),
            )
    })
    .bind(("127.0.0.1", 8777))?
    .run()
    .await
}
