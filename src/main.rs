mod cache;
mod error;
mod routes;

use mimalloc::MiMalloc;
use tikv_jemallocator::Jemalloc;

/*
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;
*/

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::Extension, response, routing::*, Router, Server};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use std::sync::Arc;
use tf_database::Database;

use tf_graphql::Schema;
use tf_auth::scopes::Grant;

async fn graphql_handler(
    //grant: Grant<()>,
    Extension(schema): Extension<Schema>,
    req: GraphQLRequest,
    //Extension(_database): Extension<tf_auth::database::Database>,
) -> GraphQLResponse {
    schema
        .execute(req.into_inner()
                 //.data(grant.grant)
        )
        .await
        .into()
}

async fn graphql_playground() -> impl response::IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = Database::open("db-nebari").unwrap();
    //let auth_database = tf_auth::database::Database::load_or_create().unwrap();

    /*
    auth_database.register_client(oxide_auth::primitives::registrar::Client::public(
            "pYGCSFvo8OtQXWu2y34SK",
            oxide_auth::primitives::registrar::RegisteredUrl::Semantic("http://localhost:8080/callback".parse().unwrap()),
            "gear:read user:read activity:read".parse().unwrap()));
            */


    //let state = Arc::new(tf_auth::InnerState::new(&auth_database));
    let cache = cache::ThumbnailCache::new();
    let schema = Schema::build(
        tf_graphql::QueryRoot,
        async_graphql::EmptyMutation,
        async_graphql::EmptySubscription,
    )
    .data(database.clone())
    //.data(state.clone())
    .finish();

    let router = Router::new()
        .route("/", get(graphql_playground).post(graphql_handler))
        .nest("/oauth", tf_auth::routes())
        .nest("/user/:user_id/activity", routes::activity::router())
        .layer(Extension(schema))
        .layer(Extension(database))
        //.layer(Extension(auth_database))
        //.layer(Extension(state))
        .layer(Extension(cache))
        .layer(CorsLayer::permissive())
        .layer(CompressionLayer::new());

    Server::bind(&([127, 0, 0, 1], 8777).into())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
