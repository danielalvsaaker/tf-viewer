use crate::error::Result;
use actix_web::{http, web, HttpRequest, HttpResponse, Responder};
use std::ops::Deref;
use tf_database::{query::UserQuery, Database};
use tf_macro::protect;
use tf_models::backend::User;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/{user_id}")
            .name("user")
            .route(web::get().to(get_user))
            .route(web::put().to(put_user)),
    );
}

#[protect]
async fn get_user(
    db: web::Data<Database>,
    query: web::Path<UserQuery<'_>>,
) -> Result<impl Responder> {
    let user = db.user.get_user(&query)?;

    Ok(web::Json(user))
}

#[protect]
pub async fn post_user(
    db: web::Data<Database>,
    user: web::Json<User>,
    req: HttpRequest,
) -> Result<impl Responder> {
    let id = nanoid::nanoid!(10);
    let query = UserQuery::from(id.as_str());

    db.user.insert_user(&query, user.deref())?;

    let url = req.url_for("user", &[query.user_id]).unwrap();

    Ok(HttpResponse::Created()
        .insert_header((http::header::LOCATION, url.to_string()))
        .finish())
}

#[protect]
async fn put_user(
    db: web::Data<Database>,
    query: web::Path<UserQuery<'_>>,
    user: web::Json<User>,
) -> Result<impl Responder> {
    Ok(match db.user.insert_user(&query, user.deref())? {
        Some(_) => HttpResponse::NoContent(),
        None => HttpResponse::Created(),
    }
    .finish())
}
