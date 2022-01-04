use oxide_auth::{
    code_grant::extensions::Pkce,
    endpoint::{
        AccessTokenFlow, AuthorizationFlow, Endpoint, Extension, OAuthError, OwnerSolicitor,
        RefreshFlow, ResourceFlow, Scopes, Template,
    },
    frontends::simple::{endpoint::Vacant, extensions::AddonList},
    primitives::{prelude::*, registrar::RegisteredUrl, scope::Scope},
};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};
use serde::Deserialize;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Deserialize)]
#[serde(tag = "consent", rename_all = "lowercase")]
pub enum Consent {
    Allow,
    Deny,
}

mod routes;
pub use routes::routes;
pub mod templates;

pub type State = Arc<InnerState>;

pub struct InnerState {
    registrar: Mutex<ClientMap>,
    authorizer: Mutex<AuthMap<RandomGenerator>>,
    issuer: Mutex<TokenMap<RandomGenerator>>,
}

pub struct AuthEndpoint<'a, E, S, C> {
    registrar: MutexGuard<'a, ClientMap>,
    authorizer: MutexGuard<'a, AuthMap<RandomGenerator>>,
    issuer: MutexGuard<'a, TokenMap<RandomGenerator>>,
    extension: E,
    solicitor: S,
    scopes: C,
}

impl<'a, E, S, C> Endpoint<OAuthRequest> for AuthEndpoint<'a, E, S, C>
where
    E: Extension,
    S: OwnerSolicitor<OAuthRequest>,
    C: Scopes<OAuthRequest>,
{
    type Error = WebError;

    fn registrar(&self) -> Option<&dyn Registrar> {
        Some(&self.registrar)
    }

    fn authorizer_mut(&mut self) -> Option<&mut dyn Authorizer> {
        Some(&mut self.authorizer)
    }

    fn issuer_mut(&mut self) -> Option<&mut dyn Issuer> {
        Some(&mut self.issuer)
    }

    fn owner_solicitor(&mut self) -> Option<&mut dyn OwnerSolicitor<OAuthRequest>> {
        Some(&mut self.solicitor)
    }

    fn scopes(&mut self) -> Option<&mut dyn Scopes<OAuthRequest>> {
        Some(&mut self.scopes)
    }

    fn response(
        &mut self,
        _: &mut OAuthRequest,
        _: Template,
    ) -> Result<OAuthResponse, Self::Error> {
        Ok(Default::default())
    }

    fn error(&mut self, err: OAuthError) -> WebError {
        WebError::Endpoint(err)
    }

    fn web_error(&mut self, err: WebError) -> WebError {
        err
    }

    fn extension(&mut self) -> Option<&mut dyn Extension> {
        Some(&mut self.extension)
    }
}

impl InnerState {
    pub fn preconfigured() -> Self {
        Self {
            registrar: Mutex::new(
                vec![Client::public(
                    "tf-viewer",
                    RegisteredUrl::Semantic("http://localhost:8080/callback".parse().unwrap()),
                    "activity:read activity:write user:read gear:write gear:read"
                        .parse()
                        .unwrap(),
                )]
                .into_iter()
                .collect(),
            ),
            // Authorization tokens are 16 byte random keys to a memory hash map.
            authorizer: Mutex::new(AuthMap::new(RandomGenerator::new(16))),
            // Bearer tokens are also random generated but 256-bit tokens, since they live longer
            // and this example is somewhat paranoid.
            //
            // We could also use a `TokenSigner::ephemeral` here to create signed tokens which can
            // be read and parsed by anyone, but not maliciously created. However, they can not be
            // revoked and thus don't offer even longer lived refresh tokens.
            issuer: Mutex::new(TokenMap::new(RandomGenerator::new(16))),
        }
    }

    pub fn endpoint(&self) -> AuthEndpoint<'_, (), Vacant, Vacant> {
        AuthEndpoint {
            registrar: self.registrar.lock().unwrap(),
            authorizer: self.authorizer.lock().unwrap(),
            issuer: self.issuer.lock().unwrap(),
            extension: (),
            solicitor: Vacant,
            scopes: Vacant,
        }
    }
}

impl<'a, E: 'a, O: 'a, C: 'a> AuthEndpoint<'a, E, O, C>
where
    E: Extension,
    O: OwnerSolicitor<OAuthRequest>,
    C: Scopes<OAuthRequest>,
{
    pub fn with_scopes(self, scopes: &'a [Scope]) -> AuthEndpoint<'a, E, O, &'a [Scope]> {
        AuthEndpoint {
            registrar: self.registrar,
            authorizer: self.authorizer,
            issuer: self.issuer,
            extension: self.extension,
            solicitor: self.solicitor,
            scopes,
        }
    }

    pub fn with_solicitor<S>(self, solicitor: S) -> AuthEndpoint<'a, AddonList, S, C>
    where
        S: OwnerSolicitor<OAuthRequest>,
    {
        let pkce = Pkce::required();
        let mut extension = AddonList::new();
        extension.push_code(pkce);

        AuthEndpoint {
            registrar: self.registrar,
            authorizer: self.authorizer,
            issuer: self.issuer,
            extension,
            solicitor,
            scopes: self.scopes,
        }
    }

    pub fn authorization_flow(self) -> AuthorizationFlow<Self, OAuthRequest> {
        match AuthorizationFlow::prepare(self) {
            Ok(flow) => flow,
            Err(_) => unreachable!(),
        }
    }

    pub fn access_token_flow(self) -> AccessTokenFlow<Self, OAuthRequest> {
        match AccessTokenFlow::prepare(self) {
            Ok(flow) => flow,
            Err(_) => unreachable!(),
        }
    }

    pub fn refresh_flow(self) -> RefreshFlow<Self, OAuthRequest> {
        match RefreshFlow::prepare(self) {
            Ok(flow) => flow,
            Err(_) => unreachable!(),
        }
    }

    pub fn resource_flow(self) -> ResourceFlow<Self, OAuthRequest> {
        match ResourceFlow::prepare(self) {
            Ok(flow) => flow,
            Err(_) => unreachable!(),
        }
    }
}
