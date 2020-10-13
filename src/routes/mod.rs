use actix_web::HttpRequest;
use actix_identity::Identity;
use askama_actix::Template;

pub mod authentication;
pub mod index;
pub mod upload;
pub mod user;
pub mod activity;
pub mod gear;
pub mod utils;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    url: UrlFor,
    id: Identity,
    title: &'a str,
}

pub struct UrlFor {
    pub _static: url::Url,
    pub index: url::Url,
    pub user: url::Url,
    pub userindex: url::Url,
    pub activityindex: url::Url,
    pub gearindex: url::Url,
    pub upload: url::Url,
    pub login: url::Url,
    pub register: url::Url,
}

impl UrlFor {
    pub fn new(user: &Identity, req: HttpRequest) -> Self {
        UrlFor {
            _static: req.url_for_static("static").unwrap(),
            index: req.url_for_static("index").unwrap(),
            user: req.url_for("user", &[&user.identity().unwrap_or("None".to_string())]).unwrap(),
            userindex: req.url_for_static("userindex").unwrap(),
            activityindex: req.url_for("activityindex", &[&user.identity().unwrap_or("None".to_string())]).unwrap(),
            gearindex: req.url_for("gearindex", &[&user.identity().unwrap_or("None".to_string())]).unwrap(),
            upload: req.url_for_static("upload").unwrap(),
            login: req.url_for_static("login").unwrap(),
            register: req.url_for_static("register").unwrap(),
        }
    }
}

pub struct UrlActivity {
    pub url: url::Url,
}

impl UrlActivity {
    pub fn new(user: &str, activity: &str, req: HttpRequest) -> Self {
        UrlActivity {
            url: req.url_for("activity", &[user, activity]).unwrap(),
        }
    }
}

mod date_format {
    use chrono::{DateTime, Local, TimeZone};
    use serde::{self, Serializer};

    pub fn serialize<S>(
        date: &DateTime<Local>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format("%d.%m.%Y %H:%M"));
        serializer.serialize_str(&s)
    }
}

pub trait FormatDuration {
    fn to_string(&self) -> String;
}

impl FormatDuration for std::time::Duration {
    fn to_string(&self) -> String {
        let mut s = self.as_secs();
        let ms = self.subsec_millis();

        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);

        format!("{:02}:{:02}:{:02}", h, m, s)
    }
}
