use async_graphql::{async_trait, Context, Error, ErrorExtensions, Guard, Result};
use oxide_auth::primitives::{grant::Grant, scope::Scope};

pub struct OAuthGuard {
    scope: Scope,
}

impl OAuthGuard {
    pub fn new<S>(_scope: S) -> Self
    where
        S: tf_scopes::Scope,
    {
        Self {
            scope: S::SCOPE.parse().unwrap(),
        }
    }
}

#[async_trait::async_trait]
impl Guard for OAuthGuard {
    async fn check(&self, ctx: &Context<'_>) -> Result<()> {
        let grant = ctx.data_unchecked::<Grant>();

        if self.scope.allow_access(&grant.scope) {
            Ok(())
        } else {
            Err(Error::new("Invalid scope").extend_with(|_, e| e.set("code", 401)))
        }
    }
}
