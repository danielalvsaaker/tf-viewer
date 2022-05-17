use oxide_auth::primitives::registrar::EncodedClient as Inner;
use serde::{Deserialize, Serialize};
use tf_database::{
    primitives::{Index, Relation},
    query::{ClientQuery, UserQuery},
    resource::Resource,
    Traverse,
};

#[derive(Serialize, Deserialize)]
pub struct EncodedClient {
    pub inner: Inner,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Username;

impl Resource for Username {
    const NAME: &'static str = "username";

    type Key = String;
}

impl Traverse<User> for Username {
    type Collection = Index<String, Username, UserQuery, User>;
}

impl Resource for EncodedClient {
    const NAME: &'static str = "client";

    type Key = ClientQuery;
}

impl Resource for User {
    const NAME: &'static str = "user";

    type Key = UserQuery;
}

impl Traverse<EncodedClient> for User {
    type Collection = Relation<ClientQuery, EncodedClient, UserQuery, User>;
}
