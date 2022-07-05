mod cache;
mod error;
mod routes;

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{Html, IntoResponse},
    routing::get,
    Router, Server,
};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use std::sync::Arc;
use tf_database::Database;

use tf_auth::scopes::Grant;
use tf_graphql::Schema;

async fn graphql_handler(
    grant: Grant<()>,
    Extension(schema): Extension<Schema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner().data(grant.grant))
        .await
        .into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = Database::open("db").unwrap();
    let auth_db = tf_auth::database::Database::open("db-auth").unwrap();

    let state = Arc::new(tf_auth::InnerState::new());
    let cache = cache::ThumbnailCache::new();
    let schema = Schema::build(
        tf_graphql::QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .data(database.clone())
    .finish();

    database.compact().unwrap();

    let middleware = tower::ServiceBuilder::new()
        .layer(Extension(schema))
        .layer(Extension(auth_db))
        .layer(Extension(database))
        .layer(Extension(state))
        .layer(Extension(cache))
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive());

    let router = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .nest("/oauth", tf_auth::routes())
        .nest("/user/:user_id/activity", routes::activity::router())
        .layer(middleware);

    Server::bind(&([127, 0, 0, 1], 8777).into())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
