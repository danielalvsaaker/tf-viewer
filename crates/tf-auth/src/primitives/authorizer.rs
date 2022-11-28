use super::Guard;
use oxide_auth::primitives::{authorizer::Authorizer, grant::Grant};
use oxide_auth_async::primitives::Authorizer as AuthorizerAsync;

#[async_trait::async_trait]
impl<T> AuthorizerAsync for Guard<'_, T>
where
    T: Authorizer + Send,
{
    async fn authorize(&mut self, grant: Grant) -> Result<String, ()> {
        Authorizer::authorize(&mut *self.inner, grant)
    }

    async fn extract(&mut self, token: &str) -> Result<Option<Grant>, ()> {
        Authorizer::extract(&mut *self.inner, token)
    }
}
