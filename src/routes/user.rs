use super::{utils, PasswordEnum, UrlFor};
use crate::models::UserTotals;
use actix_identity::Identity;
use actix_multipart::Multipart;
use actix_web::{http, web, Either, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use futures::{StreamExt, TryStreamExt};
use serde::Deserialize;
use std::io::Write;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").name("user_index").to(user_index))
        .service(web::resource("/{username}").name("user").to(user))
        .service(
            web::resource("/{username}/settings")
                .name("user_settings")
                .wrap(crate::middleware::Restricted)
                .route(web::get().to(user_settings))
                .route(web::post().to(user_settings_post)),
        )
        .service(
            web::resource("/{username}/settings/avatar")
                .name("user_avatar")
                .wrap(crate::middleware::Restricted)
                .route(web::get().to(user_avatar))
                .route(web::post().to(user_avatar_post)),
        );
}

#[derive(Template)]
#[template(path = "user/user.html")]
struct UserTemplate<'a> {
    url: UrlFor,
    id: Identity,
    user_totals: &'a UserTotals,
    username: &'a str,
    title: &'a str,
}

async fn user(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    username: web::Path<String>,
) -> impl Responder {
    data.users.exists(&username)?;

    let user_totals = data.activities.user_totals(&username)?;

    UserTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        user_totals: &user_totals,
        username: &username,
        title: &username,
    }
    .into_response()
}

#[derive(Template)]
#[template(path = "user/index.html")]
struct UserIndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    users: Vec<String>,
    title: &'a str,
}

async fn user_index(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
) -> impl Responder {
    let users: Vec<String> = data.users.iter_id()?.collect();

    UserIndexTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        title: "Users",
        users,
    }
    .into_response()
}

#[derive(Deserialize, Debug)]
struct HeartrateForm {
    heartrate_rest: u8,
    heartrate_max: u8,
}

#[derive(Deserialize, Debug)]
pub struct PasswordForm {
    pub current_password: String,
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Template)]
#[template(path = "user/settings.html")]
struct UserSettingsTemplate<'a> {
    url: UrlFor,
    id: Identity,
    heartrate: &'a Option<(u8, u8)>,
    message: &'a Option<crate::error::Error>,
    title: &'a str,
}

async fn user_settings(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    username: web::Path<String>,
) -> impl Responder {
    let heartrate = data.users.get_heartrate(&username)?;

    UserSettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        heartrate: &heartrate,
        message: &None,
        title: "Settings",
    }
    .into_response()
}

async fn user_settings_post(
    req: HttpRequest,
    id: Identity,
    username: web::Path<String>,
    data: web::Data<crate::Database>,
    form: Either<web::Form<HeartrateForm>, web::Form<PasswordForm>>,
) -> impl Responder {
    let form_result = match form {
        Either::A(x) => {
            data.users
                .set_heartrate(&username, (x.heartrate_rest, x.heartrate_max))?;
            Ok(())
        }
        Either::B(x) => {
            let check_result = utils::validate_form(&PasswordEnum::Settings(&username, &x), &data);

            if check_result.is_ok() {
                data.users.insert(&username, &x.confirm_password)?;
            }

            check_result
        }
    };

    let url: UrlFor = UrlFor::new(&id, &req)?;

    if form_result.is_ok() {
        Ok(HttpResponse::Found()
            .header(http::header::LOCATION, url.user.as_str())
            .finish()
            .into_body())
    } else {
        let heartrate = data.users.get_heartrate(&username)?;
        UserSettingsTemplate {
            url,
            id,
            heartrate: &heartrate,
            message: &form_result.err(),
            title: "Settings",
        }
        .into_response()
    }
}

#[derive(Template)]
#[template(path = "user/avatar.html")]
struct UserAvatarTemplate<'a> {
    url: UrlFor,
    id: Identity,
    title: &'a str,
}

async fn user_avatar(req: HttpRequest, id: Identity) -> impl Responder {
    UserAvatarTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        title: "Upload profile picture",
    }
    .into_response()
}

async fn user_avatar_post(mut payload: Multipart, username: web::Path<String>) -> impl Responder {
    while let Ok(Some(mut field)) = payload.try_next().await {
        // Should be improved, jpg-files are saved as png. Works, but not preferable
        let filepath = format!("static/img/user/{}.png", &username);
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            f = web::block(move || f.write_all(&chunk).map(|_| f))
                .await
                .unwrap();
        }
    }

    HttpResponse::Ok().finish().into_body()
}
