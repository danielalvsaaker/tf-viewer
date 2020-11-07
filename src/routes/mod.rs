use actix_identity::Identity;
use actix_web::{error::UrlGenerationError, HttpRequest};
use url::Url;

pub mod activity;
pub mod api;
pub mod authentication;
pub mod error;
pub mod gear;
pub mod index;
pub mod upload;
pub mod user;
pub mod utils;

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
    pub fn new(user: &Identity, req: HttpRequest) -> Result<Self, UrlGenerationError> {
        Ok(UrlFor {
            _static: req.url_for_static("static")?,
            index: req.url_for_static("index")?,
            user: req
                .url_for("user", &[&user.identity().unwrap_or("None".to_string())])?,
            userindex: req.url_for_static("userindex")?,
            activityindex: req.url_for(
                "activityindex",
                &[&user.identity().unwrap_or("None".to_string())],
            )?,
            gearindex: req.url_for(
                "gearindex",
                &[&user.identity().unwrap_or("None".to_string())],
            )?,
            upload: req.url_for_static("upload")?,
            login: req.url_for_static("login")?,
            register: req.url_for_static("register")?,
        })
    }
}

pub struct UrlActivity {
    pub url: Url,
}

impl UrlActivity {
    pub fn new(user: &str, activity: &str, req: &HttpRequest) -> Result<Self, UrlGenerationError> {
        Ok(UrlActivity {
            url: req.url_for("activity", &[user, activity])?,
        })
    }
}

mod date_format {
    use chrono::{DateTime, Local};
    use serde::{self, Serializer};

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
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
