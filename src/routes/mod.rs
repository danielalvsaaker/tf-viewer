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
    pub user_index: Url,
    pub activity_index: Url,
    pub gear_index: Url,
    pub gear_add: Url,
    pub upload: Url,
    pub login: Url,
    pub register: Url,
}

impl UrlFor {
    pub fn new(user: &Identity, req: HttpRequest) -> Result<Self, UrlGenerationError> {
        Ok(UrlFor {
            _static: req.url_for_static("static")?,
            index: req.url_for_static("index")?,
            user: req.url_for("user", &[&user.identity().unwrap_or("None".to_string())])?,
            user_index: req.url_for_static("user_index")?,
            activity_index: req.url_for(
                "activity_index",
                &[&user.identity().unwrap_or("None".to_string())],
            )?,
            gear_index: req.url_for(
                "gear_index",
                &[&user.identity().unwrap_or("None".to_string())],
            )?,
            gear_add: req.url_for(
                "gear_add",
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
