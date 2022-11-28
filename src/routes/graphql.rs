use crate::{state::AppState, Grant, Schema};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig, ALL_WEBSOCKET_PROTOCOLS},
    Data,
};
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/graphql", get(graphql_playground).post(graphql_handler))
        .route("/subscription", get(graphql_subscription))
}

async fn graphql_handler(
    Grant { grant, .. }: Grant,
    State(schema): State<Schema>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let request = request.into_inner().data(grant);

    schema.execute(request).await.into()
}

async fn graphql_subscription(
    grant: Option<Grant>,
    State(schema): State<Schema>,
    State(state): State<tf_auth::State>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> impl IntoResponse {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema, protocol)
                .on_connection_init(move |value| async move {
                    let mut data = Data::default();

                    if let Some(Grant { grant, .. }) = grant {
                        data.insert(grant);
                        Ok(data)
                    } else if let Ok(payload) =
                        serde_json::from_value::<tf_auth::websocket::Payload>(value)
                    {
                        let grant = tf_auth::websocket::protect(
                            &mut state.endpoint().await.with_scopes(&["".parse().unwrap()]),
                            &tf_auth::websocket::WebSocketRequest {
                                payload: Some(payload),
                            },
                        )
                        .await
                        .unwrap();
                        data.insert(grant);
                        Ok(data)
                    } else {
                        Err("Token is required".into())
                    }
                })
                .serve()
        })
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("http://10.200.200.1:12000/api/ws"),
    ))
}
