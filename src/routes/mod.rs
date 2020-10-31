use actix_web::HttpRequest;
use actix_identity::Identity;
use url::Url;

pub mod authentication;
pub mod index;
pub mod upload;
pub mod user;
pub mod activity;
pub mod gear;
pub mod utils;
pub mod api;
pub mod error;

         
pub struct UrlFor {
    pub _static: Url,
    pub index: Url,
    pub user: Url,
    pub userindex: Url,
    pub activityindex: Url,
    pub gearindex: Url,
    pub upload: Url,
    pub login: Url,
    pub register: Url,
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
    pub url: Url,
}

impl UrlActivity {
    pub fn new(user: &str, activity: &str, req: &HttpRequest) -> Self {
        UrlActivity {
            url: req.url_for("activity", &[user, activity]).unwrap(),
        }
    }
}

mod date_format {
    use chrono::{DateTime, Local};
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
        let s = self.as_secs();

        let (h, s) = (s / 3600, s % 3600);
        let (m, s) = (s / 60, s % 60);

        format!("{:02}:{:02}:{:02}", h, m, s)
    }
}
