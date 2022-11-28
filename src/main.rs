mod cache;
mod error;
mod routes;
mod state;

/*
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;
*/

use axum::{Router, Server};
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

use tf_auth::scopes::Grant;
use tf_database::Database;
use tf_events::Broker;
use tf_graphql::Schema;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let database = Database::open("db").unwrap();
    let auth_db = tf_auth::database::Database::open("db-auth").unwrap();

    let state = tf_auth::State::new(auth_db.clone());
    let broker = Broker::default();

    let schema = Schema::build(Default::default(), Default::default(), Default::default())
        .data(broker.clone())
        .data(database.clone())
        .finish();

    tokio::task::spawn({
        let database = database.clone();
        async move {
            loop {
                let database = database.clone();
                tokio::task::spawn_blocking(move || {
                    database.compact().unwrap();
                })
                .await
                .unwrap();

                tokio::time::sleep(tokio::time::Duration::from_secs(3600 * 3)).await;
            }
        }
    });

    std::fs::write("schema.graphql", &schema.sdl()).unwrap();

    let state = state::AppState {
        broker,
        cache: Default::default(),
        state,
        schema,
        database: state::Database {
            main: database,
            auth: auth_db,
        },
    };

    let middleware = tower::ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(CorsLayer::permissive());

    let router = Router::new()
        .nest("/oauth", tf_auth::routes())
        .nest("/user/:user_id/activity", routes::activity::router())
        .merge(routes::graphql::routes())
        .layer(middleware)
        .with_state(state);

    Server::bind(&([0, 0, 0, 0], 12001).into())
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
