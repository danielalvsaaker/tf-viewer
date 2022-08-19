use super::client::EncodedClient;
use serde::{Deserialize, Serialize};
use tf_database::{
    error::Result,
    primitives::{Index, Key, Relation},
    query::{ClientQuery, UserQuery},
    resource::Resource,
    Traverse,
};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

impl Resource for User {
    const NAME: &'static str = "user";

    type Key = UserQuery;
}

impl Traverse<EncodedClient> for User {
    type Collection = Relation<ClientQuery, EncodedClient, UserQuery, User>;
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

pub struct AuthorizationQuery {
    pub user: UserQuery,
    pub client: ClientQuery,
}

impl Key for AuthorizationQuery {
    fn as_key(&self) -> Vec<u8> {
        [self.user.as_key(), self.client.as_key()].concat()
    }

    fn as_prefix(&self) -> [u8; 21] {
        self.user.as_prefix()
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let (prefix, suffix) = bytes.split_at(21);

        Ok(Self {
            user: UserQuery::from_bytes(prefix)?,
            client: ClientQuery::from_bytes(suffix)?,
        })
    }
}

impl Traverse<Authorization> for User {
    type Collection = Relation<AuthorizationQuery, Authorization, UserQuery, User>;
}

#[derive(Serialize, Deserialize)]
pub struct Authorization {
    pub scope: oxide_auth::primitives::scope::Scope,
}

impl Resource for Authorization {
    const NAME: &'static str = "authorization";

    type Key = AuthorizationQuery;
}
