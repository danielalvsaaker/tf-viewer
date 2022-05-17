use crate::error::Result;
use oxide_auth::primitives::{registrar::Client, scope::Scope};

pub mod resources;

use resources::{EncodedClient, User};
use std::borrow::Cow;
use tf_database::{
    primitives::Key,
    query::{ClientQuery, UserQuery},
};

use oxide_auth::primitives::registrar::{
    Argon2, BoundClient, ClientUrl, PreGrant, RegisteredClient, Registrar, RegistrarError,
};

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

impl Registrar for Database {
    fn bound_redirect<'a>(&self, bound: ClientUrl<'a>) -> Result<BoundClient<'a>, RegistrarError> {
        let client = self
            .inner
            .root::<User>()
            .map_err(|_| RegistrarError::PrimitiveError)?
            .traverse::<EncodedClient>()
            .map_err(|_| RegistrarError::PrimitiveError)?
            .get(
                &ClientQuery::from_bytes(bound.client_id.as_bytes())
                    .map_err(|_| RegistrarError::Unspecified)?,
            )
            .map_err(|_| RegistrarError::PrimitiveError)?
            .ok_or(RegistrarError::Unspecified)?
            .inner;

        let registered_url = match bound.redirect_uri {
            None => client.redirect_uri,
            Some(ref url) => {
                let original = std::iter::once(&client.redirect_uri);
                let alternatives = client.additional_redirect_uris.iter();

                original
                    .chain(alternatives)
                    .find(|&registered| *registered == *url.as_ref())
                    .cloned()
                    .ok_or(RegistrarError::Unspecified)?
            }
        };

        Ok(BoundClient {
            client_id: bound.client_id,
            redirect_uri: Cow::Owned(registered_url),
        })
    }

    fn negotiate(
        &self,
        bound: BoundClient,
        _scope: Option<Scope>,
    ) -> Result<PreGrant, RegistrarError> {
        let client = self
            .inner
            .root::<User>()
            .map_err(|_| RegistrarError::PrimitiveError)?
            .traverse::<EncodedClient>()
            .map_err(|_| RegistrarError::PrimitiveError)?
            .get(&ClientQuery::from_bytes(bound.client_id.as_bytes()).unwrap())
            .map_err(|_| RegistrarError::PrimitiveError)?
            .map(|x| x.inner)
            .unwrap();

        Ok(PreGrant {
            client_id: bound.client_id.into_owned(),
            redirect_uri: bound.redirect_uri.into_owned(),
            scope: client.default_scope,
        })
    }

    fn check(&self, client_id: &str, passphrase: Option<&[u8]>) -> Result<(), RegistrarError> {
        let password_policy = Argon2::default();

        self.inner
            .root::<User>()
            .map_err(|_| RegistrarError::PrimitiveError)?
            .traverse::<EncodedClient>()
            .map_err(|_| RegistrarError::PrimitiveError)?
            .get(&ClientQuery::from_bytes(client_id.as_bytes()).unwrap())
            .map_err(|_| RegistrarError::PrimitiveError)?
            .ok_or(RegistrarError::Unspecified)
            .map(|x| x.inner)
            .and_then(|client| {
                RegisteredClient::new(&client, &password_policy).check_authentication(passphrase)
            })?;

        Ok(())
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
        user: &UserQuery,
    ) -> Result<()> {
        let inner = client.encode(&Argon2::default());
        let client = EncodedClient { inner };

        self.inner
            .root::<User>()?
            .traverse::<EncodedClient>()?
            .insert(query, &client, user)?;

        Ok(())
    }
}
