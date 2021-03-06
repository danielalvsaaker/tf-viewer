use super::{utils, PasswordEnum, UrlFor};
use crate::models::{DisplayUnit, Unit, UserTotals};
use actix_identity::Identity;
use actix_web::{http, web, Either, HttpRequest, HttpResponse, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use serde::Deserialize;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").name("user_index").to(user_index))
        .service(web::resource("/{username}").name("user").to(user))
        .service(
            web::resource("/{username}/settings")
                .name("user_settings")
                .wrap(crate::middleware::Restricted)
                .route(web::get().to(user_settings))
                .route(web::post().to(user_settings_post)),
        );
}

#[derive(Template)]
#[template(path = "user/user.html")]
struct UserTemplate<'a> {
    url: UrlFor,
    id: Identity,
    unit: &'a Unit,
    user_totals: &'a UserTotals,
    username: &'a str,
    title: &'a str,
}

async fn user(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    username: web::Path<String>,
    unit: web::Data<Unit>,
) -> impl Responder {
    data.users.exists(&username)?;

    let user_totals = data.activities.user_totals(&username)?;

    UserTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        unit: &unit,
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
    users: &'a [String],
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
        users: &users,
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
