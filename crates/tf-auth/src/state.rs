use super::endpoint::{extension::Empty, Endpoint};
use oxide_auth::{
    frontends::simple::endpoint::Vacant,
    primitives::{authorizer::AuthMap, generator::RandomGenerator, issuer::TokenMap},
};
use oxide_auth_async::primitives;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type State = Arc<InnerState>;

pub struct InnerState {
    authorizer: Mutex<AuthMap<RandomGenerator>>,
    issuer: Mutex<TokenMap<RandomGenerator>>,
}

impl Default for InnerState {
    fn default() -> Self {
        Self::new()
    }
}

impl InnerState {
    pub fn new() -> Self {
        Self {
            authorizer: Mutex::new(AuthMap::new(RandomGenerator::new(16))),
            issuer: Mutex::new(TokenMap::new(RandomGenerator::new(16))),
        }
    }

    pub async fn endpoint<Registrar>(
        &self,
        registrar: Registrar,
    ) -> Endpoint<'_, Registrar, Empty, Vacant, Vacant>
    where
        Registrar: primitives::Registrar,
    {
        Endpoint {
            registrar,
            authorizer: self.authorizer.lock().await.into(),
            issuer: self.issuer.lock().await.into(),
            extension: Empty,
            solicitor: Vacant,
            scopes: Vacant,
        }
    }
}
