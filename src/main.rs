mod cache;
mod error;
mod routes;

use axum::{AddExtensionLayer, Router, Server};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use std::sync::Arc;
use tf_database::Database;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = Database::load_or_create().unwrap();
    let state = Arc::new(tf_auth::InnerState::preconfigured());
    let cache = cache::ThumbnailCache::new();

    let router = Router::new()
        .nest("/oauth", tf_auth::routes())
        .nest("/user", routes::user::router())
        .nest("/user/:user_id/activity", routes::activity::router())
        .nest("/user/:user_id/gear", routes::gear::router())
        .layer(AddExtensionLayer::new(database))
        .layer(AddExtensionLayer::new(state))
        .layer(AddExtensionLayer::new(cache))
        .layer(CorsLayer::permissive())
        .layer(CompressionLayer::new());

    Server::bind(&([127, 0, 0, 1], 8777).into())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
