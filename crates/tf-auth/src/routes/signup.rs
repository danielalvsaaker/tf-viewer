use super::{Callback, UserForm};
use crate::templates::SignUp;
use crate::{
    database::{
        resources::{User, Username},
        Database,
    },
    error::Result,
};
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
use tf_database::{primitives::Key, query::UserQuery};

pub fn routes() -> Router {
    Router::new().route("/", get(get_signup).post(post_signup))
}

async fn get_signup(query: Option<Query<Callback<'_>>>) -> impl IntoResponse {
    let query = &query
        .as_ref()
        .map(|Query(x)| serde_urlencoded::to_string(x).unwrap())
        .unwrap_or_default();
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
        .serialize();

    let key = UserQuery::from_bytes(nanoid::nanoid!().as_bytes())?;
    let user = User {
        username: user.username,
        password: hash.as_str().into(),
    };

    db.root::<User>()?.insert(&key, &user)?;
    db.root::<Username>()?
        .traverse::<User>()?
        .insert(&user.username, &key)?;

    let query = query.as_ref().map(|x| x.as_str()).unwrap_or_default();

    Ok(Redirect::to(query))
}
