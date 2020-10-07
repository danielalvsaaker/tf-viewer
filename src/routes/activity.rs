use actix_web::{get, Responder, web, HttpRequest};
use actix_identity::Identity;
use askama_actix::{Template, TemplateIntoResponse};
use super::UrlFor;

#[derive(Template)]
#[template(path = "activity/activity.html")]
struct ActivityTemplate<'a> {
    url: UrlFor,
    id: Identity,
    user: &'a str,
    activity: &'a str,
    title: &'a str,
}

pub async fn activity(
    req: HttpRequest,
    id: Identity,
    web::Path((user, activity)): web::Path<(String, String)>
    ) -> impl Responder {

    ActivityTemplate {
        url: UrlFor::new(&id, req),
        id: id,
        user: &user,
        activity: &activity,
        title: &activity,
    }.into_response()
}


#[derive(Template)]
#[template(path = "activity/activityindex.html")]
struct ActivityIndexTemplate<'a> {
    url: UrlFor,
    id: Identity,
    user: &'a str,
    title: &'a str,
}

pub async fn activityindex(
    req: HttpRequest,
    id: Identity,
    user: web::Path<String>
    ) -> impl Responder {

    ActivityIndexTemplate {
        url: UrlFor::new(&id, req),
        id: id,
        user: &user,
        title: "Activities",
    }.into_response()
}
