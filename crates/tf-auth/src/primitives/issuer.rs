use super::Guard;
use oxide_auth::primitives::{
    grant::Grant,
    issuer::{IssuedToken, Issuer, RefreshedToken},
};
use oxide_auth_async::primitives::Issuer as IssuerAsync;

#[async_trait::async_trait]
impl<T> IssuerAsync for Guard<'_, T>
where
    T: Issuer + Send,
{
    async fn issue(&mut self, grant: Grant) -> Result<IssuedToken, ()> {
        Issuer::issue(&mut **self, grant)
    }

    async fn refresh(&mut self, token: &str, grant: Grant) -> Result<RefreshedToken, ()> {
        Issuer::refresh(&mut **self, token, grant)
    }

    async fn recover_token(&mut self, token: &str) -> Result<Option<Grant>, ()> {
        Issuer::recover_token(&**self, token)
    }

    async fn recover_refresh(&mut self, token: &str) -> Result<Option<Grant>, ()> {
        Issuer::recover_refresh(&**self, token)
    }
}
