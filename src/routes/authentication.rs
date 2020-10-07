use actix_web::{get, Responder};
use actix_identity::Identity;


#[get("/login")]
pub async fn login(id: Identity) -> impl Responder {
    id.remember("test".to_owned());
    "Logged in"
}

#[get("/logout")]
pub async fn logout(id: Identity) -> impl Responder {
    id.forget();
    "Logged out"
}
