use crate::AuthState;
use oxide_auth_axum::{OAuthRequest, OAuthResponse, WebError};
use axum::extract::Extension;

pub async fn post_refresh(
    req: OAuthRequest,
    Extension(state): Extension<AuthState>,
) -> Result<OAuthResponse, WebError> {
    state
        .endpoint()
        .refresh_flow()
        .execute(req)
}
