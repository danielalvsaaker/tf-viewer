pub use tf_scopes::*;

pub struct Grant<S = ()> {
    pub grant: oxide_auth::primitives::grant::Grant,
    _type: std::marker::PhantomData<S>,
}

use crate::{database::Database, State};
use axum::{
    body::HttpBody,
    extract::{Extension, FromRequest, RequestParts},
    BoxError,
};
use oxide_auth_axum::{OAuthResource, OAuthResponse, WebError};

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
        let Extension(db) = Extension::<Database>::from_request(req).await.unwrap();
        let req = OAuthResource::from_request(req).await.unwrap();

        let auth = state
            .endpoint(db)
            .await
            .with_scopes(&[S::SCOPE.parse().unwrap()])
            .resource_flow()
            .execute(req.into())
            .await;

        auth.map(|grant| Self {
            grant,
            _type: Default::default(),
        })
    }
}
