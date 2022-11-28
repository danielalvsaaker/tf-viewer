pub use primitives::scopes;

pub mod database;
mod endpoint;
mod primitives;
mod solicitor;
mod state;

pub use state::State;

#[derive(serde::Deserialize)]
#[serde(tag = "consent", rename_all = "lowercase")]
pub enum Consent {
    Allow,
    Deny,
}

mod routes;
pub use routes::routes;
pub mod error;
pub mod templates;

pub mod websocket {
    use oxide_auth::code_grant::resource::Request;
    use oxide_auth::primitives::scope::Scope;
    pub use oxide_auth_async::{
        code_grant::resource::{protect, Endpoint},
        primitives::Issuer,
    };
    use serde::Deserialize;
    use std::borrow::Cow;

    #[derive(Default)]
    pub struct WebSocketRequest {
        pub payload: Option<Payload>,
    }

    pub struct WebSocketResponse;

    #[derive(Deserialize)]
    pub struct Payload {
        #[serde(rename = "Authorization")]
        authorization: String,
    }

    impl Request for WebSocketRequest {
        fn valid(&self) -> bool {
            true
        }
        fn token(&self) -> Option<Cow<'_, str>> {
            self.payload
                .as_ref()
                .map(|payload| Cow::Borrowed(payload.authorization.as_ref()))
        }
    }

    impl<'a, Registrar, Extension, Solicitor> Endpoint
        for super::endpoint::Endpoint<'a, Registrar, Extension, Solicitor, &'a [Scope]>
    {
        fn scopes(&mut self) -> &[Scope] {
            self.scopes
        }

        fn issuer(&mut self) -> &mut (dyn Issuer + Send) {
            &mut self.issuer
        }
    }
}
