use oxide_auth::{
    code_grant::{
        accesstoken::Request as AccessTokenRequest, authorization::Request as AuthorizationRequest,
    },
    endpoint,
    frontends::simple::extensions,
    primitives::grant::Extensions,
};
use oxide_auth_async::endpoint::{AccessTokenExtension, AuthorizationExtension, Extension};

pub struct Empty;

impl Extension for Empty {}

#[derive(Default)]
pub struct AddonList {
    inner: extensions::AddonList,
}

impl std::ops::Deref for AddonList {
    type Target = extensions::AddonList;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for AddonList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[async_trait::async_trait]
impl AuthorizationExtension for AddonList {
    async fn extend(
        &mut self,
        request: &(dyn AuthorizationRequest + Sync),
    ) -> std::result::Result<Extensions, ()> {
        endpoint::AuthorizationExtension::extend(&mut self.inner, request)
    }
}

#[async_trait::async_trait]
impl AccessTokenExtension for AddonList {
    async fn extend(
        &mut self,
        request: &(dyn AccessTokenRequest + Sync),
        data: Extensions,
    ) -> std::result::Result<Extensions, ()> {
        endpoint::AccessTokenExtension::extend(&mut self.inner, request, data)
    }
}

impl Extension for AddonList {
    fn authorization(&mut self) -> Option<&mut (dyn AuthorizationExtension + Send)> {
        Some(self)
    }

    fn access_token(&mut self) -> Option<&mut (dyn AccessTokenExtension + Send)> {
        Some(self)
    }
}
