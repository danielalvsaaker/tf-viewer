use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};

pub async fn get_signout(id: Identity) -> impl Responder {
    id.forget();

    HttpResponse::Found()
        .append_header(("Location", "signin"))
        .finish()
}
