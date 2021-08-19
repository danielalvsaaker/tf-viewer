mod error;
mod routes;

use actix_web::{middleware::Compress, web, App, HttpServer, Responder, HttpResponse};
use tf_database::Database;

async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html;charset=utf8")
        .body(include_str!("../static/index.html"))
}

async fn favicon() -> impl Responder {
    const FAVICON: &'static [u8] = include_bytes!("../static/img/favicon.ico");

    HttpResponse::Ok()
        .content_type("image/x-icon")
        .body(FAVICON)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::load_or_create().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .app_data(web::Data::new(database.clone()))
            .app_data(web::PayloadConfig::new(1024 * 1024 * 15))
            .route("/", web::route().to(index))
            .route("/favicon.ico", web::route().to(favicon))
            .service(actix_files::Files::new("/static", "static"))
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
