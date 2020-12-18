use super::{
    api::{DataRequest, DataResponse, UserData},
    error::ErrorTemplate,
    UrlFor,
};
use actix_identity::Identity;
use actix_web::{web, HttpRequest, Responder};
use askama_actix::{Template, TemplateIntoResponse};

#[derive(Template)]
#[template(path = "user/user.html")]
struct UserTemplate<'a> {
    url: UrlFor,
    id: Identity,
    username: &'a str,
    user: &'a crate::User,
    title: &'a str,
}

pub async fn user(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    username: web::Path<String>,
) -> impl Responder {
    let user = {
        let username = username.clone();
        web::block(move || data.as_ref().users.get(&username))
    }
    .await?;

    UserTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        username: &username,
        user: &user,
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

pub async fn user_index(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
) -> impl Responder {
    let users: Vec<String> = web::block(move || data.as_ref().users.iter_id())
        .await?
        .collect();

    UserIndexTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        title: "Users",
        users,
    }
    .into_response()
}

pub async fn user_index_post(
    request: web::Json<DataRequest>,
    data: web::Data<crate::Database>,
) -> impl Responder {
    let ids = data.as_ref().users.iter_id().unwrap();

    let mut users: Vec<UserData> = ids.map(|x| UserData { name: x }).collect();

    let amount = users.len();

    if request.dir.as_str() == "asc" {
        users.reverse();
    }

    let results: Vec<UserData> = users
        .into_iter()
        .skip(request.start)
        .take(request.length)
        .collect();

    web::Json(DataResponse {
        draw: request.draw,
        records_total: amount,
        records_filtered: amount,
        data: results,
    })
}

#[derive(Template)]
#[template(path = "user/settings.html")]
struct UserSettingsTemplate<'a> {
    url: UrlFor,
    id: Identity,
    title: &'a str,
}

pub async fn user_settings(req: HttpRequest, id: Identity) -> impl Responder {
    UserSettingsTemplate {
        url: UrlFor::new(&id, &req)?,
        id,
        title: "Settings",
    }
    .into_response()
}
