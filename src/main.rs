mod error;
mod routes;

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

use actix_web::{middleware::Compress, web, App, HttpServer, Responder};
use tf_database::Database;

async fn index() -> impl Responder {
    include_str!("../static/index.html")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = Database::load_or_create().unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Compress::default())
            .app_data(web::Data::new(database.clone()))
            .route("/", web::route().to(index))
            .service(actix_files::Files::new("/static", "./static"))
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
