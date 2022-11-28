use super::session::Session;
use crate::{
    database::{
        resource::{client::EncodedClient, user::User},
        Database,
    },
    templates,
};
use axum::{
    extract::{FromRef, State},
    response::IntoResponse,
    routing::get,
    Router,
};

pub fn routes<S>() -> Router<S>
where
    S: Send + Sync + 'static + Clone,
    Database: FromRef<S>,
{
    Router::new().route("/", get(index))
}

pub async fn index(Session { user }: Session, State(db): State<Database>) -> impl IntoResponse {
    let collection = db
        .root::<User>()
        .unwrap()
        .traverse::<EncodedClient>()
        .unwrap();

    let keys = collection.keys(&user, 0, 10, false).unwrap();
    let clients = keys
        .flat_map(|key| collection.get(&key))
        .flatten()
        .collect::<Vec<_>>();

    templates::Index { clients: &clients }.into_response()
}
