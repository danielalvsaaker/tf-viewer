mod error;
mod routes;

use axum::{AddExtensionLayer, Router, Server};
use tower_http::cors::CorsLayer;

use tf_database::Database;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = Database::load_or_create().unwrap();
    let state = std::sync::Arc::new(tf_auth::InnerState::preconfigured());

    let router = Router::new()
        .nest("/oauth", tf_auth::routes())
        .nest("/user/:user_id/activity", routes::activity::router())
        .layer(AddExtensionLayer::new(database))
        .layer(AddExtensionLayer::new(state))
        .layer(CorsLayer::permissive());

    Server::bind(&([127, 0, 0, 1], 8777).into())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
