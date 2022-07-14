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

use tf_database::Database;

use tf_auth::{scopes::Grant, State};
use tf_graphql::Schema;

async fn graphql_handler(
    Grant { grant, .. }: Grant,
    Extension(schema): Extension<Schema>,
    Extension(db): Extension<Database>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let request = request.into_inner().data(db).data(grant);

    schema.execute(request).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = Database::open("db").unwrap();
    let auth_db = tf_auth::database::Database::open("db-auth").unwrap();

    let state = State::default();
    let cache = cache::ThumbnailCache::new();
    let schema = Schema::default();

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
