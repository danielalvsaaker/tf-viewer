pub use primitives::scopes;

pub mod database;
mod endpoint;
mod primitives;
pub mod session;
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
