use crate::{error::Result};
use oxide_auth::primitives::{
    registrar::{Client, EncodedClient},
    scope::Scope,
};

use std::borrow::Cow;
use tf_database::{
    query::UserQuery,
    primitives::{self, Relation, Tree},
};
use tf_models::{ClientId, UserId};

#[derive(Clone)]
pub struct Database {
    user: UserTree,
    client: ClientTree,
    pub(super) _db: primitives::Database,
}

#[derive(Clone)]
pub struct ClientTree {
    pub(super) client: Relation<UserQuery, EncodedClient, String, ()>,
}

#[derive(Clone)]
pub struct UserTree {
    pub(super) password: Tree<String, String>,
    pub(super) userid: Tree<String, ()>,
    pub(super) consent: Tree<UserQuery, Scope>,
}

use oxide_auth::primitives::registrar::{
    Argon2, BoundClient, ClientUrl, PreGrant, RegisteredClient, Registrar, RegistrarError,
};

impl Registrar for Database {
    fn bound_redirect<'a>(&self, bound: ClientUrl<'a>) -> Result<BoundClient<'a>, RegistrarError> {
        let client = self
            .client
            .client
            //.get(&ClientId::from_bytes(bound.client_id.as_bytes()).unwrap())
            .get(todo!())
            .map_err(|_| RegistrarError::PrimitiveError)?
            .ok_or(RegistrarError::Unspecified)?;

        let registered_url = match bound.redirect_uri {
            None => client.redirect_uri.clone(),
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
            .client
            .client
            //.get(&ClientId::from_bytes(bound.client_id.as_bytes()).unwrap())
            .get(todo!())
            .map_err(|_| RegistrarError::PrimitiveError)?
            .unwrap();

        Ok(PreGrant {
            client_id: bound.client_id.into_owned(),
            redirect_uri: bound.redirect_uri.into_owned(),
            scope: client.default_scope,
        })
    }

    fn check(&self, client_id: &str, passphrase: Option<&[u8]>) -> Result<(), RegistrarError> {
        let password_policy = Argon2::default();

        self.client
            .client
            //.get(&ClientId::from_bytes(client_id.as_bytes()).unwrap())
            .get(todo!())
            .map_err(|_| RegistrarError::PrimitiveError)?
            .ok_or(RegistrarError::Unspecified)
            .and_then(|client| {
                RegisteredClient::new(&client, &password_policy).check_authentication(passphrase)
            })?;

        Ok(())
    }
}

impl Database {
    pub fn load_or_create() -> Result<Self> {
        let db = primitives::Database::open("auth-db")?;
        /*
        let user = Tree::new(db.open_tree("user_userid").unwrap());

        Ok(Self {
            client: ClientTree {
                client: Relation {
                    local: Tree::new(db.open_tree("client_client").unwrap()),
                    index: Tree::new(db.open_tree("client_owner").unwrap()),
                    foreign: user.clone(),
                },
            },

            user: UserTree {
                password: Tree::new(db.open_tree("user_password").unwrap()),
                userid: user,
                consent: Tree::new(db.open_tree("user_consent").unwrap()),
            },

            _db: db,
        })
        */
        todo!()
    }

    pub fn register_client(&self, client: Client) {
        let client = client.encode(&Argon2::default());
        self.client.client.insert(
            //ClientId::from_bytes(client.client_id.as_bytes()).unwrap(),
            todo!(),
            &client,
            todo!(),
        );
    }

    pub fn insert(&self, _user: String, _password: String) -> Result<()> {
        //self.username_password.insert(&user, &password)?;

        Ok(())
    }

    pub fn get(&self, _user: &String) -> Result<Option<String>> {
        //Ok(self.username_password.get(user)?)
        todo!()
    }
}
