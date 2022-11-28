use crate::database::Database;
use crate::error::Result;
use crate::templates;

use axum::{
    extract::{Form, FromRef, State},
    response::{IntoResponse, Json, Redirect},
    routing::get,
    Router,
};
use axum_sessions::extractors::ReadableSession;
use oxide_auth::primitives::registrar::{Client, RegisteredUrl};
use serde::{Deserialize, Serialize};
use tf_database::{
    primitives::Key,
    query::{ClientQuery, UserQuery},
};
use tf_models::ClientId;

pub fn routes<S>() -> Router<S>
where
    S: Send + Sync + 'static + Clone,
    Database: FromRef<S>,
{
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
    name: String,
    redirect_uri: String,
    r#type: ClientType,
}

async fn post_client(
    State(db): State<Database>,
    session: ReadableSession,
    Form(client): Form<ClientForm>,
) -> Result<impl IntoResponse> {
    let user = match session.get::<UserQuery>("user") {
        Some(user) => user,
        _ => return Ok(Redirect::to("/oauth/signin").into_response()),
    };

    let mut client_secret = None;

    let client_id = {
        let id = ClientId::from_bytes(nanoid::nanoid!().as_bytes())?;

        ClientQuery {
            user_id: user.user_id,
            id,
        }
    };

    let client_name = client.name;

    let client = match client.r#type {
        ClientType::Confidential => {
            let secret = nanoid::nanoid!(32);

            let client = Client::confidential(
                &client_id.as_string(),
                RegisteredUrl::Semantic(client.redirect_uri.parse().unwrap()),
                "".parse().unwrap(),
                secret.as_bytes(),
            );

            client_secret = Some(secret);

            client
        }
        ClientType::Public => Client::public(
            &client_id.as_string(),
            RegisteredUrl::Semantic(client.redirect_uri.parse().unwrap()),
            "".parse().unwrap(),
        ),
    };

    db.register_client(&client_id, client, client_name, &user)?;

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
