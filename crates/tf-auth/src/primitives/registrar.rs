use crate::database::{
    resource::{client::EncodedClient, user::User},
    Database,
};
use oxide_auth::primitives::{
    registrar::{Argon2, BoundClient, ClientUrl, PreGrant, RegisteredClient, RegistrarError},
    scope::Scope,
};
use oxide_auth_async::primitives::Registrar;
use tf_database::{primitives::Key, query::ClientQuery};

#[async_trait::async_trait]
impl Registrar for Database {
    async fn bound_redirect<'a>(
        &self,
        bound: ClientUrl<'a>,
    ) -> Result<BoundClient<'a>, RegistrarError> {
        let client = tokio::task::spawn_blocking({
            let db = self.clone();
            let query = ClientQuery::from_bytes(bound.client_id.as_bytes());

            move || {
                db.root::<User>()
                    .and_then(|root| root.traverse::<EncodedClient>())
                    .and_then(|root| root.get(&query?))
                    .map_err(|_| RegistrarError::PrimitiveError)?
                    .map(|EncodedClient { inner }| inner)
                    .ok_or(RegistrarError::Unspecified)
            }
        })
        .await
        .map_err(|_| RegistrarError::PrimitiveError)??;

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
            redirect_uri: std::borrow::Cow::Owned(registered_url),
        })
    }

    async fn negotiate<'a>(
        &self,
        bound: BoundClient<'a>,
        scope: Option<Scope>,
    ) -> Result<PreGrant, RegistrarError> {
        let client = tokio::task::spawn_blocking({
            let db = self.clone();
            let query = ClientQuery::from_bytes(bound.client_id.as_bytes());

            move || {
                db.root::<User>()
                    .and_then(|root| root.traverse::<EncodedClient>())
                    .and_then(|root| root.get(&query?))
                    .map_err(|_| RegistrarError::PrimitiveError)?
                    .map(|EncodedClient { inner }| inner)
                    .ok_or(RegistrarError::Unspecified)
            }
        })
        .await
        .map_err(|_| RegistrarError::PrimitiveError)??;

        let scope = scope
            .and_then(|scope| {
                scope
                    .iter()
                    .filter(|scope| tf_scopes::SCOPES.contains(scope))
                    .collect::<Vec<_>>()
                    .join(" ")
                    .parse()
                    .ok()
            })
            .unwrap_or(client.default_scope);

        Ok(PreGrant {
            client_id: bound.client_id.into_owned(),
            redirect_uri: bound.redirect_uri.into_owned(),
            scope,
        })
    }

    async fn check(
        &self,
        client_id: &str,
        passphrase: Option<&[u8]>,
    ) -> Result<(), RegistrarError> {
        let password_policy = Argon2::default();

        tokio::task::spawn_blocking({
            let db = self.clone();
            let query = ClientQuery::from_bytes(client_id.as_bytes());

            move || {
                db.root::<User>()
                    .and_then(|root| root.traverse::<EncodedClient>())
                    .and_then(|root| root.get(&query?))
                    .map_err(|_| RegistrarError::PrimitiveError)?
                    .map(|EncodedClient { inner }| inner)
                    .ok_or(RegistrarError::Unspecified)
            }
        })
        .await
        .map_err(|_| RegistrarError::PrimitiveError)?
        .and_then(|client| {
            RegisteredClient::new(&client, &password_policy).check_authentication(passphrase)
        })?;

        Ok(())
    }
}
