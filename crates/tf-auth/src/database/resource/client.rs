use oxide_auth::primitives::registrar::EncodedClient as Inner;
use serde::{Deserialize, Serialize};
use tf_database::{primitives::Relation, query::ClientQuery, resource::Resource, Traverse};

#[derive(Serialize, Deserialize)]
pub struct EncodedClient {
    pub inner: Inner,
}

impl Resource for EncodedClient {
    const NAME: &'static str = "client";

    type Key = ClientQuery;
}

impl Traverse<ClientName> for EncodedClient {
    type Collection = Relation<ClientQuery, ClientName, ClientQuery, EncodedClient>;
}

#[derive(Serialize, Deserialize)]
pub struct ClientName {
    pub inner: String,
}

impl Resource for ClientName {
    const NAME: &'static str = "client_name";

    type Key = ClientQuery;
}
