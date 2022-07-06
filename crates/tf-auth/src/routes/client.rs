use crate::database::Database;
use crate::error::Result;
use crate::templates;

use axum::{
    extract::{Extension, Form},
    response::{IntoResponse, Json, Redirect},
    routing::get,
    Router,
};
use oxide_auth::primitives::registrar::{Client, RegisteredUrl};
use serde::{Deserialize, Serialize};
use tf_database::{primitives::Key, query::ClientQuery};
use tf_models::ClientId;

pub fn routes() -> Router {
    Router::new().route("/", get(get_client).post(post_client))
}

async fn get_client() -> impl IntoResponse {
    templates::Client.into_response()
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum ClientType {
    Public,
    Confidential,
}

#[derive(Deserialize)]
struct ClientForm {
    redirect_uri: String,
    scopes: String,
    r#type: ClientType,
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

    let mut client_secret = None;

    let client_id = {
        let id = ClientId::from_bytes(nanoid::nanoid!().as_bytes())?;

        ClientQuery {
            user_id: username.user_id,
            id,
        }
    };

    let client = match client.r#type {
        ClientType::Confidential => {
            let secret = nanoid::nanoid!(32);

            let client = Client::confidential(
                &client_id.as_string(),
                RegisteredUrl::Semantic(client.redirect_uri.parse().unwrap()),
                client.scopes.parse().unwrap(),
                secret.as_bytes(),
            );

            client_secret = Some(secret);

            client
        }
        ClientType::Public => Client::public(
            &client_id.as_string(),
            RegisteredUrl::Semantic(client.redirect_uri.parse().unwrap()),
            client.scopes.parse().unwrap(),
        ),
    };

    db.register_client(&client_id, client, &username)?;

    #[derive(Serialize)]
    struct Response {
        client_id: ClientQuery,
        client_secret: Option<String>,
    }

    Ok(Json(Response {
        client_id,
        client_secret,
    })
    .into_response())
}
