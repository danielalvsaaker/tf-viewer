use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use axum::{extract::Extension, Router};

pub fn routes() -> Router {
    let store = async_session::MemoryStore::new();

    Router::new()
        .merge(oauth::routes())
        .nest("/signin", signin::routes())
        .nest("/signout", signout::routes())
        .nest("/signup", signup::routes())
        .layer(Extension(store))
}

mod oauth;
mod signin;
mod signout;
mod signup;

#[derive(Default, Serialize, Deserialize)]
pub struct Callback<'a> {
    callback: Cow<'a, str>,
}

impl<'a> Callback<'a> {
    fn as_str(&self) -> &str {
        self.callback.as_ref()
    }

    fn from_str(callback: &'a str) -> Self {
        Self {
            callback: Cow::Borrowed(callback),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct UserForm {
    pub username: String,
    pub password: String,
}
