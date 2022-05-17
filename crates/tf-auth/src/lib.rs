use oxide_auth::{
    code_grant::extensions::Pkce,
    endpoint::{
        AccessTokenFlow, AuthorizationFlow, Endpoint, Extension, OAuthError, OwnerSolicitor,
        RefreshFlow, ResourceFlow, Scopes, Template,
    },
    frontends::simple::{endpoint::Vacant, extensions::AddonList},
    primitives::{prelude::*, scope::Scope},
};
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};
use serde::Deserialize;
use std::sync::{Arc, Mutex, MutexGuard};

pub mod database;
pub mod session;

#[derive(Deserialize)]
#[serde(tag = "consent", rename_all = "lowercase")]
pub enum Consent {
    Allow,
    Deny,
}

pub mod scopes {
    pub trait Inner {
        const READ: &'static str;
        const WRITE: &'static str;
    }

    pub struct Activity;
    pub struct Gear;
    pub struct User;

    impl Inner for Activity {
        const READ: &'static str = "activity:read";
        const WRITE: &'static str = "activity:write";
    }

    impl Inner for Gear {
        const READ: &'static str = "gear:read";
        const WRITE: &'static str = "gear:write";
    }

    impl Inner for User {
        const READ: &'static str = "user:read";
        const WRITE: &'static str = "user:write";
    }

    pub struct Read<S>(pub S);
    pub struct Write<S>(pub S);

    pub trait Scope {
        const SCOPE: &'static str;
    }

    impl Scope for () {
        const SCOPE: &'static str = "";
    }

    impl<S: Inner> Scope for Read<S> {
        const SCOPE: &'static str = S::READ;
    }

    impl<S: Inner> Scope for Write<S> {
        const SCOPE: &'static str = S::WRITE;
    }

    pub struct Grant<S> {
        pub grant: oxide_auth::primitives::grant::Grant,
        _type: std::marker::PhantomData<S>,
    }

    use super::{State, WebError};
    use axum::{
        body::HttpBody,
        extract::{Extension, FromRequest, RequestParts},
        BoxError,
    };
    use oxide_auth_axum::{OAuthResource, OAuthResponse};

    #[axum::async_trait]
    impl<B, S> FromRequest<B> for Grant<S>
    where
        B: Send + HttpBody,
        B::Data: Send,
        B::Error: Into<BoxError>,
        S: Scope,
    {
        type Rejection = Result<OAuthResponse, WebError>;

        async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
            let Extension(state) = Extension::<State>::from_request(req).await.unwrap();
            let Extension(db) = Extension::<super::Database>::from_request(req)
                .await
                .unwrap();
            let req = OAuthResource::from_request(req).await.unwrap();

            let auth = state
                .endpoint(db)
                .with_scopes(&[S::SCOPE.parse().unwrap()])
                .resource_flow()
                .execute(req.into());

            auth.map(|grant| Self {
                grant,
                _type: Default::default(),
            })
        }
    }
}

mod routes;
pub use routes::routes;
pub mod error;
pub mod templates;
use database::Database;

pub type State = Arc<InnerState>;

pub struct InnerState {
    authorizer: Mutex<AuthMap<RandomGenerator>>,
    issuer: Mutex<TokenMap<RandomGenerator>>,
}

pub struct AuthEndpoint<'a, R, E, S, C> {
    registrar: R,
    authorizer: MutexGuard<'a, AuthMap<RandomGenerator>>,
    issuer: MutexGuard<'a, TokenMap<RandomGenerator>>,
    extension: E,
    solicitor: S,
    scopes: C,
}

impl<'a, R, E, S, C> Endpoint<OAuthRequest> for AuthEndpoint<'a, R, E, S, C>
where
    R: Registrar,
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

impl Default for InnerState {
    fn default() -> Self {
        Self::new()
    }
}

impl InnerState {
    pub fn new() -> Self {
        Self {
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

    pub fn endpoint<R>(&self, registrar: R) -> AuthEndpoint<'_, R, (), Vacant, Vacant>
    where
        R: Registrar,
    {
        AuthEndpoint {
            registrar,
            authorizer: self.authorizer.lock().unwrap(),
            issuer: self.issuer.lock().unwrap(),
            extension: (),
            solicitor: Vacant,
            scopes: Vacant,
        }
    }
}

impl<'a, R: 'a, E: 'a, O: 'a, C: 'a> AuthEndpoint<'a, R, E, O, C>
where
    R: Registrar,
    E: Extension,
    O: OwnerSolicitor<OAuthRequest>,
    C: Scopes<OAuthRequest>,
{
    pub fn with_scopes(self, scopes: &'a [Scope]) -> AuthEndpoint<'a, R, E, O, &'a [Scope]> {
        AuthEndpoint {
            registrar: self.registrar,
            authorizer: self.authorizer,
            issuer: self.issuer,
            extension: self.extension,
            solicitor: self.solicitor,
            scopes,
        }
    }

    pub fn with_solicitor<S>(self, solicitor: S) -> AuthEndpoint<'a, R, AddonList, S, C>
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
