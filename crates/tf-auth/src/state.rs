use crate::{
    database::Database,
    endpoint::{extension::Empty, Endpoint},
};
use oxide_auth::{
    frontends::simple::endpoint::Vacant,
    primitives::{authorizer::AuthMap, generator::RandomGenerator, issuer::TokenMap},
};
use oxide_auth_async::primitives;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct State {
    authorizer: Arc<Mutex<AuthMap<RandomGenerator>>>,
    issuer: Arc<Mutex<TokenMap<RandomGenerator>>>,
    registrar: Database,
}

impl State {
    pub fn new(registrar: Database) -> Self {
        Self {
            authorizer: Arc::new(Mutex::new(AuthMap::new(RandomGenerator::new(16)))),
            issuer: Arc::new(Mutex::new(TokenMap::new(RandomGenerator::new(16)))),
            registrar,
        }
    }

    pub async fn endpoint(
        &self,
    ) -> Endpoint<'_, impl primitives::Registrar, Empty, Vacant, Vacant> {
        Endpoint {
            registrar: &self.registrar,
            authorizer: self.authorizer.lock().await.into(),
            issuer: self.issuer.lock().await.into(),
            extension: Empty,
            solicitor: Vacant,
            scopes: Vacant,
        }
    }
}
