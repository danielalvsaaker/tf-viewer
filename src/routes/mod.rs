use actix_web::HttpRequest;
use actix_identity::Identity;

pub mod authentication;
pub mod index;
pub mod user;
pub mod activity;
pub mod gear;

pub struct UrlFor {
    pub index: url::Url,
    pub user: url::Url,
    pub userindex: url::Url,
    pub activityindex: url::Url,
    pub gearindex: url::Url,
    pub upload: url::Url,
    pub login: url::Url,
}

impl UrlFor {
    pub fn new(user: &Identity, req: HttpRequest) -> Self {
        UrlFor {
            index: req.url_for("index", &[""]).unwrap(),
            user: req.url_for("user", &[&user.identity().unwrap()]).unwrap(),
            userindex: req.url_for("userindex", &[""]).unwrap(),
            activityindex: req.url_for("activityindex", &[&user.identity().unwrap()]).unwrap(),
            gearindex: req.url_for("gearindex", &[&user.identity().unwrap()]).unwrap(),
            upload: req.url_for("upload", &[""]).unwrap(),
            login: req.url_for("login", &[""]).unwrap(),
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


