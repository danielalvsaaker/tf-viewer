use crate::database::Database;
use crate::error::Result;
use crate::templates;

use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use oxide_auth::primitives::registrar::{Client, RegisteredUrl};
use tf_database::{primitives::Key, query::ClientQuery};
use tf_models::ClientId;

pub fn routes() -> Router {
    Router::new().route("/", get(get_client).post(post_client))
}

async fn get_client() -> impl IntoResponse {
    templates::Client.into_response()
}

#[derive(serde::Deserialize)]
struct ClientForm {
    redirect_uri: String,
    scopes: String,
}

async fn post_client(
    session: crate::session::Session,
    Form(client): Form<ClientForm>,
    Extension(db): Extension<Database>,
) -> Result<impl IntoResponse> {
    let username = match session.id() {
        Some(username) => username,
        _ => return Ok(Redirect::to("/oauth/signin").into_response()),
    };

    let client_id = nanoid::nanoid!();
    let query = ClientQuery {
        user_id: username.user_id,
        id: ClientId::from_bytes(client_id.as_bytes())?,
    };

    let client = Client::public(
        &query.as_string(),
        RegisteredUrl::Semantic(client.redirect_uri.parse().unwrap()),
        client.scopes.parse().unwrap(),
    );

    db.register_client(&query, client, &username)?;

    Ok(query.as_string().into_response())
}
