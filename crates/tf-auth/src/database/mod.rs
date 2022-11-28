use crate::error::Result;

pub mod resource;

use oxide_auth::primitives::registrar::{Argon2, Client};
use resource::{
    client::{ClientName, EncodedClient},
    user::User,
};
use tf_database::query::{ClientQuery, UserQuery};

#[derive(Clone)]
pub struct Database {
    inner: tf_database::Database,
}

impl std::ops::Deref for Database {
    type Target = tf_database::Database;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl Database {
    pub fn open<P>(path: P) -> Result<Self>
    where
        P: AsRef<std::path::Path>,
    {
        Ok(Self {
            inner: tf_database::Database::open(path)?,
        })
    }

    pub fn register_client(
        &self,
        query: &ClientQuery,
        client: Client,
        client_name: String,
        user: &UserQuery,
    ) -> Result<()> {
        let inner = client.encode(&Argon2::default());
        let client = EncodedClient { inner };

        let collection = self.inner.root::<User>()?.traverse::<EncodedClient>()?;

        collection.insert(query, &client, user)?;

        collection.traverse::<ClientName>(query)?.insert(
            query,
            &ClientName { inner: client_name },
            query,
        )?;

        Ok(())
    }
}
