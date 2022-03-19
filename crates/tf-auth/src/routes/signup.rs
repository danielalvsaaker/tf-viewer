use super::{Callback, UserForm};
use crate::templates::SignUp;
use crate::{database::Database, error::Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::{
    extract::{Extension, Form, Query},
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/", get(get_signup).post(post_signup))
}

async fn get_signup(query: Option<Query<Callback<'_>>>) -> impl IntoResponse {
    let query = query.as_ref().map(|x| x.as_str()).unwrap_or_default();
    SignUp { query }.into_response()
}

async fn post_signup(
    Extension(db): Extension<Database>,
    Form(user): Form<UserForm>,
    query: Option<Query<Callback<'_>>>,
) -> Result<impl IntoResponse> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(user.password.as_bytes(), &salt)?
        .serialize()
        .to_string();

    db.insert(user.username, hash)?;
    let query = query.as_ref().map(|x| x.as_str()).unwrap_or_default();

    Ok(Redirect::to(query.parse().unwrap_or_default()))
}
