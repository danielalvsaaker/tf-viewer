pub use tf_scopes::*;

pub struct Grant<S = ()> {
    pub grant: oxide_auth::primitives::grant::Grant,
    _type: std::marker::PhantomData<S>,
}

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use oxide_auth_axum::{OAuthResource, OAuthResponse, WebError};

#[axum::async_trait]
impl<State, Scope> FromRequestParts<State> for Grant<Scope>
where
    crate::State: FromRef<State>,
    State: Send + Sync + 'static,
    Scope: tf_scopes::Scope,
{
    type Rejection = Result<OAuthResponse, WebError>;

    async fn from_request_parts(parts: &mut Parts, state: &State) -> Result<Self, Self::Rejection> {
        let req = OAuthResource::from_request_parts(parts, state)
            .await
            .map_err(Err)?;

        let state = crate::State::from_ref(state);

        let auth = state
            .endpoint()
            .await
            .with_scopes(&[Scope::SCOPE.parse().unwrap()])
            .resource_flow()
            .execute(req.into())
            .await;

        auth.map(|grant| Self {
            grant,
            _type: Default::default(),
        })
    }
}
