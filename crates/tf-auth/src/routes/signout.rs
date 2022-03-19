use axum::{
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};

pub fn routes() -> Router {
    Router::new().route("/", get(get_signout))
}

async fn get_signout(mut session: crate::session::Session) -> impl IntoResponse {
    session.forget().await;

    Redirect::to("/oauth/signin".parse().unwrap())
}
