use axum::{
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_sessions::extractors::WritableSession;

pub fn routes() -> Router {
    Router::new().route("/", get(get_signout))
}

async fn get_signout(mut session: WritableSession) -> impl IntoResponse {
    session.destroy();

    Redirect::to("/oauth/signin")
}
