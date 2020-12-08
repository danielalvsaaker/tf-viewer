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
    user: crate::User,
    title: &'a str,
}

pub async fn user(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
    username: web::Path<String>,
) -> impl Responder {

    if !data.as_ref().users.exists(&username).unwrap() {
        return ErrorTemplate::not_found(req, id).await;
    }

    let user = data.as_ref().users.get(&username).unwrap();

    UserTemplate {
        url: UrlFor::new(&id, req)?,
        id,
        username: &username,
        user,
        title: &username,
    }.into_response()
}

#[derive(Template)]
#[template(path = "user/userindex.html")]
struct UserIndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    users: Vec<String>,
    title: &'a str,
}

pub async fn userindex(
    req: HttpRequest,
    id: Identity,
    data: web::Data<crate::Database>,
) -> impl Responder {
    let users: Vec<String> = data.as_ref().users.iter_id().unwrap().collect();

    UserIndexTemplate {
        url: UrlFor::new(&id, req)?,
        id,
        title: "Users",
        users,
    }
    .into_response()
}

pub async fn userindex_post(
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
        recordsTotal: amount,
        recordsFiltered: amount,
        data: results,
    })
}
