use crate::primitives::Guard;
use oxide_auth::{
    endpoint::{OAuthError, Template, WebRequest},
    frontends::simple::extensions::Pkce,
    primitives::{authorizer::AuthMap, generator::RandomGenerator, issuer::TokenMap, scope::Scope},
};
use oxide_auth_async::{
    endpoint::{
        self, access_token::AccessTokenFlow, authorization::AuthorizationFlow,
        refresh::RefreshFlow, resource::ResourceFlow,
    },
    primitives,
};
use oxide_auth_axum::OAuthRequest;

pub mod extension;

pub struct Endpoint<'a, Registrar, Extension, Solicitor, Scopes> {
    pub(super) registrar: &'a Registrar,
    pub(super) authorizer: Guard<'a, AuthMap<RandomGenerator>>,
    pub(super) issuer: Guard<'a, TokenMap<RandomGenerator>>,
    pub(super) extension: Extension,
    pub(super) solicitor: Solicitor,
    pub(super) scopes: Scopes,
}

impl<'a, Registrar, Extension, Solicitor, Scopes>
    Endpoint<'a, Registrar, Extension, Solicitor, Scopes>
where
    Registrar: primitives::Registrar + Send + Sync,
    Extension: endpoint::Extension + Send + Sync,
    Solicitor: endpoint::OwnerSolicitor<OAuthRequest> + Send + Sync,
    Scopes: oxide_auth::endpoint::Scopes<OAuthRequest> + Send + Sync,
{
    pub fn with_scopes(
        self,
        scopes: &'a [Scope],
    ) -> Endpoint<'a, Registrar, Extension, Solicitor, &'a [Scope]> {
        Endpoint {
            registrar: self.registrar,
            authorizer: self.authorizer,
            issuer: self.issuer,
            extension: self.extension,
            solicitor: self.solicitor,
            scopes,
        }
    }

    pub fn with_solicitor<S>(
        self,
        solicitor: S,
    ) -> Endpoint<'a, Registrar, extension::AddonList, S, Scopes>
    where
        S: endpoint::OwnerSolicitor<OAuthRequest>,
    {
        let pkce = Pkce::required();
        let mut extension = extension::AddonList::default();
        extension.push_code(pkce);

        Endpoint {
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

impl<'a, Request, Registrar, Extension, Solicitor, Scopes> endpoint::Endpoint<Request>
    for Endpoint<'a, Registrar, Extension, Solicitor, Scopes>
where
    Request: WebRequest,
    Request::Response: Default,
    Request::Error: From<OAuthError>,
    Registrar: primitives::Registrar + Sync,
    Extension: endpoint::Extension + Send,
    Solicitor: endpoint::OwnerSolicitor<Request> + Send,
    Scopes: oxide_auth::endpoint::Scopes<Request>,
{
    type Error = Request::Error;

    fn registrar(&self) -> Option<&(dyn primitives::Registrar + Sync)> {
        Some(self.registrar)
    }

    fn authorizer_mut(&mut self) -> Option<&mut (dyn primitives::Authorizer + Send)> {
        Some(&mut self.authorizer)
    }

    fn issuer_mut(&mut self) -> Option<&mut (dyn primitives::Issuer + Send)> {
        Some(&mut self.issuer)
    }

    fn owner_solicitor(&mut self) -> Option<&mut (dyn endpoint::OwnerSolicitor<Request> + Send)> {
        Some(&mut self.solicitor)
    }

    fn scopes(&mut self) -> Option<&mut dyn oxide_auth::endpoint::Scopes<Request>> {
        Some(&mut self.scopes)
    }

    fn response(&mut self, _: &mut Request, _: Template) -> Result<Request::Response, Self::Error> {
        Ok(Default::default())
    }

    fn error(&mut self, err: OAuthError) -> Self::Error {
        err.into()
    }

    fn web_error(&mut self, err: Request::Error) -> Self::Error {
        err
    }

    fn extension(&mut self) -> Option<&mut (dyn endpoint::Extension + Send)> {
        Some(&mut self.extension)
    }
}
