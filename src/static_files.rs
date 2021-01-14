use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    #[cfg(not(feature = "embedded_static"))]
    cfg.service(actix_files::Files::new("/static", "static"))
        .service(web::resource("/static").name("static"));

    #[cfg(feature = "embedded_static")]
    cfg.service(web::resource("/static/css/{file}").route(web::get().to(embedded::serve_css)))
        .service(web::resource("/static/js/{file}").route(web::get().to(embedded::serve_js)))
        .service(
            web::resource("/static/img/favicon.png").route(web::get().to(embedded::serve_favicon)),
        )
        .service(actix_files::Files::new(
            "/static/img/activity",
            "static/img/activity",
        ))
        .service(web::resource("/static").name("static"));
}

#[cfg(feature = "embedded_static")]
mod embedded {
    use {
        crate::error::{Error, ErrorKind, Result},
        actix_web::{http, web, HttpRequest, HttpResponse},
        lazy_static::lazy_static,
    };

    lazy_static! {
        // Current time as of when the server starts. Passed as a header value
        // to tell the browser to cache static content.

        static ref MODIFIED: http::header::HttpDate =
            http::header::HttpDate::from(std::time::SystemTime::now());
    }

    pub async fn serve_css(file: web::Path<String>, req: HttpRequest) -> Result<HttpResponse> {
        static DATATABLES_CSS: &[u8] = include_bytes!("../static/css/datatables.min.css");
        static DROPZONE_CSS: &[u8] = include_bytes!("../static/css/dropzone.min.css");
        static LEAFLET_CSS: &[u8] = include_bytes!("../static/css/leaflet.css");
        static SPECTRE_CSS: &[u8] = include_bytes!("../static/css/spectre.min.css");
        static SPECTRE_ICONS_CSS: &[u8] = include_bytes!("../static/css/spectre-icons.min.css");
        static STYLESHEET_CSS: &[u8] = include_bytes!("../static/css/stylesheet.css");

        let body = match file.into_inner().as_str() {
            "datatables.min.css" => Ok(DATATABLES_CSS),
            "dropzone.min.css" => Ok(DROPZONE_CSS),
            "leaflet.css" => Ok(LEAFLET_CSS),
            "spectre.min.css" => Ok(SPECTRE_CSS),
            "spectre-icons.min.css" => Ok(SPECTRE_ICONS_CSS),
            "stylesheet.css" => Ok(STYLESHEET_CSS),
            _ => Err(Error::BadRequest(ErrorKind::NotFound, "File not found")),
        };

        // Because the files are embedded at compile time, the files are guaranteed
        // to not be modified. Hence the value of IF_MODIFIED_SINCE does not need to be matched,
        // only the existence of it tells that the browser has cached the file.

        Ok(
            if req.headers().contains_key(http::header::IF_MODIFIED_SINCE) {
                HttpResponse::NotModified()
            } else {
                HttpResponse::Ok()
            }
            .header(
                http::header::LAST_MODIFIED,
                http::header::LastModified(*MODIFIED),
            )
            .content_type("text/css")
            .body(body?),
        )
    }

    pub async fn serve_js(file: web::Path<String>, req: HttpRequest) -> Result<HttpResponse> {
        static DATATABLES_JS: &[u8] = include_bytes!("../static/js/datatables.min.js");
        static DROPZONE_JS: &[u8] = include_bytes!("../static/js/dropzone.min.js");
        static LEAFLET_JS: &[u8] = include_bytes!("../static/js/leaflet.js");
        static PLOTLY_JS: &[u8] = include_bytes!("../static/js/plotly-basic.min.js");

        let body = match file.into_inner().as_str() {
            "datatables.min.js" => Ok(DATATABLES_JS),
            "dropzone.min.js" => Ok(DROPZONE_JS),
            "leaflet.js" => Ok(LEAFLET_JS),
            "plotly-basic.min.js" => Ok(PLOTLY_JS),
            _ => Err(Error::BadRequest(ErrorKind::NotFound, "File not found")),
        };

        Ok(
            if req.headers().contains_key(http::header::IF_MODIFIED_SINCE) {
                HttpResponse::NotModified()
            } else {
                HttpResponse::Ok()
            }
            .header(
                http::header::LAST_MODIFIED,
                http::header::LastModified(*MODIFIED),
            )
            .content_type("application/javascript")
            .body(body?),
        )
    }

    pub async fn serve_favicon(req: HttpRequest) -> HttpResponse {
        static FAVICON: &[u8] = include_bytes!("../static/img/favicon.png");

        if req.headers().contains_key(http::header::IF_MODIFIED_SINCE) {
            HttpResponse::NotModified()
        } else {
            HttpResponse::Ok()
        }
        .header(
            http::header::LAST_MODIFIED,
            http::header::LastModified(*MODIFIED),
        )
        .content_type("image/png")
        .body(FAVICON)
    }
}
